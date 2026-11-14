use rgen_base::{BlockState, ChunkPos, ChunkRelPos};

use super::util::chunk_rng;
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

/// Scatters single blocks of a different material across the surface.
pub struct Splatter {
  pub replace: BlockState,
  pub place:   BlockState,

  /// How many random columns are tried in each chunk.
  pub attempts: u32,
}

impl Default for Splatter {
  fn default() -> Self { Splatter { replace: block![stone], place: block![grass], attempts: 10 } }
}

impl ChunkPlacer for Splatter {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    let mut rng = chunk_rng(rng, chunk_pos);

    for _ in 0..self.attempts {
      let column = ChunkRelPos::new(rng.range(0..16) as u8, 0, rng.range(0..16) as u8);

      for i in 0..chunk.chunk.surfaces(column).len() {
        let pos = column.with_y(chunk.chunk.surfaces(column)[i] as i32);

        if chunk.is_active(pos) && chunk.get(pos) == self.replace {
          chunk.set(pos, self.place);
        }
      }
    }
  }
}
