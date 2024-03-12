use rgen_base::{Block, BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct BasicTree {
  pub place_above:  BlockFilter,
  pub trunk:        BlockState,
  pub leaves:       BlockState,
  pub avg_in_chunk: f64,
}

impl Placer for BasicTree {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(5, 8);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return;
    }
    if !self.place_above.contains(world.get(pos))
      || world.get(pos + Pos::new(0, 1, 0)).block != Block::AIR
    {
      return;
    }
    // Does the main body
    for y in -2..=-1_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if (x.abs() == 2 && z.abs() == 2) && rng.rand_inclusive(0, 1) == 0 {
            continue;
          }
          if world.get(pos + Pos::new(x, y + height, z)) == BlockState::AIR {
            world.set(pos + Pos::new(x, y + height, z), self.leaves);
          }
        }
      }
    }
    // Does the peak
    for y in 0..=1_i32 {
      for x in -1..=1_i32 {
        for z in -1..=1_i32 {
          // Remove the corners.
          if x.abs() == 1 && z.abs() == 1 && y == 1 {
            continue; // next loop
          }
          if world.get(pos + Pos::new(x, y + height, z)) == BlockState::AIR {
            world.set(pos + Pos::new(x, y + height, z), self.leaves);
          }
        }
      }
    }
    // Does the trunk
    for y in 1..=height {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }
  }
}
