use crate::{chunk::Chunk, pos::ChunkRelPos, ChunkContext};

pub trait BiomeGenerator {
  fn generate(&self, ctx: &ChunkContext, chunk: &mut Chunk);
}

pub struct Plains;

impl BiomeGenerator for Plains {
  fn generate(&self, ctx: &ChunkContext, chunk: &mut Chunk) {
    for y in 0..=255 {
      chunk.set(ChunkRelPos::new(3, y, 3), ctx.blocks.stone);
    }
  }
}
