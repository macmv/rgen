use rgen_base::{BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct DeadTree {
  pub trunk: BlockState,
}

impl Default for DeadTree {
  fn default() -> Self { DeadTree { trunk: block![rgen:log2[12]] } }
}

impl Placer for DeadTree {
  fn radius(&self) -> u8 { 1 }

  fn avg_per_chunk(&self) -> f64 { 2.0 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let height = rng.range(4..=7);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    for y in 0..height {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }

    Ok(())
  }
}
