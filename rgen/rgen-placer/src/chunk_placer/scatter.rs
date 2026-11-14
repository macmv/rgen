use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use super::util::{chunk_rng, count};
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

/// Places single blocks (plants, usually) on top of the ground at random.
pub struct Scatter {
  pub place_above:   BlockFilter,
  /// How many groups of `attempts` are tried in each chunk, on average.
  pub avg_per_chunk: f64,
  pub place:         BlockState,

  pub attempts: u32,
}

impl Default for Scatter {
  fn default() -> Self {
    Scatter {
      avg_per_chunk: 1.0,
      attempts:      20,
      place_above:   block![grass].into(),
      place:         block![tallgrass[type = "tall_grass"]],
    }
  }
}

impl ChunkPlacer for Scatter {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    let mut rng = chunk_rng(rng, chunk_pos);

    // The old placer picked a random y within +-4 of the anchor, so only 1 in 9
    // attempts landed on the surface. Every attempt here lands on a surface, so
    // scale the count down to keep the same density.
    const Y_RANGE: f64 = 9.0;

    for _ in 0..count(&mut rng, self.avg_per_chunk * self.attempts as f64 / Y_RANGE) {
      let column = ChunkRelPos::new(rng.range(0..16) as u8, 0, rng.range(0..16) as u8);

      for i in 0..chunk.chunk.surfaces(column).len() {
        let below = column.with_y(chunk.chunk.surfaces(column)[i] as i32);
        let pos = below.with_y(below.y() + 1);

        if chunk.is_active(below)
          && self.place_above.contains(chunk.get(below))
          && chunk.get(pos) == block![air]
        {
          chunk.set(pos, self.place);
        }
      }
    }
  }
}
