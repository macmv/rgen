use crate::{chunk::Chunk, ctx::Blocks, pos::ChunkRelPos};

pub struct Generator {
  seed: u64,
}

impl Generator {
  pub fn new(seed: u64) -> Generator { Generator { seed } }

  pub fn generate(&self, chunk_x: i32, chunk_z: i32, blocks: &Blocks, chunk: &mut Chunk) {
    chunk.set(ChunkRelPos::new(0, 6, 0), blocks.dirt);

    for x in 0..16 {
      for z in 0..16 {
        chunk.set(ChunkRelPos::new(x, 0, z), blocks.stone);
      }
    }
  }
}
