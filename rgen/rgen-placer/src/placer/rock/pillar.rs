use rgen_base::{block, BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct Pillar {
  pub ground:                  BlockFilter,
  pub material:                BlockState,
  pub avg_in_chunk:            f64,
  pub fluid:                   BlockState,
  chance_of_secondary_pillars: i32,
}

impl Pillar {
  pub fn new() -> Self {
    Pillar {
      ground:       [block![stone], block![dirt], block![grass]].into(),
      material:     block![rgen:basalt[0]],
      avg_in_chunk: 0.8,
      fluid:        block![lava],

      chance_of_secondary_pillars: 11,
    }
  }
}

impl Placer for Pillar {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    self.build_base(rng, pos + Pos::new(0, 0, 0), world);
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if rng.rand_inclusive(0, self.chance_of_secondary_pillars) == 0 {
          self.build_base(rng, pos + Pos::new(rel_x, 0, rel_z), world);
        }
      }
    }

    Ok(())
  }
}

impl Pillar {
  fn build_base(&self, rng: &mut Rng, pos: Pos, world: &mut PartialWorld) {
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        let min;
        let max;
        if rel_x == 0 && rel_z == 0 {
          min = 4;
          max = 6;
        } else if (rel_x == 0 && (rel_z == 1 || rel_z == -1))
          || (rel_z == 0 && (rel_x == 1 || rel_x == -1))
        {
          min = 2;
          max = 4;
        } else {
          min = 0;
          max = 2;
        }

        for pillar_height in -1..rng.rand_inclusive(min, max) {
          world.set(pos + Pos::new(rel_x, pillar_height, rel_z), self.material);
        }
      }
    }
  }
}
