use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Result, Rng, rng::Random};

pub struct LavenderScatter {
  pub place_above: BlockFilter,
  pub place:       BlockState,
  pub is_large:    bool,
  pub attempts:    u32,
}

impl Placer for LavenderScatter {
  fn radius(&self) -> u8 { 8 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    //8  9    10   11
    //0  1    2    3
    let lav_options = [[0, 8], [1, 9], [2, 10], [3, 11]];
    for _ in 0..self.attempts {
      let pos = pos + Pos::new(rng.range(-8..=8), rng.range(-4..=4), rng.range(-8..=8));

      let below_pos = pos + Pos::new(0, -1, 0);

      if self.place_above.contains(world.get(below_pos))
        && world.get(pos) == block![air]
        && world.get(pos + Pos::new(0, 1, 0)) == block![air]
      {
        if self.is_large {
          let bush_var = lav_options[rng.range(0..4) as usize]; //0, 1, 2, & 3

          let bush_dw = self.place.with_data(bush_var[0] as u8);
          let bush_up = self.place.with_data(bush_var[1] as u8);

          world.set(pos, bush_dw);
          world.set(pos + Pos::new(0, 1, 0), bush_up);
        } else {
          let lav = self.place.with_data(rng.range(0..4) as u8);
          world.set(pos, lav);
        }
      }
    }

    Ok(())
  }
}
