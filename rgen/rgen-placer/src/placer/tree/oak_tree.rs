use rgen_base::{block, BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

#[derive(PartialEq, Debug, Clone, Copy)]
enum SplitTree {
  Simple,
  Square,
  Big,
}
pub struct OakTree {
  pub place_above:  BlockFilter,
  pub trunk:        BlockState,
  pub leaves:       BlockState,
  pub avg_in_chunk: f64,
}

impl OakTree {
  pub fn new() -> Self {
    OakTree {
      avg_in_chunk: 6.5, //40.0,
      place_above:  block![grass].into(),
      trunk:        block![log[0]],
      leaves:       block![leaves[0]],
    }
  }
}

impl Placer for OakTree {
  fn radius(&self) -> u8 { 3 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let mut tree_choices = vec![SplitTree::Simple, SplitTree::Square, SplitTree::Big];
    for _ in 0..10 {
      tree_choices.push(SplitTree::Simple);
    }

    let tree_choice = rng.choose(&tree_choices);

    match tree_choice {
      SplitTree::Simple => self.build_simple(world, pos, rng),
      SplitTree::Square => self.build_simple(world, pos, rng),
      SplitTree::Big => self.build_simple(world, pos, rng),
    }
  }
}

impl OakTree {
  fn build_simple(&self, world: &mut PartialWorld, pos: Pos, rng: &mut Rng) -> Result {
    let height = rng.rand_inclusive(3, 5);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }

    // Builds the main body.
    for y in -2..=-1_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if (x.abs() == 2 && z.abs() == 2) && rng.rand_inclusive(0, 1) == 0 {
            continue;
          }

          let pos = pos + Pos::new(x, y + height, z);
          if world.get(pos) == BlockState::AIR {
            world.set(pos, self.leaves);
          }
        }
      }
    }

    // Builds the peak.
    for y in 0..=1_i32 {
      for x in -1..=1_i32 {
        for z in -1..=1_i32 {
          // Remove the corners.
          if x.abs() == 1 && z.abs() == 1 && y == 1 {
            continue; // next loop
          }

          let pos = pos + Pos::new(x, y + height, z);
          if world.get(pos) == BlockState::AIR {
            world.set(pos, self.leaves);
          }
        }
      }
    }

    // Builds the trunk.
    for y in 0..=height {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }

    Ok(())
  }
}
