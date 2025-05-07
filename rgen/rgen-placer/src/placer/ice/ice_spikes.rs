use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct IceSpikes {
  pub ground:                  BlockFilter,
  pub material:                BlockState,
  pub avg_per_chunk:           f64,
  pub fluid:                   BlockState,
  pub replacables:             BlockFilter,
  chance_of_secondary_pillars: i32,
}

impl IceSpikes {
  pub fn new() -> Self {
    IceSpikes {
      ground:                      [block![stone], block![dirt], block![grass]].into(),
      material:                    block![packed_ice],
      avg_per_chunk:               0.8,
      fluid:                       block![lava],
      chance_of_secondary_pillars: 3,
      replacables:                 [
        block![air],
        block![snow_layer],
        block![snow],
        block![ice],
        block![packed_ice],
        block![water],
      ]
      .into(),
    }
  }
}

impl Placer for IceSpikes {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    self.build_base(rng, pos + Pos::new(0, 0, 0), world);
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if rng.range(0..=self.chance_of_secondary_pillars) == 0 {
          self.build_base(rng, pos + Pos::new(rel_x, 0, rel_z), world);
        }
      }
    }

    Ok(())
  }
}

impl IceSpikes {
  fn build_base(&self, rng: &mut Rng, pos: Pos, world: &mut PartialWorld) {
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        let min;
        let max;
        if rel_x == 0 && rel_z == 0 {
          min = 8;
          max = 12;
        } else if (rel_x == 0 && (rel_z == 1 || rel_z == -1))
          || (rel_z == 0 && (rel_x == 1 || rel_x == -1))
        {
          min = 7;
          max = 9;
        } else {
          min = 3;
          max = 6;
        }

        for pillar_height in -1..rng.range(min..=max) {
          world.set(pos + Pos::new(rel_x, pillar_height, rel_z), self.material);
        }
        self.ground_placement(rng, pos + Pos::new(rel_x, 0, rel_z), world);
      }
    }
  }
  fn ground_placement(&self, _rng: &mut Rng, pos: Pos, world: &mut PartialWorld) {
    for neg_y in 1..8 {
      if self.replacables.contains(world.get(pos + Pos::new(0, -neg_y, 0))) {
        world.set(pos + Pos::new(0, -neg_y, 0), self.material)
      }
    }
  }
}
