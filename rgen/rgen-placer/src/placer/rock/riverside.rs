use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_llama::Structure;
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};
const GRID_SIZE: usize = 40;
const BLOB_SIZE: usize = 20;

pub struct RiverSide {
  pub ground:       BlockFilter,
  pub material:     Vec<BlockState>,
  pub avg_in_chunk: f64,
  pub fluid:        BlockState,
}

impl RiverSide {
  pub fn new(blocks: &Blocks) -> Self {
    RiverSide {
      ground:       [blocks.dirt.block, blocks.grass.block].into(),
      material:     vec![
        blocks.gravel.default_state,
        blocks.gravel.default_state,
        blocks.rgen_mossy_cobblestone.default_state,
        blocks.cobblestone.default_state,
      ],
      avg_in_chunk: 0.1,
      fluid:        blocks.lava.default_state.into(),
    }
  }
}

impl Placer for RiverSide {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return;
    }
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        self.build_siding(rng, pos + Pos::new(rel_x, 0, rel_z), world);
      }
    }
  }
}

impl RiverSide {
  fn build_siding(&self, rng: &mut Rng, pos: Pos, world: &mut PartialWorld) {
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if !(rel_x == 0 && rel_z == 0) {
          let rel_pos = pos + Pos::new(rel_x, 0, rel_z);
          if world.get(rel_pos).block == Block::WATER && self.ground.contains(world.get(pos)) {
            world.set(pos, *rng.choose(&self.material));
          }
        }
      }
    }
  }
}
