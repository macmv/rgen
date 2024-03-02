use rgen_base::{Block, BlockSet, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{rng::Random, Placer, Rng};

pub struct Scatter {
  pub place_above: BlockSet,
  pub place:       BlockState,

  pub attempts: u32,
}

impl Placer for Scatter {
  fn radius(&self) -> u8 { 8 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    for _ in 0..self.attempts {
      let pos = pos
        + Pos::new(
          rng.rand_inclusive(-8, 8),
          rng.rand_inclusive(-4, 4) as u8,
          rng.rand_inclusive(-8, 8),
        );

      let above_pos = pos + Pos::new(0, 1, 0);

      if self.place_above.contains(world.get(pos)) && world.get(above_pos).block == Block::AIR {
        world.set(above_pos, self.place);
      }
    }
  }
}
