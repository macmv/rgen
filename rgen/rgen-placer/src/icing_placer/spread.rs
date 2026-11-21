use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use crate::{BiomeCachedChunk, IcingPlacer, Random, Rng, chunk_placer::util::blobs_near_radius};

/// Creates spreads (circle-ish) of blocks 1 above the ground level.
pub struct Spread {
  pub replace:       BlockFilter,
  pub place:         BlockState,
  pub radius:        RangeInclusive<u8>,
  pub avg_per_chunk: f64,
}

struct MossGroundSpread(Spread);

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

impl Spread {
  /// Turn eligible soil into moss and leave half of those spots uncovered.
  pub fn with_moss_ground(self) -> impl IcingPlacer { MossGroundSpread(self) }
}

impl IcingPlacer for Spread {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    place_spread(chunk, rng, chunk_pos, &self.replace, std::slice::from_ref(&self.place), &self.radius, self.avg_per_chunk, false);
  }
}

impl IcingPlacer for MossGroundSpread {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    place_spread(chunk, rng, chunk_pos, &self.0.replace, std::slice::from_ref(&self.0.place), &self.0.radius, self.0.avg_per_chunk, true);
  }
}

fn place_spread(
  chunk: &mut BiomeCachedChunk,
  rng: &mut Rng,
  chunk_pos: ChunkPos,
  replace: &BlockFilter,
  place: &[BlockState],
  radius_range: &RangeInclusive<u8>,
  avg_per_chunk: f64,
  moss_ground: bool,
) {
    if place.is_empty() { return; }
    let min = chunk_pos.min_block_pos();

    for blob in
      blobs_near_radius(rng.next(), chunk_pos, avg_per_chunk, i32::from(*radius_range.end()))
    {
      let radius = Rng::new(blob.seed)
        .range::<i32>((*radius_range.start()).into()..=(*radius_range.end()).into());
      let r2 = radius.pow(2);

      for x in 0..16_u8 {
        for z in 0..16_u8 {
          let wx = min.x + x as i32;
          let wz = min.z + z as i32;

          let dist2 = (wx - blob.x).pow(2) + (wz - blob.z).pow(2);
          if dist2 > r2 {
            continue;
          }

          let mut column_rng = blob.column_rng(wx, wz);
          if dist2 > column_rng.range(r2 / 2..=r2) {
            continue;
          }

          let column = ChunkRelPos::new(x, 0, z);
          for i in 0..chunk.chunk.surfaces(column).len() {
            let pos = column.with_y(chunk.chunk.surfaces(column)[i] as i32);
            let above = pos.with_y(pos.y() + 1);

            if above.y() <= 255
              && chunk.is_active(pos)
              && replace.contains(chunk.get(pos))
              && chunk.get(above) == block![air]
            {
              if moss_ground {
                chunk.set(pos, block![rgen:mossy_block]);
              }
              if !moss_ground || column_rng.range(0..2) == 0 {
                chunk.set(above, *column_rng.choose(place));
              }
            }
          }
        }
      }
    }
}
