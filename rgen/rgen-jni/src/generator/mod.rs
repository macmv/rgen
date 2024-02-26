use crate::ChunkContext;
use rgen_base::{Blocks, Chunk, ChunkRelPos, Pos};
use rgen_placer::noise::{NoiseGenerator, OctavedNoise, PerlinNoise};

pub struct Generator {
  seed: u64,

  height_map: OctavedNoise<PerlinNoise>,

  biomes: rgen_biome::Biomes,
}

impl Generator {
  pub fn new(blocks: &Blocks, biome_ids: &rgen_base::Biomes, seed: u64) -> Generator {
    Generator {
      seed,
      height_map: OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },

      biomes: rgen_biome::Biomes::new(blocks, biome_ids),
    }
  }

  pub fn generate(&self, ctx: &ChunkContext, chunk: &mut Chunk) {
    for rel_x in 0..16_u8 {
      for rel_z in 0..16_u8 {
        let pos = ctx.chunk_pos.min_block_pos() + Pos::new(rel_x.into(), 0, rel_z.into());

        let height =
          ((self.height_map.generate(pos.x as f64, pos.z as f64, self.seed) + 1.0) * 64.0) as i32;

        for y in 0..height as u8 {
          chunk.set(ChunkRelPos::new(rel_x, y, rel_z), ctx.blocks.stone);
        }
      }
    }

    self.biomes.generate(ctx.blocks, self.seed, ctx.chunk_pos, chunk);

    chunk.set(ChunkRelPos::new(0, 6, 0), ctx.blocks.dirt);
  }

  pub fn generate_biomes(&self, ctx: &ChunkContext, biomes: &mut [u8; 256]) {
    self.biomes.generate_ids(self.seed, ctx.chunk_pos, biomes);
  }
}

#[cfg(test)]
mod tests {
  use rgen_base::{Biomes, Blocks, ChunkPos};

  use super::*;

  #[test]
  fn test_generator() {
    let mut chunk = Chunk::new();
    let blocks = Blocks::test_blocks();
    let biomes = Biomes::test_blocks();
    let generator = Generator::new(&blocks, &biomes, 1);

    let ctx = ChunkContext { chunk_pos: ChunkPos::new(0, 0), blocks: &blocks };

    generator.generate(&ctx, &mut chunk);
  }
}
