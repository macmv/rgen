use std::ops::RangeInclusive;

use rgen_base::{BlockState, ChunkPos, ChunkRelPos};

use super::util::blobs_near;
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

/// Replaces the floor of lakes and oceans with a round patch of some other
/// block (clay, sand, etc).
pub struct WaterResources {
  pub placement:          BlockState,
  pub tool_placement:     BlockState,
  pub tool_placement_two: BlockState,
  /// How many patches are placed per chunk, on average, is `avg_per_chunk *
  /// multiplier`.
  pub avg_per_chunk:      f64,
  pub size:               RangeInclusive<u8>,
  pub multiplier:         i32,
}

impl Default for WaterResources {
  fn default() -> Self {
    WaterResources {
      avg_per_chunk:      1.0,
      placement:          block![clay[0]],
      tool_placement:     block![gold_block[0]],
      tool_placement_two: block![iron_ore[0]],
      size:               2..=4,
      multiplier:         3,
    }
  }
}

/// Patches are only placed in water at most this deep.
const MAX_DEPTH: i32 = 10;

impl ChunkPlacer for WaterResources {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    let min = chunk_pos.min_block_pos();

    let avg = self.avg_per_chunk * self.multiplier as f64;
    for blob in blobs_near(rng.next(), chunk_pos, avg) {
      let radius =
        Rng::new(blob.seed).range::<i32>((*self.size.start()).into()..=(*self.size.end()).into());
      let r2 = radius.pow(2);

      for x in 0..16_u8 {
        for z in 0..16_u8 {
          let wx = min.x + x as i32;
          let wz = min.z + z as i32;

          let dist2 = (wx - blob.x).pow(2) + (wz - blob.z).pow(2);
          if dist2 > r2 {
            continue;
          }

          if dist2 > blob.column_rng(wx, wz).range(r2 / 2..=r2) {
            continue;
          }

          let column = ChunkRelPos::new(x, 0, z);
          for i in 0..chunk.chunk.surfaces(column).len() {
            let pos = column.with_y(chunk.chunk.surfaces(column)[i] as i32);

            if pos.y() >= 63 || !chunk.is_active(pos) {
              continue;
            }

            // Only the floor of a body of water, that isn't too deep.
            let mut depth = 0;
            while depth <= MAX_DEPTH && chunk.get(pos.with_y(pos.y() + 1 + depth)) == block![water]
            {
              depth += 1;
            }
            if depth == 0 || depth > MAX_DEPTH {
              continue;
            }

            chunk.set(pos, self.placement);
          }
        }
      }
    }
  }
}
