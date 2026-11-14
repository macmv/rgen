use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use super::util::{chunk_rng, count, for_each_clump_point};
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

/// Clusters of a single plant.
pub struct Clumps {
  pub place_above: BlockFilter,
  pub place:       BlockState,

  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

/// Clusters of short and tall grass.
pub struct GrassClumps {
  pub place_above:      BlockFilter,
  pub place_short:      BlockState,
  pub place_tall_lower: BlockState,
  pub place_tall_upper: BlockState,

  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

/// A small bush: a log, surrounded by leaves.
///
/// Bushes never cross a chunk border.
pub struct BushClumps {
  pub place_above: BlockFilter,
  pub log:         BlockState,
  pub leaves:      BlockState,

  pub radius:        RangeInclusive<u8>,
  pub avg_per_chunk: f64,
}

impl GrassClumps {
  pub fn new() -> Self {
    GrassClumps {
      place_above:      block![grass].into(),
      place_short:      block![tallgrass[1]],     // Grass
      place_tall_lower: block![double_plant[2]],  // Tall grass lower
      place_tall_upper: block![double_plant[10]], // Tall grass upper

      radius:        4..=10,
      attempts:      60,
      avg_per_chunk: 3.0,
    }
  }
}

impl ChunkPlacer for Clumps {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    for_each_clump_point(
      rng.next(),
      chunk_pos,
      self.avg_per_chunk,
      &self.radius,
      self.attempts,
      |column, _| {
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
      },
    );
  }
}

impl ChunkPlacer for GrassClumps {
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
          let above = below.with_y(below.y() + 2);

          if chunk.is_active(below)
            && self.place_above.contains(chunk.get(below))
            && chunk.get(pos) == block![air]
          {
            let height = *rng.choose(&[1, 1, 1, 1, 1, 1, 2]);

            if height == 1 || chunk.get(above) != block![air] {
              chunk.set(pos, self.place_short);
            } else {
              chunk.set(pos, self.place_tall_lower);
              chunk.set(above, self.place_tall_upper);
            }
          }
        }
      },
    );
  }
}

impl ChunkPlacer for BushClumps {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    let mut rng = chunk_rng(rng, chunk_pos);

    for _ in 0..count(&mut rng, self.avg_per_chunk) {
      // The leaves reach up to 2 blocks away from the log.
      let column = ChunkRelPos::new(rng.range(2..=13) as u8, 0, rng.range(2..=13) as u8);

      for i in 0..chunk.chunk.surfaces(column).len() {
        let below = column.with_y(chunk.chunk.surfaces(column)[i] as i32);
        if chunk.is_active(below) {
          self.place_one(chunk, &mut rng, below.with_y(below.y() + 1));
        }
      }
    }
  }
}

impl BushClumps {
  fn place_one(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, pos: ChunkRelPos) {
    let at = |x: i32, y: i32, z: i32| {
      ChunkRelPos::new((pos.x() as i32 + x) as u8, pos.y() + y, (pos.z() as i32 + z) as u8)
    };

    if !(self.place_above.contains(chunk.get(at(0, -1, 0))) && chunk.get(pos) == block![air]) {
      return;
    }

    chunk.set(pos, self.log);

    for (x, y, z) in [
      // surround the log in leaves
      (-1, 0, 0),
      (0, 0, -1),
      (0, 0, 1),
      (1, 0, 0),
      // and build a few leaves on top
      (0, 1, 0),
    ] {
      if chunk.get(at(x, y, z)) == block![air] {
        chunk.set(at(x, y, z), self.leaves);
      }
    }

    // now sprinkle a few more leaves around
    for _ in 0..10 {
      let (x, y, z) = (rng.range(-2..=2), rng.range(0..=1), rng.range(-2..=2));
      if chunk.get(at(x, y, z)) != block![air] && chunk.get(at(x, y + 1, z)) == block![air] {
        chunk.set(at(x, y + 1, z), self.leaves);
      }
    }
  }
}
