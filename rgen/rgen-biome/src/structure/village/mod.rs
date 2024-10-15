use rgen_base::{block, BlockState, Chunk, ChunkPos, ChunkRelPos, Pos};
use rgen_llama::Structure;
use rgen_placer::{grid::PointGrid, Random, Rng};

mod building;
mod math;
mod road;

use building::Building;
use math::Direction;
use rgen_world::BlockInfoSupplier;
use road::Road;

pub struct VillageGenerator {
  seed: u64,
  grid: PointGrid,

  buildings: Vec<Structure>,

  road_block: BlockState,
}

const VILLAGE_RADIUS: i32 = 96;

impl VillageGenerator {
  pub fn new(seed: u64) -> Self {
    let grid = PointGrid::new();

    VillageGenerator {
      seed,
      grid,
      road_block: block![log],
      buildings: vec![
        rgen_llama::parse(include_str!("building/house_1.ll")),
        rgen_llama::parse(include_str!("building/house_2.ll")),
      ],
    }
  }

  pub fn generate(&self, info: &BlockInfoSupplier, chunk: &mut Chunk, chunk_pos: ChunkPos) {
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
      village.generate(info, chunk, chunk_pos);
    }
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
      for x in road.min().x..=road.max().x {
        for z in road.min().z..=road.max().z {
          let pos = Pos::new(x, 100, z);

          for dx in -1..=1 {
            for dz in -1..=1 {
              let pos = pos + Pos::new(dx, 0, dz);
              if !pos.in_chunk(chunk_pos) {
                continue;
              }

              let rel = pos.chunk_rel();

              let y = highest_block(chunk, rel).y();
              chunk.set(rel.with_y(y), info.encode(self.generator.road_block));
            }
          }
        }
      }

      for building in &self.buildings {
        let structure = &self.generator.buildings[building.building_id as usize];

        // This is the axis of rotation for the building.
        let front_center = Pos::new(structure.width() as i32 / 2, 0, 0);
        for rel_pos in structure.blocks() {
          let block = structure.get(rel_pos);
          if block != BlockState::AIR {
            // Rotate `rel_pos` about the `front_center`.
            let rotated_x = rel_pos.x - front_center.x;
            let rotated_z = rel_pos.z - front_center.z;
            let (rotated_x, rotated_z) = match building.forward {
              Direction::North => (rotated_x, rotated_z),
              Direction::East => (-rotated_z, rotated_x),
              Direction::South => (-rotated_x, -rotated_z),
              Direction::West => (rotated_z, -rotated_x),
            };
            let pos = building.pos + Pos::new(rotated_x, rel_pos.y, rotated_z);

            if pos.in_chunk(chunk_pos) {
              // FIXME
              chunk.set(pos.chunk_rel(), info.encode(block));
            }
          }
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

          let pos = Pos::new(x, 100, z) - forward.dir() * 2;

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
