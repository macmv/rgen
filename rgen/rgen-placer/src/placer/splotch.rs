use std::ops::RangeInclusive;

use rgen_base::{Block, BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{rng::Random, Placer, Rng};

pub struct Splotch {
  pub replace:       BlockFilter,
  pub place:         BlockState,
  pub avg_per_chunk: f64,
  pub radius:        RangeInclusive<u8>,
}

impl Placer for Splotch {
  fn radius(&self) -> u8 { *self.radius.end() }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let radius = rng.rand_inclusive((*self.radius.start()).into(), (*self.radius.end()).into());

    let r2 = radius.pow(2);

    for x in -radius..=radius {
      for y in -radius..=radius {
        for z in -radius..=radius {
          let pos = pos + Pos::new(x, y, z);

          let dist2 = x.pow(2) + y.pow(2) + z.pow(2);
          if dist2 > r2 {
            continue;
          }

          if dist2 > rng.rand_inclusive(r2 / 2, r2) {
            continue;
          }

          if self.replace.contains(world.get(pos))
            && !(world.get(pos + Pos::new(0, 1, 0)).block == Block::WATER)
          {
            world.set(pos, self.place);
          }
        }
      }
    }
  }
}
