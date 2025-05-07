use rgen_base::{BlockState, ChunkRelPos, Pos};

use crate::{BiomeCachedChunk, ChunkPlacer, Random};

pub struct LushCaveMoss {
  pub moss: BlockState,
}

impl LushCaveMoss {
  pub fn new() -> Self { LushCaveMoss { moss: block![rgen:mossy_carpet] } }
}

impl ChunkPlacer for LushCaveMoss {
  fn place(
    &self,
    chunk: &mut BiomeCachedChunk,
    rng: &mut crate::Rng,
    chunk_pos: rgen_base::ChunkPos,
  ) {
    for x in 0..16 {
      for z in 0..16 {
        for y in (0..256).rev() {
          let rel_pos = ChunkRelPos::new(x, y, z);
          if !chunk.is_active(rel_pos) {
            continue;
          }

          let pos = chunk_pos.min_block_pos() + Pos::new(x as i32, y, z as i32);

          let below = chunk.get((pos - Pos::new(0, 1, 0)).chunk_rel());
          let block = chunk.get(pos.chunk_rel());
          if below != block![air] && block == block![air] && rng.range(0..10) == 0 {
            chunk.set(pos.chunk_rel(), self.moss);
          }
        }
      }
    }
  }
}
