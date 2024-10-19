use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct LogAndStump {
  pub log:            BlockState,
  pub moss_log:       BlockState,
  pub ground:         BlockState,
  pub plants:         BlockFilter,
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

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    // Checks to make sure is in open space
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if world.get(pos + Pos::new(rel_x, 0, rel_z)) != block![air] {
          return Err(UndoError);
        }
      }
    }

    // Checks if on ground
    let below_pos = pos + Pos::new(0, -1, 0);
    if world.get(below_pos) != self.ground {
      return Err(UndoError);
    }

    if self.place_stump(world, rng, pos) {
      self.place_log(world, rng, pos);
    }

    Ok(())
  }
}

impl LogAndStump {
  fn place_stump(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> bool {
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if world.get(pos + Pos::new(rel_x, 0, rel_z)) != block![air] {
          return false;
        }
      }
    }
    world.set(pos, self.moss_log);

    if self.is_shrooms {
      for rel_x in -1..=1_i32 {
        for rel_z in -1..=1_i32 {
          if world.get(pos + Pos::new(rel_x, 0, rel_z)) != block![air] {
            continue;
          }
          if rng.rand_exclusive(0, 9) < 3 {
            //sets mushroom varients (this is exclusive so state 0, 1, or 2)
            let mut mushroom_state = rng.rand_exclusive(0, 3) as u8;

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

            world.set(pos + Pos::new(rel_x, 0, rel_z), self.shroom.with_data(mushroom_state))
          }
          // ()
        }
      }
    }
    true
  }

  fn place_log(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> bool {
    let mut dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    rng.shuffle(&mut dirs);

    for (dx, dz) in dirs {
      let mut buildable = true;
      let length = rng.rand_inclusive(4, 6);
      let pos_st = pos + Pos::new(dx * (length - (length - 2)), -1, dz * (length - (length - 2)));
      let pos_nd = pos + Pos::new(dx * length, -1, dz * length);
      if (world.get(pos_st) != block![air])
        && (world.get(pos_st) != block![water])
        && (world.get(pos_nd) != block![air])
        && (world.get(pos_nd) != block![water])
      {
        for i in 1..=length {
          let i_pos = pos + Pos::new(i * dx, 0, i * dz);
          if world.get(i_pos) != block![air] {
            buildable = false;
            break;
          }
        }
      } else {
        buildable = false;
      }

      if !buildable {
        continue;
      } else {
        for i in 2..=length {
          let i_pos = pos + Pos::new(i * dx, 0, i * dz);

          let log_block =
            if self.chance_of_moss < rng.rand_inclusive(0, 10) { self.moss_log } else { self.log };
          let mut log_state = log_block.state.state().unwrap_or_default();

          log_state &= 0b0011; //reset

          if dx != 0 {
            // x axis be it (5, 6)
            log_state |= 0b0100;
          } else {
            // z axis be it (9, 10)
            log_state |= 0b1000;
          }

          world.set(i_pos, log_block.with_data(log_state));
        }
        return true;
      }
    }
    false
  }
}
