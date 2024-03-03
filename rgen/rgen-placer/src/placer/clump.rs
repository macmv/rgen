use std::ops::RangeInclusive;

use rgen_base::{Block, BlockSet, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{rng::Random, Placer, Rng};

pub struct Clumps {
  pub place_above: BlockSet,
  pub place:       BlockState,

  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

pub struct GrassClumps {
  pub place_above:      BlockSet,
  pub place_short:      BlockState,
  pub place_tall_lower: BlockState,
  pub place_tall_upper: BlockState,

  pub radius:   RangeInclusive<u8>,
  pub attempts: u32,
}

pub struct BushClumps {
  pub place_above: BlockSet,
  pub log:         BlockState,
  pub leaves:      BlockState,

  pub radius:   RangeInclusive<u8>,
  pub attempts: u32,
}

impl Placer for Clumps {
  fn radius(&self) -> u8 { *self.radius.end() }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let radius = rng.rand_inclusive(*self.radius.start() as i32, *self.radius.end() as i32);

    for _ in 0..self.attempts {
      let mut pos = pos;
      for _ in 0..radius {
        pos = pos + Pos::new(rng.rand_inclusive(-1, 1), 0, rng.rand_inclusive(-1, 1));
      }

      let above_pos = pos + Pos::new(0, 1, 0);

      if self.place_above.contains(world.get(pos)) && world.get(above_pos).block == Block::AIR {
        world.set(above_pos, self.place);
      }
    }
  }
}

impl Placer for GrassClumps {
  fn radius(&self) -> u8 { *self.radius.end() }
  fn avg_per_chunk(&self) -> f64 { 3.0 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let radius = rng.rand_inclusive(*self.radius.start() as i32, *self.radius.end() as i32);

    for _ in 0..self.attempts {
      let mut pos = pos;
      for _ in 0..radius {
        pos = pos + Pos::new(rng.rand_inclusive(-1, 1), 0, rng.rand_inclusive(-1, 1));
      }

      let above_pos = pos + Pos::new(0, 1, 0);

      if self.place_above.contains(world.get(pos)) && world.get(above_pos).block == Block::AIR {
        let height = *rng.choose(&[1, 1, 1, 1, 1, 1, 2]);

        if height == 1 {
          world.set(above_pos, self.place_short);
        } else {
          world.set(above_pos, self.place_tall_lower);
          world.set(above_pos + Pos::new(0, 1, 0), self.place_tall_upper);
        }
      }
    }
  }
}

impl Placer for BushClumps {
  fn radius(&self) -> u8 { *self.radius.end() }
  fn avg_per_chunk(&self) -> f64 { 0.3 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let radius = rng.rand_inclusive(*self.radius.start() as i32, *self.radius.end() as i32);

    for _ in 0..self.attempts {
      let mut pos = pos;
      for _ in 0..radius {
        pos = pos + Pos::new(rng.rand_inclusive(-1, 1), 0, rng.rand_inclusive(-1, 1));
      }

      if self.place_above.contains(world.get(pos)) {
        let above = pos + Pos::new(0, 1, 0);
        if world.get(above).block == Block::AIR {
          world.set(above, self.log);

          for offset in [
            // surround the log in leaves
            Pos::new(-1, 0, 0),
            Pos::new(0, 0, -1),
            Pos::new(0, 0, 1),
            Pos::new(1, 0, 0),
            // and build a few leaves on top
            Pos::new(0, 1, 0),
          ] {
            let side = above + offset;
            if world.get(side).block == Block::AIR {
              world.set(side, self.leaves);
            }
          }

          // now sprink a few more leaves around
          for _ in 0..10 {
            let side_below = pos
              + Pos::new(
                rng.rand_inclusive(-2, 2),
                rng.rand_inclusive(0, 1) as u8,
                rng.rand_inclusive(-2, 2),
              );
            let side = side_below + Pos::new(0, 1, 0);
            if world.get(side_below).block != Block::AIR && world.get(side).block == Block::AIR {
              world.set(side, self.leaves);
            }
          }
        }
      }
    }
  }
}
