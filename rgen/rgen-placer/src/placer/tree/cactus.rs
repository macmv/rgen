use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct Cactus {
  pub place_above:  BlockFilter,
  pub body:         BlockState,
  pub arms:         BlockState,
  pub avg_in_chunk: f64,
}

impl Placer for Cactus {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let height = rng.rand_inclusive(2, 3);

    if pos.y + height >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }

    // Builds the main body.

    let x_or_y = rng.rand_inclusive(0, 1) == 1;

    for unit in -1..=1_i32 {
      if unit == 0 {
        continue;
      }

      let y_chance = rng.rand_inclusive(-1, 1);
      if y_chance != 1 {
        let mut arm_pos = pos;

        if x_or_y {
          arm_pos = arm_pos + Pos::new(unit, height + y_chance, 0);
        } else {
          arm_pos = arm_pos + Pos::new(0, height + y_chance, unit);
        }

        if world.get(arm_pos) == block![air] {
          let arm_state = if !x_or_y && unit == -1 {
            2
          } else if !x_or_y && unit == 1 {
            0
          } else if unit == -1 {
            1
          } else {
            3
          };

          world.set(arm_pos, self.arms.with_data(arm_state));
        }
      } else {
        continue;
      }
    }

    // Builds the trunk.
    for y in 0..=height {
      world.set(pos + Pos::new(0, y, 0), self.body);
    }

    Ok(())
  }
}
