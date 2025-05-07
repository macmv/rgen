use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Result, Rng, rng::Random};

pub struct Scatter {
  pub place_above:   BlockFilter,
  pub avg_per_chunk: f64,
  pub place:         BlockState,

  pub attempts: u32,
}

impl Default for Scatter {
  fn default() -> Self {
    Scatter {
      avg_per_chunk: 1.0,
      attempts:      20,
      place_above:   block![grass].into(),
      place:         block![tallgrass[type = "tall_grass"]],
    }
  }
}

impl Placer for Scatter {
  fn radius(&self) -> u8 { 8 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    for _ in 0..self.attempts {
      let pos = pos + Pos::new(rng.range(-8..=8), rng.range(-4..=4), rng.range(-8..=8));

      let below_pos = pos + Pos::new(0, -1, 0);

      if self.place_above.contains(world.get(below_pos)) && world.get(pos) == block![air] {
        world.set(pos, self.place);
      }
    }

    Ok(())
  }
}
