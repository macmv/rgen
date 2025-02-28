use math::Direction;
use rgen_base::{BlockFilter, BlockState, Chunk, ChunkPos, ChunkRelPos, Pos, block, block_kind};
use rgen_llama::Structure;
use rgen_placer::{Random, Rng, grid::PointGrid};
use rgen_world::PartialWorld;

mod building;
mod math;
mod road;

use building::Building;
use rgen_world::BlockInfoSupplier;
use road::Road;

pub struct VillageGenerator {
  seed: u64,
  grid: PointGrid,

  replaceable: BlockFilter,
  road_block:  BlockState,

  buildings: Vec<Structure>,
}

const VILLAGE_RADIUS: i32 = 96;

impl VillageGenerator {
  pub fn new(seed: u64) -> Self {
    let grid = PointGrid::new();

    VillageGenerator {
      seed,
      grid,
      // FIXME: Needs so much replacing.
      replaceable: [
        block![air],
        block![leaves],
        block![rgen:leaves],
        block![rgen:leaves2],
        block![rgen:leaves3],
        block![double_plant],
        block![tallgrass],
      ]
      .into(),
      road_block: block![grass_path],
      buildings: vec![
        rgen_llama::parse(include_str!("building/house_1.ll")),
        rgen_llama::parse(include_str!("building/house_2.ll")),
      ],
    }
  }

  fn call_villages(&self, chunk_pos: ChunkPos, mut f: impl FnMut(Village)) {
    // FIXME: A lot of this is copied from noodle caves, need to dedupe.

    let scale = 256.0;

    let min_pos = chunk_pos.min_block_pos();
    let cave_min_x = ((min_pos.x - VILLAGE_RADIUS) as f64) / scale;
    let cave_min_z = ((min_pos.z - VILLAGE_RADIUS) as f64) / scale;
    let cave_max_x = ((min_pos.x + 16 + VILLAGE_RADIUS) as f64) / scale;
    let cave_max_z = ((min_pos.z + 16 + VILLAGE_RADIUS) as f64) / scale;

    let points =
      self.grid.points_in_area(self.seed, cave_min_x, cave_min_z, cave_max_x, cave_max_z);
    for point in points {
      let pos = Pos::new((point.0 * scale) as i32, 0, (point.1 * scale) as i32);

      // A seed unique to this village.
      let village_seed = self.seed ^ ((pos.x as u64) << 8) ^ ((pos.y as u64) << 16);

      let village = Village::new(self, village_seed, pos);
      f(village)
    }
  }

  pub fn generate(&self, info: &BlockInfoSupplier, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    self.call_villages(chunk_pos, |village| {
      village.generate(info, chunk, chunk_pos);
    });
  }

  pub fn decorate(&self, world: &mut PartialWorld, chunk_pos: ChunkPos) {
    self.call_villages(chunk_pos, |village| {
      village.decorate(world, chunk_pos);
    });
  }
}

struct Village<'a> {
  generator: &'a VillageGenerator,

  roads:     Vec<Road>,
  buildings: Vec<Building>,
}

impl<'a> Village<'a> {
  pub fn new(generator: &'a VillageGenerator, seed: u64, origin: Pos) -> Self {
    let mut village = Village { generator, roads: vec![], buildings: vec![] };

    let mut rng = Rng::new(seed);
    village.recursive_road(&mut rng, origin, 0);

    village.place_buildings(&mut rng);

    village
  }

  pub fn generate(&self, info: &BlockInfoSupplier, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    for road in &self.roads {
      for x in road.min().x - 1..=road.max().x + 1 {
        for z in road.min().z - 1..=road.max().z + 1 {
          let pos = Pos::new(x, 100, z);

          if !pos.in_chunk(chunk_pos) {
            continue;
          }

          let rel = pos.chunk_rel();

          let y = highest_block(chunk, rel).y();

          let replacing = info.decode(chunk.get(rel.with_y(y)));
          // Intersections get placed multiple times, so we need to check for planks here
          // too.
          let placing = if replacing == block![water] || replacing == block![planks] {
            block![planks]
          } else {
            self.generator.road_block
          };
          chunk.set(rel.with_y(y), info.encode(placing));
        }
      }
    }
  }

  pub fn decorate(&self, world: &mut PartialWorld, chunk_pos: ChunkPos) {
    for building in &self.buildings {
      let structure = &self.generator.buildings[building.building_id as usize];

      // If the building is in this chunk, we place it. Because this is part of the
      // decoration pass, we can modify blocks in neighboring chunks. So we'll
      // place the entire building at once, and we can consistently find ground
      // level at the same time.
      if !building.pos.in_chunk(chunk_pos) {
        continue;
      }

      // The Y position of the base of the building.
      let mut max_height = 0;
      let mut min_height = 255;
      for x in 0..structure.width() {
        for z in 0..structure.depth() {
          let rel_pos = Pos::new(x as i32, 0, z as i32);
          let pos = building.transform_to_world(structure, rel_pos);

          for y in (0..=255).rev() {
            if !self.generator.replaceable.contains(world.get(pos.with_y(y))) {
              if y < min_height {
                min_height = y;
              }
              if y > max_height {
                max_height = y;
              }
              break;
            }
          }
        }
      }

      // If the ground is too steep, don't place the building.
      if max_height - min_height > 5 {
        continue;
      }

      for rel_pos in structure.blocks() {
        let block = structure.get(rel_pos);
        // NB: The building is placed at `max_height` to set it into the surface by 1
        // block.
        let pos = building.transform_to_world(structure, rel_pos + Pos::new(0, max_height, 0));

        if block != block![air] {
          world.set(pos, rotate_block(block, building.forward));
        }
      }

      // Fill in some dirt below the foundation.
      for x in 0..structure.width() {
        for z in 0..structure.depth() {
          let rel_pos = Pos::new(x as i32, 0, z as i32);
          let pos = building.transform_to_world(structure, rel_pos);

          let mut y = max_height - 1;
          while self.generator.replaceable.contains(world.get(pos.with_y(y))) {
            world.set(pos.with_y(y), block![dirt]);
            y -= 1;
          }

          // Set the layer below to dirt (this is usually grass, which we want to
          // replace).
          world.set(pos.with_y(y), block![dirt]);
        }
      }
    }
  }
}

// Village generation
impl<'a> Village<'a> {
  fn recursive_road(&mut self, rng: &mut Rng, pos: Pos, depth: u32) {
    if depth > 3 {
      return;
    }

    let mut dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    rng.shuffle(&mut dirs);

    for (dx, dz) in dirs {
      if rng.rand_inclusive(0, 2) < 1 {
        continue;
      }

      let length = rng.rand_inclusive(8, 32);

      let new_pos = pos + Pos::new(dx * length, 0, dz * length);
      if self.roads.iter().any(|road| {
        let min_x = road.start.x.min(road.end.x) - 4;
        let max_x = road.start.x.max(road.end.x) + 4;
        let min_z = road.start.z.min(road.end.z) - 4;
        let max_z = road.start.z.max(road.end.z) + 4;

        new_pos.x >= min_x && new_pos.x <= max_x && new_pos.z >= min_z && new_pos.z <= max_z
      }) {
        continue;
      }

      let road = Road { start: pos, end: new_pos };
      self.roads.push(road);

      self.recursive_road(rng, new_pos, depth + 1);
    }
  }

  fn place_buildings(&mut self, rng: &mut Rng) {
    for road in self.roads.clone() {
      self.place_buildings_along(rng, &road);
    }
  }

  fn place_buildings_along(&mut self, rng: &mut Rng, road: &Road) {
    let mut i = 0;

    let off_axis = road.axis().orthogonal();

    for x in road.min().x..=road.max().x {
      for z in road.min().z..=road.max().z {
        for side in [true, false] {
          i += 1;
          if i % 9 != 0 {
            continue;
          }

          let building_id = rng.rand_exclusive(0, self.generator.buildings.len() as i32) as u32;
          let building = &self.generator.buildings[building_id as usize];

          let forward = if side { off_axis.positive_dir() } else { off_axis.negative_dir() };

          let pos = Pos::new(x, 0, z) - forward.dir() * 2;

          self.try_place_building(Building {
            pos,
            forward,
            building_id,
            width: building.width(),
            depth: building.depth(),
          });
        }
      }
    }
  }

  fn try_place_building(&mut self, building: Building) {
    if self.can_place_building(&building) {
      self.buildings.push(building);
    }
  }

  fn can_place_building(&self, building: &Building) -> bool {
    for road in &self.roads {
      if building.bounding_box().intersects(&road.bounding_box()) {
        return false;
      }
    }
    for other in &self.buildings {
      if building.bounding_box().intersects(&other.bounding_box()) {
        return false;
      }
    }

    true
  }
}

fn highest_block(chunk: &Chunk, pos: ChunkRelPos) -> ChunkRelPos {
  let mut y = 255;

  // TODO: A better air check?
  while chunk.get(pos.with_y(y)).0 == 0 {
    y -= 1;
  }

  ChunkRelPos::new(pos.x(), y, pos.z())
}

fn rotate_block(block: BlockState, dir: Direction) -> BlockState {
  fn rotate_ccw(block: BlockState) -> BlockState {
    let state = block.state.state().unwrap_or_default();

    let new_state = match block.block {
      // axis=x -> axis=z
      block_kind![log] if state & 0b1100 == 0b0100 => state & 0b0011 | 0b1000,
      // axis=z -> axis=x
      block_kind![log] if state & 0b1100 == 0b1000 => state & 0b0011 | 0b0100,

      _ => return block,
    };

    block.with_data(new_state)
  }

  match dir {
    Direction::North => block,
    Direction::East => rotate_ccw(block),
    Direction::South => rotate_ccw(rotate_ccw(block)),
    Direction::West => rotate_ccw(rotate_ccw(rotate_ccw(block))),
  }
}
