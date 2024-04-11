use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_llama::Structure;
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

macro_rules! bool {
  (x) => {
    true
  };
  (.) => {
    false
  };
}
macro_rules! bools {
  ($($x:tt)*) => {
    [$( bool!($x) ),*]
  };
}
pub struct MossBoulder {
  pub place_above:      BlockFilter,
  pub phobic:           BlockFilter,
  pub material:         BlockState,
  pub avg_in_chunk:     f64,
  pub plants:           BlockFilter,
  pub use_large_plants: bool,
  pub large_plants:     BlockFilter,
}

impl MossBoulder {
  pub fn new(blocks: &Blocks) -> Self {
    MossBoulder {
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

impl Placer for MossBoulder {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    // Checks if tree will breach build height
    let mut bolder_map = [
      [bools!(. . .), bools!(. . .), bools!(. . .)],
      [bools!(. . .), bools!(. . .), bools!(. . .)],
      [bools!(. . .), bools!(. . .), bools!(. . .)],
    ];

    if pos.y + 20 >= 255 || pos.y <= 1 {
      return;
    }

    // Checks if tree will be built on air
    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos).block != Block::AIR {
      return;
    }

    //let min_y = rng.rand_inclusive(4, 6);
    for (y, col) in bolder_map.iter_mut().enumerate() {
      for (x, row) in col.iter_mut().enumerate() {
        for (z, cell) in row.iter_mut().enumerate() {
          self.build_bolder(world, pos, *cell, x as i32 - 1, y as i32 - 1, z as i32 - 1);
          *cell = true;
        }
      }
    }
  }
}

impl MossBoulder {
  fn build_bolder(
    &self,
    world: &mut PartialWorld,
    pos: Pos,
    cell: bool,
    x: i32,
    height: i32,
    z: i32,
  ) {
    let rel_pos = pos + Pos::new(x as i32, height, z as i32);
    if cell {
      if world.get(rel_pos) == BlockState::AIR {
        world.set(rel_pos, self.material);
      }
    }
  }
}
