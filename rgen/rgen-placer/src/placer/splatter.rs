use rgen_base::{BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{rng::Random, Placer, Result, Rng};

pub struct Splatter {
  pub replace: BlockState,
  pub place:   BlockState,

  pub attempts: u32,
}

impl Placer for Splatter {
  fn radius(&self) -> u8 { 8 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    for _ in 0..self.attempts {
      let pos = pos
        + Pos::new(rng.rand_inclusive(-8, 8), rng.rand_inclusive(-4, 4), rng.rand_inclusive(-8, 8));

      let below_pos = pos + Pos::new(0, -1, 0);
      if world.get(below_pos) == self.replace {
        world.set(below_pos, self.place);
      }
    }

    Ok(())
  }
}
