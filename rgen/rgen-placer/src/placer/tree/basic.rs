use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};
use crate::placer::wood_clearance::any_wood_adjacent;

pub struct BasicTree {
  pub place_above:   BlockFilter,
  pub trunk:         BlockState,
  pub leaves:        BlockState,
  pub avg_per_chunk: f64,
  /// Number of log blocks, excluding the leaf above the trunk. Minimum three.
  pub trunk_height: RangeInclusive<i32>,
}

impl Default for BasicTree {
  fn default() -> Self {
    BasicTree {
      place_above:   block![grass].into(),
      trunk:         block![log[variant = "oak"]],
      leaves:        block![leaves[variant = "oak"]],
      avg_per_chunk: 0.8,
      trunk_height: 6..=9,
    }
  }
}

impl Placer for BasicTree {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if self.trunk_height.is_empty() || *self.trunk_height.start() < 3 {
      return Err(UndoError);
    }
    let height = rng.range(self.trunk_height.clone()) - 1;

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }
    if self.trunk == block![log[variant = "oak", axis = "y"]]
      && (trunk_in_3x3(world, pos, self.trunk) || boulder_in_3x3(world, pos)) {
      return Err(UndoError);
    }
    if (0..=height).any(|y| any_wood_adjacent(world, pos + Pos::new(0, y, 0))) {
      return Err(UndoError);
    }

    // Builds the main body.
    for y in -2..=-1_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if (x.abs() == 2 && z.abs() == 2) && rng.range(0..=1) == 0 {
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

pub(super) fn trunk_in_3x3(world: &PartialWorld, pos: Pos, trunk: BlockState) -> bool {
  for x in -1..=1 {
    for z in -1..=1 {
      for y in -2..=8 {
        if world.get(pos + Pos::new(x, y, z)) == trunk {
          return true;
        }
      }
    }
  }
  false
}

pub(super) fn boulder_in_3x3(world: &PartialWorld, pos: Pos) -> bool {
  for x in -1..=1 {
    for z in -1..=1 {
      for y in -1..=2 {
        let block = world.get(pos + Pos::new(x, y, z)).block_kind();
        if block == block![mossy_cobblestone].block
          || block == block![rgen:mossy_cobblestone_rgen].block
          || block == block![rgen:mossy_stone].block
          || block == block![stone_slab].block
          || block == block![stone_stairs].block {
          return true;
        }
      }
    }
  }
  false
}
