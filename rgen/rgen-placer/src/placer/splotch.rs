use rgen_base::{BlockSet, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{rng::Random, Placer, Rng};

pub struct Splotch {
  pub replace: BlockSet,
  pub place:   BlockState,

  pub radius: u8,
}

impl Placer for Splotch {
  fn radius(&self) -> u8 { self.radius }
  fn avg_per_chunk(&self) -> f64 { 1.0 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let r2 = (self.radius as i32).pow(2);

    for x in -(self.radius as i32)..=self.radius as i32 {
      for y in -(self.radius as i32)..=self.radius as i32 {
        for z in -(self.radius as i32)..=self.radius as i32 {
          let pos = pos + Pos::new(x, y as u8, z);

          let dist2 = x.pow(2) + y.pow(2) + z.pow(2);
          if dist2 > r2 {
            continue;
          }

          if dist2 > rng.rand_inclusive(r2 / 4 * 3, r2) {
            continue;
          }

          if self.replace.contains(world.get(pos)) {
            world.set(pos, self.place);
          }
        }
      }
    }
  }
}
