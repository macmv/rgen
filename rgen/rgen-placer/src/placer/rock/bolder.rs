use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_llama::Structure;
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct moss_bolder {
  pub place_above:      BlockFilter,
  pub phobic:           BlockFilter,
  pub material:         BlockState,
  pub avg_in_chunk:     f64,
  pub plants:           BlockFilter,
  pub use_large_plants: bool,
  pub large_plants:     BlockFilter,
}

impl moss_bolder {
  pub fn new(blocks: &Blocks) -> Self {
    moss_bolder {
      place_above:      blocks.stone.default_state.into(),
      phobic:           blocks.grass.default_state.into(),
      material:         blocks.rgen_mossy_cobblestone.default_state.into(),
      avg_in_chunk:     2.0,
      plants:           [blocks.tallgrass.with_data(1), blocks.tallgrass.with_data(2)].into(),
      use_large_plants: false,
      large_plants:     [blocks.double_plant.with_data(3), blocks.double_plant.with_data(2)].into(),
    }
  }
}

impl Placer for moss_bolder {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    // Checks if tree will breach build height
    let bottom_level: [[bool; 3]; 3] = [bools!(. . .), bools!(. . .), bools!(. . .)];
    let mid_level: [[bool; 3]; 3] = [bools!(. . .), bools!(. . .), bools!(. . .)];
    let top_level: [[bool; 3]; 3] = [bools!(. . .), bools!(. . .), bools!(. . .)];
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return;
    }

    // Checks if tree will be built on air
    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos).block != Block::AIR {
      return;
    }
  }
}
impl moss_bolder {
  fn build_bolder(
    &self,
    world: &mut PartialWorld,
    rng: &mut Rng,
    pos: Pos,
    cell: bool,
    x: i32,
    height: i32,
    z: i32,
  ) {
    let rel_pos = pos + Pos::new(x as i32, height, z as i32);
    if cell {
      if world.get(rel_pos) == BlockState::AIR {
        world.set(rel_pos, self.leaves);
      }
    }
  }
}
