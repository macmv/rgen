use rgen_base::{BlockState, Chunk, ChunkPos, Pos};
use rgen_placer::grid::PointGrid;

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

  roads: Vec<Road>,
}

struct Road {
  start: Pos,
  end:   Pos,
}

impl<'a> Village<'a> {
  pub fn new(generator: &'a VillageGenerator, seed: u64, origin: Pos) -> Self {
    let roads = vec![
      Road { start: origin, end: origin + Pos::new(16, 0, 0) },
      Road { start: origin, end: origin + Pos::new(0, 0, 16) },
    ];

    Village { generator, roads }
  }

  pub fn generate(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    for road in &self.roads {
      for x in road.start.x..=road.end.x {
        for z in road.start.z..=road.end.z {
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
    }
  }
}
