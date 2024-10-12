use rgen_base::{Block, BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct Cactus {
  pub place_above:  BlockFilter,
  pub body:         BlockState,
  pub arms:         BlockState,
  pub avg_in_chunk: f64,
}

impl Placer for Cactus {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(2, 3);

    if pos.y + height >= 255 || pos.y <= 1 {
      return;
    }

    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos).block != Block::AIR {
      return;
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

        if world.get(arm_pos) == BlockState::AIR {
          let mut an_arm = self.arms;

          if !x_or_y && unit == -1 {
            an_arm.state = 2;
          } else if !x_or_y && unit == 1 {
            an_arm.state = 0;
          } else if unit == -1 {
            an_arm.state = 1;
          } else {
            an_arm.state = 3;
          }

          world.set(arm_pos, an_arm);
        }
      } else {
        continue;
      }
    }

    // Builds the trunk.
    for y in 0..=height {
      world.set(pos + Pos::new(0, y, 0), self.body);
    }
  }
}
