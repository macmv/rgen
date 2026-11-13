use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use super::util::blobs_near;
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

/// Creates spreads (circle-ish) of blocks 1 above the ground level.
pub struct Spread {
  pub replace:       BlockFilter,
  pub place:         BlockState,
  pub radius:        RangeInclusive<u8>,
  pub avg_per_chunk: f64,
}

impl Default for Spread {
  fn default() -> Self {
    Spread {
      replace:       block![grass].into(),
      place:         block![gravel],
      radius:        2..=5,
      avg_per_chunk: 1.0,
    }
  }
}

impl ChunkPlacer for Spread {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    let min = chunk_pos.min_block_pos();

    for blob in blobs_near(rng.next(), chunk_pos, self.avg_per_chunk) {
      let radius = Rng::new(blob.seed)
        .range::<i32>((*self.radius.start()).into()..=(*self.radius.end()).into());
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
            let above = pos.with_y(pos.y() + 1);

            if chunk.is_active(pos)
              && self.replace.contains(chunk.get(pos))
              && chunk.get(above) == block![air]
            {
              chunk.set(above, self.place);
            }
          }
        }
      }
    }
  }
}
