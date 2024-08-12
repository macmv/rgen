use rgen_base::{BlockState, Chunk, ChunkPos, Pos};
use rgen_llama::Structure;
use rgen_placer::{grid::PointGrid, Random, Rng};

use crate::biome::IdContext;

pub struct VillageGenerator {
  seed: u64,
  grid: PointGrid,

  road_block: BlockState,
}

const VILLAGE_RADIUS: i32 = 96;

impl VillageGenerator {
  pub fn new(ctx: &IdContext, seed: u64) -> Self {
    let grid = PointGrid::new();
    VillageGenerator { seed, grid, road_block: ctx.blocks.log.default_state }
  }

  pub fn generate(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
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
      village.generate(chunk, chunk_pos);
    }
  }
}

struct Village<'a> {
  generator: &'a VillageGenerator,

  roads:     Vec<Road>,
  buildings: Vec<Building>,

  origin: Pos,
}

struct Road {
  start: Pos,
  end:   Pos,
}

struct Building {
  pos: Pos,
}

impl<'a> Village<'a> {
  pub fn new(generator: &'a VillageGenerator, seed: u64, origin: Pos) -> Self {
    let mut roads = vec![];
    let mut rng = Rng::new(seed);
    recursive_road(&mut roads, &mut rng, origin, origin, 0);

    let mut buildings = vec![];
    place_buildings(&mut buildings, &mut rng, &roads);

    Village { generator, roads, buildings, origin }
  }

  pub fn generate(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    for road in &self.roads {
      for x in road.start.x.min(road.end.x)..=road.start.x.max(road.end.x) {
        for z in road.start.z.min(road.end.z)..=road.start.z.max(road.end.z) {
          let pos = Pos::new(x, 100, z);

          for dx in -1..=1 {
            for dz in -1..=1 {
              let pos = pos + Pos::new(dx, 0, dz);
              if !pos.in_chunk(chunk_pos) {
                continue;
              }

              chunk.set(pos.chunk_rel(), self.generator.road_block);
            }
          }
        }
      }

      for building in &self.buildings {
        for y in 0..3 {
          let rel_pos = Pos::new(0, y as i32, 0);
          let pos = building.pos + rel_pos;

          if pos.in_chunk(chunk_pos) {
            chunk.set(pos.chunk_rel(), self.generator.road_block);
          }
        }
      }

      if road.start.in_chunk(chunk_pos) {
        chunk.set(
          road.start.chunk_rel().with_y(100),
          BlockState { block: self.generator.road_block.block, state: 1 },
        );
      }
      if road.end.in_chunk(chunk_pos) {
        chunk.set(
          road.end.chunk_rel().with_y(100),
          BlockState { block: self.generator.road_block.block, state: 2 },
        );
      }
    }

    if self.origin.in_chunk(chunk_pos) {
      chunk.set(self.origin.chunk_rel().with_y(101), self.generator.road_block);
    }
  }
}

fn recursive_road(roads: &mut Vec<Road>, rng: &mut Rng, origin: Pos, pos: Pos, depth: u32) {
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
    if roads.iter().any(|road| {
      let min_x = road.start.x.min(road.end.x) - 4;
      let max_x = road.start.x.max(road.end.x) + 4;
      let min_z = road.start.z.min(road.end.z) - 4;
      let max_z = road.start.z.max(road.end.z) + 4;

      new_pos.x >= min_x && new_pos.x <= max_x && new_pos.z >= min_z && new_pos.z <= max_z
    }) {
      continue;
    }

    let road = Road { start: pos, end: new_pos };
    roads.push(road);

    recursive_road(roads, rng, origin, new_pos, depth + 1);
  }
}

fn place_buildings(buildings: &mut Vec<Building>, rng: &mut Rng, roads: &[Road]) {
  fn place_buildings_along(buildings: &mut Vec<Building>, _rng: &mut Rng, road: &Road) {
    let mut i = 0;

    let off_axis = if road.start.x != road.end.x { (0, 1) } else { (1, 0) };

    for x in road.start.x.min(road.end.x)..=road.start.x.max(road.end.x) {
      for z in road.start.z.min(road.end.z)..=road.start.z.max(road.end.z) {
        for side in [-1, 1] {
          i += 1;
          if i % 9 != 0 {
            continue;
          }

          let pos = Pos::new(x + off_axis.0 * side * 4, 100, z + off_axis.1 * side * 4);

          buildings.push(Building { pos })
        }
      }
    }
  }

  for road in roads {
    place_buildings_along(buildings, rng, road);
  }
}
