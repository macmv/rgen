use rgen_base::{block, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct BasicBirch {
  pub trunk:            BlockState,
  pub leaves:           BlockState,
  pub avg_per_chunk:    f64,
  pub is_shrooms:       bool,
  pub chance_of_shroom: f64,
  pub shroom:           BlockState,
  pub ground:           BlockState,
}

impl Placer for BasicBirch {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(8, 9);

    // Checks if outside world boundry
    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return;
    }

    // Checks to make sure is in open space
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if world.get(pos + Pos::new(rel_x, 0, rel_z)) != block![air] {
          return;
        }
      }
    }

    // Checks if on ground
    if world.get(pos + Pos::new(0, -1, 0)) != self.ground {
      return;
    }

    // Builds the bottom of the canopy
    for y in height - 3..=height - 2_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if rng.rand_inclusive(0, 4) == 1 && x.abs() == 2 && z.abs() == 2 {
            continue;
          }
          //sets the leaves
          world.set(pos + Pos::new(x, y, z), self.leaves);
        }
      }
    }
    // Builds the top of the canopy
    for y in height..=height + 1_i32 {
      for x in -1..=1_i32 {
        for z in -1..=1_i32 {
          // Remove the corners of the top to make the plus shape
          if y == height + 1 && x.abs() == 1 && z.abs() == 1 {
            continue;
          }
          // Sometimes removes the lower level of the leaves on the corner
          if rng.rand_inclusive(0, 4) == 1 && x.abs() == 1 && z.abs() == 1 {
            continue;
          }
          world.set(pos + Pos::new(x, y - 1, z), self.leaves);
        }
      }
    }

    // Builds trunk
    for y in 0..height {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }

    // Builds polypores
    if self.is_shrooms {
      for rel_y in 2..=4_i32 {
        for rel_x in -1..=1_i32 {
          for rel_z in -1..=1_i32 {
            if rng.rand_exclusive(0, 9) < 3 {
              // Set mushroom variants (this is exclusive so state 0, 1, or 2)
              let mushroom_variant = rng.rand_exclusive(0, 3);
              let mut mushroom_state = mushroom_variant as u8;

              // Clears the rotation rotation -> 00, block kind -> 11 // no longer nessesary
              mushroom_state &= 0b0011;

              // This removes the coners and the center
              if (rel_x == 0 && rel_z == 0) || (rel_x.abs() == rel_z.abs()) {
                continue;
              }

              // 0-3 +Z   4-7 -Z   8-11 -X   12-15 +X
              if rel_x == 1 {
                // 8
                mushroom_state |= 0b1000;
              } else if rel_x == -1 {
                // 12
                mushroom_state |= 0b1100;
              } else if rel_z == 1 {
                // 4
                mushroom_state |= 0b0100;
              } else if rel_z == -1 {
                // 0
                mushroom_state |= 0b0000;
              }

              world.set(pos + Pos::new(rel_x, rel_y, rel_z), self.shroom.with_data(mushroom_state))
            }
            // ()
          }
        }
      }
    }
  }
}
