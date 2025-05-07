use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Result, Rng, rng::Random};

/// Creates spreads(cirlcesish) of blocks 1 above the ground level.
pub struct Spread {
  pub replace:       BlockFilter,
  pub place:         BlockState,
  pub radius:        RangeInclusive<u8>,
  pub avg_per_chunk: f64,
}

impl Default for Spread {
  fn default() -> Self {
    Spread {
      replace:       block![grass].into(),
      place:         block![gravel],
      radius:        2..=5,
      avg_per_chunk: 1.0,
    }
  }
}

impl Placer for Spread {
  fn radius(&self) -> u8 { *self.radius.end() }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let radius = rng.range::<i32>((*self.radius.start()).into()..=(*self.radius.end()).into());

    let r2 = radius.pow(2);

    for x in -radius..=radius {
      for y in -radius..=radius {
        for z in -radius..=radius {
          let pos = pos + Pos::new(x, y, z);

          let dist2 = x.pow(2) + y.pow(2) + z.pow(2);
          if dist2 > r2 {
            continue;
          }

          if dist2 > rng.range(r2 / 2..=r2) {
            continue;
          }

          let below_pos = pos + Pos::new(0, -1, 0);
          if self.replace.contains(world.get(below_pos)) && world.get(pos) == block![air] {
            world.set(pos, self.place);
          }
        }
      }
    }

    Ok(())
  }
}
