use rgen_base::{BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct BasicBirch {
  pub trunk:         BlockState,
  pub leaves:        BlockState,
  pub avg_per_chunk: f64,
  pub is_shrooms:    bool,
  pub shroom:        BlockState,
  pub ground:        BlockState,
}

impl Default for BasicBirch {
  fn default() -> Self {
    BasicBirch {
      trunk:         block![log[variant = "birch"]],
      leaves:        block![leaves[variant = "birch"]],
      avg_per_chunk: 5.0,
      is_shrooms:    true,
      shroom:        block![rgen:polypore[type = "one"]],
      ground:        block![grass],
    }
  }
}

impl Placer for BasicBirch {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let height = rng.range(8..=9);

    // Checks if outside world boundry
    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    // Checks to make sure is in open space
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if world.get(pos + Pos::new(rel_x, 0, rel_z)) != block![air] {
          return Err(UndoError);
        }
      }
    }

    // Checks if on ground
    if world.get(pos + Pos::new(0, -1, 0)) != self.ground {
      return Err(UndoError);
    }

    // Builds the bottom of the canopy
    for y in height - 3..=height - 2_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if rng.range(0..=4) == 1 && x.abs() == 2 && z.abs() == 2 {
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
          if rng.range(0..=4) == 1 && x.abs() == 1 && z.abs() == 1 {
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
            if rng.range(0..9) < 3 {
              let mut state = self.shroom.with_prop("type", *rng.choose(&["one", "two", "three"]));

              // This removes the coners and the center
              if (rel_x == 0 && rel_z == 0) || (rel_x.abs() == rel_z.abs()) {
                continue;
              }

              if rel_x == 1 {
                state.set_prop("facing", "east");
              } else if rel_x == -1 {
                state.set_prop("facing", "west");
              } else if rel_z == 1 {
                state.set_prop("facing", "south");
              } else if rel_z == -1 {
                state.set_prop("facing", "north");
              }

              world.set(pos + Pos::new(rel_x, rel_y, rel_z), state)
            }
          }
        }
      }
    }

    Ok(())
  }
}
