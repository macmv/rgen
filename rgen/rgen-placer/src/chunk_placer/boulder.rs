use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use super::util::{chunk_rng, count};
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

macro_rules! bool {
  (x) => {
    true
  };
  (.) => {
    false
  };
}
macro_rules! bools {
  ($($x:tt)*) => {
    [$( bool!($x) ),*]
  };
}

/// A small boulder, with a few plants growing on it.
///
/// Boulders never cross a chunk border.
pub struct MossBoulder {
  pub place_above:      BlockFilter,
  pub phobic:           BlockFilter,
  pub material:         BlockState,
  pub avg_per_chunk:    f64,
  pub plant_a:          BlockState,
  pub plant_b:          BlockState,
  pub use_large_plants: bool,
  pub large_plants:     BlockFilter,
}

impl Default for MossBoulder {
  fn default() -> Self {
    MossBoulder {
      place_above:      [block![stone], block![dirt]].into(),
      phobic:           block![grass].into(),
      material:         block![rgen:mossy_cobblestone_rgen],
      avg_per_chunk:    2.0,
      plant_a:          block![tallgrass[2]],
      plant_b:          block![tallgrass[1]],
      use_large_plants: false,
      large_plants:     [block![double_plant[3]], block![double_plant[2]]].into(),
    }
  }
}

impl ChunkPlacer for MossBoulder {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    let mut rng = chunk_rng(rng, chunk_pos);

    for _ in 0..count(&mut rng, self.avg_per_chunk) {
      // Keep the whole 3x3 boulder inside this chunk.
      let column = ChunkRelPos::new(rng.range(1..=14) as u8, 0, rng.range(1..=14) as u8);

      for i in 0..chunk.chunk.surfaces(column).len() {
        let y = chunk.chunk.surfaces(column)[i] as i32 + 1;
        if !chunk.is_active(column.with_y(y - 1)) {
          continue;
        }

        self.place_one(chunk, &mut rng, column.with_y(y));
      }
    }
  }
}

impl MossBoulder {
  fn place_one(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, pos: ChunkRelPos) {
    let at = |x: i32, y: i32, z: i32| {
      ChunkRelPos::new((pos.x() as i32 + x) as u8, pos.y() + y, (pos.z() as i32 + z) as u8)
    };

    let mut bolder_map = [
      [bools!(. . .), bools!(. . .), bools!(. . .)],
      [bools!(. . .), bools!(. . .), bools!(. . .)],
      [bools!(. . .), bools!(. . .), bools!(. . .)],
    ];

    if pos.y() + 20 >= 255 || pos.y() <= 1 {
      return;
    }

    // Checks if the boulder will be built on air
    if !self.place_above.contains(chunk.get(at(0, -1, 0))) || chunk.get(pos) != block![air] {
      return;
    }

    // Check to see if the boulder can be built
    for (y, col) in bolder_map.iter_mut().enumerate() {
      for (x, row) in col.iter_mut().enumerate() {
        for (z, cell) in row.iter_mut().enumerate() {
          let rel_y = y as i32 - 1;
          let rel_x = x as i32 - 1;
          let rel_z = z as i32 - 1;

          // Underground part of the bolder
          if rel_y == -1 {
            if rel_x == 0 || rel_z == 0 {
              if !self.place_above.contains(chunk.get(at(rel_x, rel_y - 1, rel_z))) {
                // The block below the bottom block is not in the list of place
                // above i.e. the boulder is floating and cannot
                // be built thus the build is canceled
                return;
              }

              *cell = true;
            } else if rng.range(0..=3) == 0 {
              *cell = true;
            }

          // Middle of boulder
          } else if rel_y == 0 {
            if rel_x == 0 && rel_z == 0 {
              *cell = true;
            } else if rel_x == 0 || rel_z == 0 {
              if rng.range(0..=15) != 0 {
                *cell = true;
              }
            } else if rng.range(0..=6) != 0 {
              *cell = true;
            }

          // Top of boulder
          } else if rel_y == 1 {
            if rel_x == 0 && rel_z == 0 {
              *cell = true;
            } else if rel_x == 0 || rel_z == 0 {
              if rng.range(0..=6) != 0 {
                *cell = true;
              }
            } else if rng.range(0..=3) == 0 {
              *cell = true;
            }
          }
        }
      }
    }

    // Now the bolder gets built
    for (y, col) in bolder_map.iter().enumerate() {
      for (x, row) in col.iter().enumerate() {
        for (z, cell) in row.iter().enumerate() {
          if *cell {
            chunk.set(at(x as i32 - 1, y as i32 - 1, z as i32 - 1), self.material);
          }
        }
      }
    }

    // Grass and fern placement
    for y in 0..=2_i32 {
      for x in -1..=1_i32 {
        for z in -1..=1_i32 {
          if chunk.get(at(x, y, z)) == block![air] && chunk.get(at(x, y - 1, z)) == self.material {
            let ran = rng.range(0..=5);
            if ran == 0 {
              chunk.set(at(x, y, z), self.plant_a);
            } else if ran == 1 {
              chunk.set(at(x, y, z), self.plant_b);
            }
          }
        }
      }
    }
  }
}
