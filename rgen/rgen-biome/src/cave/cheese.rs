use rgen_base::{Block, Chunk, ChunkPos, Pos};
use rgen_placer::noise::{NoiseGenerator3D, OctavedNoise, PerlinNoise};

use crate::biome::IdContext;

/// Cheese caves are the big caverns.
pub struct CheeseCarver {
  cave_map: OctavedNoise<PerlinNoise>,

  water: Block,
}

impl CheeseCarver {
  pub fn new(ctx: &IdContext) -> Self {
    CheeseCarver {
      cave_map: OctavedNoise { octaves: 4, freq: 1.0 / 128.0, ..Default::default() },

      water: ctx.blocks.water.block,
    }
  }

  pub fn carve(&self, seed: u64, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    let seed = seed.wrapping_add(200);

    for rel_x in 0..16_u8 {
      for rel_z in 0..16_u8 {
        let pos = chunk_pos.min_block_pos() + Pos::new(rel_x.into(), 0, rel_z.into());

        for y in 0..=255 {
          let pos = pos.with_y(y);
          let noise =
            self.cave_map.generate_3d(pos.x as f64, pos.y as f64 * 4.0, pos.z as f64, seed) * 0.5
              + 0.5;
          if noise < 0.1 {
            let mut near_water = false;
            for offset in [
              Pos::new(0, 0, 0),
              Pos::new(-1, 0, 0),
              Pos::new(1, 0, 0),
              Pos::new(0, 0, -1),
              Pos::new(0, 0, 1),
              Pos::new(0, 1, 0),
              Pos::new(0, -1, 0),
            ] {
              let pos = pos + offset;
              // Chunk borders: we don't care! We can let a bit of floating water exist.
              if !pos.in_chunk(chunk_pos) {
                continue;
              }

              let block = chunk.get(pos.chunk_rel());

              if block == self.water {
                near_water = true;
                break;
              }
            }

            if !near_water {
              chunk.set(pos.chunk_rel(), Block::AIR);
            }
          }
        }
      }
    }
  }
}
