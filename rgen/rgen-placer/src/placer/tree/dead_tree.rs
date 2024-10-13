use rgen_base::{BlockState, Blocks, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct DeadTree {
  pub trunk: BlockState,
}

impl DeadTree {
  pub fn new(blocks: &Blocks) -> Self { DeadTree { trunk: blocks.rgen_log2.with_data(12) } }
}

impl Placer for DeadTree {
  fn radius(&self) -> u8 { 1 }

  fn avg_per_chunk(&self) -> f64 { 2.0 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(4, 7);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return;
    }

    for y in 0..height {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }
  }
}
