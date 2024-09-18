use rgen_base::{BlockFilter, BlockState, ChunkRelPos};

use crate::{BiomeCachedChunk, ChunkPlacer};

pub struct CheckerboardSurface {
  pub replace: BlockFilter,

  pub a: BlockState,
  pub b: BlockState,
}

impl ChunkPlacer for CheckerboardSurface {
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
