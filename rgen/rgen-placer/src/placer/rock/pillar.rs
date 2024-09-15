use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_llama::Structure;
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct Pillar {
  pub ground:       BlockFilter,
  pub material:     BlockState,
  pub avg_in_chunk: f64,
  pub fluid:        BlockState,
}

impl Pillar {
  pub fn new(blocks: &Blocks) -> Self {
    Pillar {
      ground:       [blocks.stone.block, blocks.dirt.block, blocks.grass.block].into(),
      material:     blocks.rgen_basalt.with_data(0),
      avg_in_chunk: 0.1,
      fluid:        blocks.lava.default_state.into(),
    }
  }
}

impl Placer for Pillar {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return;
    }
    self.build_base(rng, pos + Pos::new(0, 0, 0), world);
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if rng.rand_inclusive(0, 60) == 0 {
          self.build_base(rng, pos + Pos::new(rel_x, 0, rel_z), world);
        }
      }
    }
  }
}

impl Pillar {
  fn build_base(&self, rng: &mut Rng, pos: Pos, world: &mut PartialWorld) {
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        let mut min = 0;
        let mut max = 0;
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
