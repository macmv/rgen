use rgen_base::{BlockFilter, BlockState, ChunkRelPos};

use crate::{
  noise::{self, OpenSimplexNoise},
  BiomeCachedChunk, ChunkPlacer,
};

pub struct SnowOnStoneSurface {
  pub replace: BlockFilter,

  pub a: BlockState,
  pub b: BlockState,

  noise: OpenSimplexNoise,
}
/*
impl SnowOnStoneSurface {
  pub fn new() -> Self {
    let noise = OpenSimplexNoise::new(0);
    noise.generate(pos.x as f64, pos.z as f64);
  }
}
  */

impl ChunkPlacer for SnowOnStoneSurface {
  fn place(
    &self,
    chunk: &mut BiomeCachedChunk,
    _rng: &mut crate::Rng,
    _chunk_pos: rgen_base::ChunkPos,
  ) {
    for x in 0..16 {
      for z in 0..16 {
        let pos = ChunkRelPos::new(x, 255, z);
        if !chunk.is_active(pos) {
          continue;
        }
        let selected = if (x / 2 + z / 2) % 2 == 0 { self.a } else { self.b };

        for y in (0..256).rev() {
          let pos = pos.with_y(y);

          let block = chunk.chunk.get_state(pos);
          if self.replace.contains(block) {
            chunk.chunk.set(pos, selected);
          }
        }
      }
    }
  }
}
