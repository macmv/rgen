use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct BasicTree {
  pub place_above:   BlockFilter,
  pub trunk:         BlockState,
  pub leaves:        BlockState,
  pub avg_per_chunk: f64,
}

impl Default for BasicTree {
  fn default() -> Self {
    BasicTree {
      place_above:   block![grass].into(),
      trunk:         block![log[variant = "oak"]],
      leaves:        block![leaves[variant = "oak"]],
      avg_per_chunk: 0.8,
    }
  }
}

impl Placer for BasicTree {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let height = rng.range(5..=8);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }

    // Builds the main body.
    for y in -2..=-1_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if (x.abs() == 2 && z.abs() == 2) && rng.range(0..=1) == 0 {
            continue;
          }

          let pos = pos + Pos::new(x, y + height, z);
          if world.get(pos) == BlockState::AIR {
            world.set(pos, self.leaves);
          }
        }
      }
    }

    // Builds the peak.
    for y in 0..=1_i32 {
      for x in -1..=1_i32 {
        for z in -1..=1_i32 {
          // Remove the corners.
          if x.abs() == 1 && z.abs() == 1 && y == 1 {
            continue; // next loop
          }

          let pos = pos + Pos::new(x, y + height, z);
          if world.get(pos) == BlockState::AIR {
            world.set(pos, self.leaves);
          }
        }
      }
    }

    // Builds the trunk.
    for y in 0..=height {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }

    Ok(())
  }
}
