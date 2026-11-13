use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use crate::{BiomeCachedChunk, IcingPlacer, Random, Rng, chunk_placer::util::for_each_clump_point};

/// A single bamboo stalk.
pub struct Bamboo {
  pub stalk:     BlockState,
  pub pint_size: bool,
}

/// Clusters of bamboo.
///
/// This is an icing placer, so every tree (including ones from neighboring
/// chunks) is already in place. A stalk never grows into a leaf: it is cut
/// short to fit under the canopy, and skipped if there is not enough room.
pub struct BambooClump {
  pub bamboo: Bamboo,

  pub place_above:   BlockFilter,
  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

/// A stalk which doesn't have this much room is not placed at all, as it would
/// just be a stub sticking out of the ground.
const MIN_HEIGHT: i32 = 6;

impl Default for Bamboo {
  fn default() -> Self { Bamboo { stalk: block![rgen:bamboo], pint_size: true } }
}

impl Default for BambooClump {
  fn default() -> Self {
    BambooClump {
      bamboo:        Bamboo::default(),
      place_above:   block![grass].into(),
      attempts:      10,
      radius:        1..=4,
      avg_per_chunk: 3.0,
    }
  }
}

impl Bamboo {
  /// Grows a stalk from `pos`, which must be the (air) block above the ground.
  fn grow(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, pos: ChunkRelPos) {
    let height = if self.pint_size { rng.range(8..=14) } else { rng.range(15..=20) };
    let shoot = rng.range(0..=3) as u8;

    if pos.y() + height + 2 >= 255 || pos.y() <= 1 {
      return;
    }

    // Count how many blocks of air there are above the ground. We stop at the
    // first thing in the way, which is almost always a leaf.
    let mut height_clear = 0;
    while height_clear <= height && chunk.get(pos.with_y(pos.y() + height_clear)) == block![air] {
      height_clear += 1;
    }

    // `height_clear` is the number of blocks of air, so the stalk has
    // `height_clear - 1` blocks above the first one.
    let height = (height_clear - 1).min(height);
    if height_clear < MIN_HEIGHT {
      return;
    }

    // The top three blocks have leaves.
    let leaf = shoot | 0b0100;

    for y in 0..=height {
      chunk.set(
        pos.with_y(pos.y() + y),
        self.stalk.with_data(if y > height - 3 { leaf } else { shoot }),
      );
    }
  }
}

impl IcingPlacer for BambooClump {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    for_each_clump_point(
      rng.next(),
      chunk_pos,
      self.avg_per_chunk,
      &self.radius,
      self.attempts,
      |column, seed| {
        let mut rng = Rng::new(seed);

        for i in 0..chunk.chunk.surfaces(column).len() {
          let below = column.with_y(chunk.chunk.surfaces(column)[i] as i32);
          let pos = below.with_y(below.y() + 1);

          if chunk.is_active(below)
            && self.place_above.contains(chunk.get(below))
            && chunk.get(pos) == block![air]
          {
            self.bamboo.grow(chunk, &mut rng, pos);
          }
        }
      },
    );
  }
}
