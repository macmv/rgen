use rgen_base::{BlockSet, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct LogAndStump {
  pub log:            BlockState,
  pub moss_log:       BlockState,
  pub ground:         BlockState,
  pub plants:         BlockSet,
  pub avg_per_chunk:  f64,
  pub chance_of_moss: i32,
  pub is_shrooms:     bool,
  pub shroom:         BlockState,
}

impl Placer for LogAndStump {
  //log_moss
  //chance_of_moss
  //plants

  fn radius(&self) -> u8 { 9 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
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

    if self.place_stump(world, rng, pos) {
      self.place_log()
    }

    //call log fn
  }
}

impl LogAndStump {
  fn place_stump(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> bool {
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if world.get(pos + Pos::new(rel_x, 1, rel_z)) != BlockState::AIR {
          return false;
        }
      }
    }
    world.set(pos + Pos::new(0, 1, 0), self.moss_log);

    if self.is_shrooms {
      for rel_x in -1..=1_i32 {
        for rel_z in -1..=1_i32 {
          if world.get(pos + Pos::new(rel_x, 1, rel_z)) != BlockState::AIR {
            continue;
          }
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

            world.set(pos + Pos::new(rel_x, 1, rel_z), mushroom)
          }
          // ()
        }
      }
    }
    return true;
  }
  fn place_log(&self) {}
}
