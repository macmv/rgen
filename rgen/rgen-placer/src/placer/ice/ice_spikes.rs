use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct IceSpikes {
  pub ground:                  BlockFilter,
  pub material:                BlockState,
  pub avg_in_chunk:            f64,
  pub fluid:                   BlockState,
  pub replacables:             BlockFilter,
  chance_of_secondary_pillars: i32,
}

impl IceSpikes {
  pub fn new(blocks: &Blocks) -> Self {
    IceSpikes {
      ground:                      [blocks.stone.block, blocks.dirt.block, blocks.grass.block]
        .into(),
      material:                    blocks.packed_ice.default_state.into(),
      avg_in_chunk:                0.8,
      fluid:                       blocks.lava.default_state.into(),
      chance_of_secondary_pillars: 3,
      replacables:                 [
        Block::AIR,
        blocks.snow_layer.block,
        blocks.snow.block,
        blocks.ice.block,
        blocks.packed_ice.block,
        blocks.water.block,
      ]
      .into(),
    }
  }
}

impl Placer for IceSpikes {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return;
    }
    self.build_base(rng, pos + Pos::new(0, 0, 0), world);
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if rng.rand_inclusive(0, self.chance_of_secondary_pillars) == 0 {
          self.build_base(rng, pos + Pos::new(rel_x, 0, rel_z), world);
        }
      }
    }
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

        for pillar_height in -1..rng.rand_inclusive(min, max) {
          world.set(pos + Pos::new(rel_x, pillar_height, rel_z), self.material);
        }
        self.ground_placement(rng, pos + Pos::new(rel_x, 0, rel_z), world);
      }
    }
  }
  fn ground_placement(&self, _rng: &mut Rng, pos: Pos, world: &mut PartialWorld) {
    for neg_y in 1..8 {
      if self.replacables.contains(world.get(pos + Pos::new(0, neg_y * -1, 0))) {
        world.set(pos + Pos::new(0, neg_y * -1, 0), self.material)
      }
    }
  }
}
