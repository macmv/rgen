use rgen_base::{BlockState, Pos};
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
    let height = rng.rand_inclusive(7, 8);
    let min_y = rng.rand_inclusive(-2, -1);

    // Checks if outside world boundry
    if pos.y as i32 + height as i32 + 2 >= 255 || pos.y <= 1 {
      return;
    }

    // Checks to make sure is in open space
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if world.get(pos + Pos::new(rel_x, 1, rel_z)) != BlockState::AIR {
          return;
        }
      }
    }

    // Checks if on ground
    if world.get(pos) != self.ground {
      return;
    }

    // Builds tree
    for y in min_y..=2_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if rng.rand_inclusive(0, 4) == 1 && x.abs() == 2 && z.abs() == 2 {
            continue;
          }

          // Make the top layer smaller.
          if (y == 1 || y == 2) && (x.abs() == 2 || z.abs() == 2) {
            continue;
          }
          world.set(pos + Pos::new(x, (y + height) as u8, z), self.leaves);
        }
      }
    }

    // Builds trunk
    for y in 0..height as u8 {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }

    // Builds polypores
    if self.is_shrooms {
      for rel_y in 2..=4_u8 {
        for rel_x in -1..=1_i32 {
          for rel_z in -1..=1_i32 {
            if rng.rand_exclusive(0, 9) < 3 {
              // Clones a copy of the mushroom that will be mutable
              let mut mushroom = self.shroom.clone();

              //sets mushroom varients (this is exclusive so state 0, 1, or 2)
              let mushroom_variant = rng.rand_exclusive(0, 3);
              mushroom.state = mushroom_variant as u8;

              // Clears the rotation rotation -> 00, block kind -> 11 // no longer nessesary
              mushroom.state &= 0b0011;

              // This removes the coners and the center
              if (rel_x == 0 && rel_z == 0) || (rel_x.abs() == rel_z.abs()) {
                continue;
              }

              // 0-3 +Z   4-7 -Z   8-11 -X   12-15 +X
              if rel_x == 1 {
                // 8
                mushroom.state |= 0b1000;
              } else if rel_x == -1 {
                // 12
                mushroom.state |= 0b1100;
              } else if rel_z == 1 {
                // 4
                mushroom.state |= 0b0100;
              } else if rel_z == -1 {
                // 0
                mushroom.state |= 0b0000;
              }

              world.set(pos + Pos::new(rel_x, rel_y, rel_z), mushroom)
            }
            // ()
          }
        }
      }
    }
  }
}
