use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Result, Rng};

pub struct BasicDryBush {
  pub place_above:   BlockFilter,
  pub trunk:         BlockState,
  pub leaves:        BlockState,
  pub avg_per_chunk: f64,
}

impl Placer for BasicDryBush {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, _rng: &mut Rng, pos: Pos) -> Result {
    // Checks if outside world boundry.
    if pos.y + 2 + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    // Checks if on ground.
    if !self.place_above.contains(world.get(pos + Pos::new(0, -1, 0))) {
      return Err(UndoError);
    }

    // Creates the core.
    world.set(pos, self.trunk);

    for y in 0..=1_i32 {
      for x in -1..=1_i32 {
        for z in -1..=1_i32 {
          // Remove the corners.
          if x.abs() == 1 && z.abs() == 1 && y == 1 {
            continue; // next loop
          }

          let pos = pos + Pos::new(x, y, z);
          if world.get(pos) == block![air] {
            world.set(pos, self.leaves);
          }
        }
      }
    }

    Ok(())
  }
}
