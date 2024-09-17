use rgen_base::{Block, BlockState, ChunkRelPos, Pos};

use crate::{BiomeCachedChunk, ChunkPlacer};

pub struct CheckerboardSurface {
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

        let mut depth = 0;
        for y in (0..256).rev() {
          let pos = pos.with_y(y);

          let block = chunk.chunk.get(pos);
          if block == Block::AIR {
            depth = 0;
          } else {
            depth += 1;
          }
          if depth > 0 {
            chunk.chunk.set(pos, selected);
          }
        }
      }
    }
  }
}
