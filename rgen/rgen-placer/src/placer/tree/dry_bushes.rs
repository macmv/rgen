use rgen_base::{Block, BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct BasicDryBush {
  pub place_above:  BlockFilter,
  pub trunk:        BlockState,
  pub leaves:       BlockState,
  pub avg_in_chunk: f64,
}

impl Placer for BasicDryBush {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    // Checks if outside world boundry
    if pos.y + 2 + 2 >= 255 || pos.y <= 1 {
      return;
    }

    // Checks if on ground
    if !self.place_above.contains(world.get(pos)) {
      return;
    }

    //creates the core
    world.set(pos + Pos::new(0, 1, 0), self.trunk);

    for y in 1..=2_i32 {
      for x in -1..=1_i32 {
        for z in -1..=1_i32 {
          // Remove the corners.
          if x.abs() == 1 && z.abs() == 1 && y == 2 {
            continue; // next loop
          }
          if world.get(pos + Pos::new(x, y, z)) == BlockState::AIR {
            world.set(pos + Pos::new(x, y, z), self.leaves);
          }
        }
      }
    }
  }
}
