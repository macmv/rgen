use rgen_base::{Block, BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

use super::birch;

pub struct Bamboo {
  pub place_above:  BlockFilter,
  pub stalk:        BlockState,
  pub pint_size:    bool,
  pub avg_in_chunk: f64,
}

impl Placer for Bamboo {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let mut height = 15;
    if self.pint_size {
      height = rng.rand_inclusive(8, 14);
    } else {
      height = rng.rand_inclusive(15, 20);
    }

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return;
    }
    if !self.place_above.contains(world.get(pos))
      || world.get(pos + Pos::new(0, 1, 0)).block != Block::AIR
    {
      return;
    }

    rng.rand_inclusive(15, 20);
    let mut shoot = self.stalk.clone();

    // Sets rotation
    shoot.state &= 0b1100;
    let rand = rng.rand_inclusive(0, 3);
    if rand == 0 {
      // 0
      shoot.state |= 0b0000;
    } else if rand == 1 {
      // 1
      shoot.state |= 0b0001;
    } else if rand == 2 {
      // 2
      shoot.state |= 0b0010;
    } else if rand == 3 {
      // 3
      shoot.state |= 0b0011;
    }

    let mut leaf = shoot.clone();
    // Sets leaft type
    leaf.state &= 0b0011;
    leaf.state |= 0b0100;

    for y in 1..=height {
      if y > height - 3 {
        if world.get(pos + Pos::new(0, y, 0)) == BlockState::AIR {
          world.set(pos + Pos::new(0, y, 0), leaf);
        } else {
          return;
        }
      } else {
        if world.get(pos + Pos::new(0, y, 0)) == BlockState::AIR {
          world.set(pos + Pos::new(0, y, 0), shoot);
        } else {
          return;
        }
      }
    }
  }
}
