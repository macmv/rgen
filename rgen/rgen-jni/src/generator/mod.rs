use crate::{
  biome::BiomeGenerator,
  chunk::Chunk,
  noise::{octaved::OctavedNoise, perlin::PerlinNoise, NoiseGenerator},
  pos::{ChunkRelPos, Pos},
  ChunkContext,
};

mod climate;

pub struct Generator {
  seed: u64,

  height_map: OctavedNoise<PerlinNoise>,
}

impl Generator {
  pub fn new(seed: u64) -> Generator {
    Generator {
      seed,
      height_map: OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },
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

    let biome = crate::biome::Plains;

    biome.generate(ctx, chunk);

    chunk.set(ChunkRelPos::new(0, 6, 0), ctx.blocks.dirt);
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    ctx::{Block, Blocks},
    pos::ChunkPos,
  };

  use super::*;

  fn blocks() -> Blocks { Blocks { stone: Block::from_raw_id(1), dirt: Block::from_raw_id(2) } }

  #[test]
  fn test_generator() {
    let mut chunk = Chunk::new();
    let blocks = blocks();
    let generator = Generator::new(1);

    let ctx = ChunkContext { chunk_pos: ChunkPos::new(0, 0), blocks: &blocks };

    generator.generate(&ctx, &mut chunk);
  }
}
