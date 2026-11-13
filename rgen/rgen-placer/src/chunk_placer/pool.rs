use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use super::util::{chunk_rng, count};
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

macro_rules! bool {
  (w) => {
    0
  };
  (b) => {
    1
  };
  (.) => {
    2
  };
}
macro_rules! bools {
  ($($x:tt)*) => {
    [$( bool!($x) ),*]
  };
}

/// A small pond with a stone and moss border.
///
/// Pools never cross a chunk border.
pub struct Pool {
  pub border_types:  BlockFilter,
  pub avg_per_chunk: f64,
  pub moss:          BlockState,
  pub moss_carpet:   BlockState,
  pub temp_filer:    BlockState,
  pub stone:         BlockState,
  pub clay:          BlockState,
}

impl Default for Pool {
  fn default() -> Self {
    Pool {
      border_types:  [block![stone], block![dirt], block![rgen:mossy_cobblestone_rgen]].into(),
      avg_per_chunk: 12.0,
      moss:          block![rgen:mossy_block],
      moss_carpet:   block![rgen:mossy_carpet],
      temp_filer:    block![concrete[12]],
      stone:         block![stone],
      clay:          block![clay],
    }
  }
}

impl ChunkPlacer for Pool {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    let mut rng = chunk_rng(rng, chunk_pos);

    for _ in 0..count(&mut rng, self.avg_per_chunk) {
      // Keep the whole 4x4 pool inside this chunk.
      let column = ChunkRelPos::new(rng.range(0..=12) as u8, 0, rng.range(0..=12) as u8);

      for i in 0..chunk.chunk.surfaces(column).len() {
        // The level of the ground.
        let y = chunk.chunk.surfaces(column)[i] as i32;
        if !chunk.is_active(column.with_y(y)) {
          continue;
        }

        self.place_one(chunk, &mut rng, column.with_y(y));
      }
    }
  }
}

impl Pool {
  fn place_one(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, level_pos: ChunkRelPos) {
    // w = Water
    // b = Border
    // . = Neither
    let pool_types = [
      [bools!(. b . .), bools!(b w b .), bools!(. b w b), bools!(. . b .)],
      [bools!(. . b .), bools!(. b w b), bools!(b w w b), bools!(. b b .)],
      [bools!(. b . .), bools!(b w b .), bools!(b w w b), bools!(. b b .)],
      [bools!(. b . .), bools!(b w b .), bools!(b w b .), bools!(. b . .)],
      [bools!(. b b .), bools!(b w w b), bools!(b w w b), bools!(. b b .)],
      [bools!(. b . .), bools!(b w b .), bools!(. b . .), bools!(. . . .)],
      [bools!(. b b .), bools!(b w w b), bools!(. b b .), bools!(. b . .)],
    ];

    if level_pos.y() + 21 >= 255 || level_pos.y() < 1 {
      return;
    }

    let at = |x: usize, y: i32, z: usize| {
      ChunkRelPos::new(level_pos.x() + x as u8, level_pos.y() + y, level_pos.z() + z as u8)
    };

    // Check to see is on surface
    let pool_map = rng.choose(&pool_types);
    for (x, row) in pool_map.iter().enumerate() {
      for (z, cell) in row.iter().enumerate() {
        if *cell == 0 {
          // water
          if chunk.get(at(x, 1, z)) != BlockState::AIR {
            return;
          }
          if !self.border_types.contains(chunk.get(at(x, -1, z))) {
            return;
          }
        } else if *cell == 1 {
          // land
          if !self.border_types.contains(chunk.get(at(x, 0, z))) {
            return;
          }
          if !self.border_types.contains(chunk.get(at(x, -1, z))) {
            return;
          }
          if chunk.get(at(x, 1, z)) != BlockState::AIR && rng.range(0..=8) == 0 {
            return;
          }
        }
      }
    }

    // Build pool
    for (x, row) in pool_map.iter().enumerate() {
      for (z, cell) in row.iter().enumerate() {
        if *cell == 0 {
          // water
          chunk.set(at(x, 0, z), block![water]);
          if rng.range(0..=1) == 0 {
            chunk.set(at(x, -1, z), self.clay);
          }
        } else if *cell == 1 {
          // border
          if rng.range(0..=5) == 0 {
            chunk.set(at(x, 0, z), self.moss);
            if rng.range(0..=2) == 0 && chunk.get(at(x, 1, z)) == BlockState::AIR {
              chunk.set(at(x, 1, z), self.moss_carpet);
            }
          } else {
            chunk.set(at(x, 0, z), self.stone);
          }
          if rng.range(0..=8) == 0 && chunk.get(at(x, 1, z)) == BlockState::AIR {
            chunk.set(at(x, 1, z), self.moss_carpet);
          }
        }
      }
    }
  }
}
