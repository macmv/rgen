use rgen_base::{block, BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

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
pub struct MossBoulder {
  pub place_above:      BlockFilter,
  pub phobic:           BlockFilter,
  pub material:         BlockState,
  pub avg_in_chunk:     f64,
  pub plant_a:          BlockState,
  pub plant_b:          BlockState,
  pub use_large_plants: bool,
  pub large_plants:     BlockFilter,
}

impl MossBoulder {
  pub fn new() -> Self {
    MossBoulder {
      place_above:      [block![stone], block![dirt]].into(),
      phobic:           block![grass].into(),
      material:         /*blocks.wool.with_data(6), */ block![rgen:mossy_cobblestone_rgen],
      avg_in_chunk:     2.0,
      plant_a:          block![tallgrass[2]],
      plant_b:          block![tallgrass[1]],
      use_large_plants: false,
      large_plants:     [block![double_plant[3]], block![double_plant[2]]].into(),
    }
  }
}

impl Placer for MossBoulder {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    // Checks if tree will breach build height
    let mut bolder_map = [
      [bools!(. . .), bools!(. . .), bools!(. . .)],
      [bools!(. . .), bools!(. . .), bools!(. . .)],
      [bools!(. . .), bools!(. . .), bools!(. . .)],
    ];

    if pos.y + 20 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    // Checks if tree will be built on air
    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }

    //let min_y = rng.rand_inclusive(4, 6);

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
              if !self.place_above.contains(world.get(pos + Pos::new(rel_x, rel_y - 1, rel_z))) {
                // The block below the bottom block is not in the list of place above i.e. the
                // boulder is floating and cannot be built thus the build is canceled

                return Err(UndoError);
              }

              *cell = true;
            } else if rng.rand_inclusive(0, 3) == 0 {
              *cell = true;
            }

          // Middle of boulder
          } else if rel_y == 0 {
            if rel_x == 0 && rel_z == 0 {
              *cell = true;
            } else if rel_x == 0 || rel_z == 0 {
              if rng.rand_inclusive(0, 15) != 0 {
                *cell = true;
              }
            } else if rng.rand_inclusive(0, 6) != 0 {
              *cell = true;
            }

          // Top of boulder
          } else if rel_y == 1 {
            if rel_x == 0 && rel_z == 0 {
              *cell = true;
            } else if rel_x == 0 || rel_z == 0 {
              if rng.rand_inclusive(0, 6) != 0 {
                *cell = true;
              }
            } else if rng.rand_inclusive(0, 3) == 0 {
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
          self.build_bolder(world, pos, *cell, x as i32 - 1, y as i32 - 1, z as i32 - 1);
        }
      }
    }

    //grass and fern placment
    for y in 0..=2_i32 {
      for x in -1..=1_i32 {
        for z in -1..=1_i32 {
          if world.get(pos + Pos::new(x, y, z)) == BlockState::AIR
            && world.get(pos + Pos::new(x, y - 1, z)) == self.material
          {
            let ran = rng.rand_inclusive(0, 5);
            if ran == 0 {
              world.set(pos + Pos::new(x, y, z), self.plant_a);
            } else if ran == 1 {
              world.set(pos + Pos::new(x, y, z), self.plant_b);
            }
          }
        }
      }
    }

    Ok(())
  }
}

impl MossBoulder {
  fn build_bolder(
    &self,
    world: &mut PartialWorld,
    pos: Pos,
    cell: bool,
    x: i32,
    height: i32,
    z: i32,
  ) {
    let rel_pos = pos + Pos::new(x, height, z);
    if cell {
      world.set(rel_pos, self.material);
    }
  }
}
