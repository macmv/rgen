use rgen_base::{Block, BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct BasicTree {
  pub place_above: BlockFilter,
  pub trunk:       BlockState,
  pub leaves:      BlockState,
  //pub avg_in_chunk: f64,
}

impl Placer for BasicTree {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { 16.0 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(3, 6);
    let min_y = rng.rand_inclusive(-2, -1);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return;
    }
    if !self.place_above.contains(world.get(pos))
      || world.get(pos + Pos::new(0, 1, 0)).block != Block::AIR
    {
      return;
    }

    for y in min_y..=1_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if x.abs() == 2 && z.abs() == 2 {
            continue;
          }
          // Make the top layer smaller.
          if y == 1 && (x.abs() == 2 || z.abs() == 2) {
            continue;
          }

          world.set(pos + Pos::new(x, y + height, z), self.leaves);
        }
      }
    }

    for y in 1..=height {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }
  }
}
