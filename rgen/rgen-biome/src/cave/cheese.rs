use rgen_base::{Block, Chunk, ChunkPos, Pos};
use rgen_placer::noise::{NoiseGenerator3D, OctavedNoise, PerlinNoise};

use crate::biome::IdContext;

/// Cheese caves are the big caverns.
pub struct CheeseCarver {
  cave_map: OctavedNoise<PerlinNoise>,
}

impl CheeseCarver {
  pub fn new(_ctx: &IdContext) -> Self {
    CheeseCarver { cave_map: OctavedNoise { octaves: 4, freq: 1.0 / 128.0, ..Default::default() } }
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
            chunk.set(pos.chunk_rel(), Block::AIR);
          }
        }
      }
    }
  }
}
