use rgen_base::{Block, BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{rng::Random, Placer, Rng};

pub struct LavenderScatter {
  pub place_above: BlockFilter,
  pub place:       BlockState,
  pub is_large:    bool,
  pub attempts:    u32,
}

impl Placer for LavenderScatter {
  fn radius(&self) -> u8 { 8 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    //8  9    10   11
    //0  1    2    3
    let lav_options = [[0, 8], [1, 9], [2, 10], [3, 11]];
    for _ in 0..self.attempts {
      let pos = pos
        + Pos::new(rng.rand_inclusive(-8, 8), rng.rand_inclusive(-4, 4), rng.rand_inclusive(-8, 8));

      let below_pos = pos + Pos::new(0, -1, 0);

      if self.place_above.contains(world.get(below_pos))
        && world.get(pos).block == Block::AIR
        && world.get(pos + Pos::new(0, 1, 0)).block == Block::AIR
      {
        if self.is_large {
          let bush_var = lav_options[rng.rand_exclusive(0, 4) as usize]; //0, 1, 2, & 3

          let mut bush_dw = self.place;
          bush_dw.state = bush_var[0] as u8;

          let mut bush_up = self.place;
          bush_up.state = bush_var[1] as u8;
          world.set(pos, bush_dw);
          world.set(pos + Pos::new(0, 1, 0), bush_up);
        } else {
          let mut lav = self.place;
          lav.state = rng.rand_exclusive(0, 4) as u8;
          world.set(pos, lav);
        }
      }
    }
  }
}
