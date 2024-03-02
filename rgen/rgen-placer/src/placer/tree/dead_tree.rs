use rgen_base::{Block, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct DeadTree {
  pub trunk: Block,
}

impl Placer for DeadTree {
  fn radius(&self) -> u8 { 1 }

  fn avg_per_chunk(&self) -> f64 { 16.0 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(4, 7);

    if pos.y as i32 + height as i32 + 2 >= 255 || pos.y <= 1 {
      return;
    }

    for y in 0..height as u8 {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }
  }
}
