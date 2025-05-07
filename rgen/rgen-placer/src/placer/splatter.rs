use rgen_base::{BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Result, Rng, rng::Random};

pub struct Splatter {
  pub replace: BlockState,
  pub place:   BlockState,

  pub attempts: u32,
}

impl Default for Splatter {
  fn default() -> Self { Splatter { replace: block![stone], place: block![grass], attempts: 10 } }
}

impl Placer for Splatter {
  fn radius(&self) -> u8 { 8 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    for _ in 0..self.attempts {
      let pos = pos + Pos::new(rng.range(-8..=8), rng.range(-4..=4), rng.range(-8..=8));

      let below_pos = pos + Pos::new(0, -1, 0);
      if world.get(below_pos) == self.replace {
        world.set(below_pos, self.place);
      }
    }

    Ok(())
  }
}
