use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

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

const LEVEL_A: [[bool; 10]; 10] = [
  bools!(. . . . x x . . . .),
  bools!(. . . x x x x . . .),
  bools!(. . x x x x x x . .),
  bools!(. x x x . . x x x .),
  bools!(x x x . . . . x x x),
  bools!(x x x . . . . x x x),
  bools!(. x x x . . x x x .),
  bools!(. . x x x x x x . .),
  bools!(. . . x x x x . . .),
  bools!(. . . . x x . . . .),
];

const LEVEL_B: [[bool; 8]; 8] = [
  bools!(. . . x x . . .),
  bools!(. x x x x x x .),
  bools!(. x x x x x x .),
  bools!(x x x . . x x x),
  bools!(x x x . . x x x),
  bools!(. x x x x x x .),
  bools!(. x x x x x x .),
  bools!(. . . x x . . .),
];

const LEVEL_C: [[bool; 8]; 8] = [
  bools!(. . . x x . . .),
  bools!(. . x x x x . .),
  bools!(. x x x x x x .),
  bools!(x x x . . x x x),
  bools!(x x x . . x x x),
  bools!(. x x x x x x .),
  bools!(. . x x x x . .),
  bools!(. . . x x . . .),
];

const LEVEL_I: [[bool; 4]; 4] =
  [bools!(. x x .), bools!(x . . x), bools!(x . . x), bools!(. x x .)];

const LEVEL_II: [[bool; 6]; 6] = [
  bools!(. . x x . .),
  bools!(. x x x x .),
  bools!(x x . . x x),
  bools!(x x . . x x),
  bools!(. x x x x .),
  bools!(. . x x . .),
];

pub struct Sequoia {
  pub place_above:   BlockFilter,
  pub trunk:         BlockState,
  pub leaves:        BlockState,
  pub avg_per_chunk: f64,
}

impl Sequoia {
  pub fn new() -> Self {
    Sequoia {
      avg_per_chunk: 3_f64,
      place_above:   block![grass].into(),
      leaves:        block![rgen:leaves[3]],
      trunk:         block![rgen:log[3]],
    }
  }
}

impl Placer for Sequoia {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if !self.place_above.contains(world.get(pos + Pos::new(0, -1, 0))) {
      return Err(UndoError);
    }

    // Creates lower trunk.
    let min_y = rng.rand_inclusive(4, 6);
    for rel_y in -2..min_y {
      for rel_x in 0..=1_i32 {
        for rel_z in 0..=1_i32 {
          if world.get(pos + Pos::new(rel_x, rel_y, rel_z)) == BlockState::AIR {
            world.set(pos + Pos::new(rel_x, rel_y, rel_z), self.trunk);
          }
        }
      }
    }

    // Create bottom rim.
    let mut height = min_y;
    self.place_leaf_slice(world, pos, LEVEL_II, height);
    self.place_trunk_slice(world, pos, height);

    // Creates the grade 'A's
    for _ in 1..rng.rand_inclusive(2, 3) {
      height += 1;
      self.place_leaf_slice(world, pos, LEVEL_A, height);
      self.place_wide_trunk_slice(world, pos, height);

      height += 1;
      self.place_leaf_slice(world, pos, LEVEL_I, height);
      self.place_trunk_slice(world, pos, height);
    }

    // Grade 'B's
    for _ in 1..rng.rand_inclusive(3, 4) {
      height += 1;
      self.place_leaf_slice(world, pos, LEVEL_B, height);
      self.place_trunk_slice(world, pos, height);

      height += 1;
      self.place_leaf_slice(world, pos, LEVEL_I, height);
      self.place_trunk_slice(world, pos, height);
    }

    // Grade 'C's
    for _ in 1..rng.rand_inclusive(2, 3) {
      height += 1;
      self.place_leaf_slice(world, pos, LEVEL_C, height);
      self.place_trunk_slice(world, pos, height);

      height += 1;
      self.place_leaf_slice(world, pos, LEVEL_I, height);
      self.place_trunk_slice(world, pos, height);
    }

    // Crown.
    height += 1;
    self.place_leaf_slice(world, pos, LEVEL_II, height);
    self.place_trunk_slice(world, pos, height);

    height += 1;
    self.place_leaf_slice(world, pos, LEVEL_I, height);
    self.place_trunk_slice(world, pos, height);

    // Pointy top.
    for rel_x in 0..=1_i32 {
      for rel_z in 0..=1_i32 {
        world.set(pos + Pos::new(rel_x, height, rel_z), self.leaves);
        world.set(pos + Pos::new(rel_x, height + 1, rel_z), self.leaves);
      }
    }

    Ok(())
  }
}

impl Sequoia {
  fn place_wide_trunk_slice(&self, world: &mut PartialWorld, pos: Pos, height: i32) {
    for rel_x in -1..=2_i32 {
      for rel_z in -1..=2_i32 {
        if rel_x.abs() == rel_z.abs() {
          continue;
        }
        if world.get(pos + Pos::new(rel_x, height, rel_z)) == BlockState::AIR {
          world.set(pos + Pos::new(rel_x, height, rel_z), self.trunk);
        }
      }
    }
  }

  fn place_trunk_slice(&self, world: &mut PartialWorld, pos: Pos, height: i32) {
    for rel_x in 0..=1_i32 {
      for rel_z in 0..=1_i32 {
        if world.get(pos + Pos::new(rel_x, height, rel_z)) == BlockState::AIR {
          world.set(pos + Pos::new(rel_x, height, rel_z), self.trunk);
        }
      }
    }
  }

  fn place_leaf_slice<const N: usize, const M: usize>(
    &self,
    world: &mut PartialWorld,
    pos: Pos,
    cells: [[bool; M]; N],
    height: i32,
  ) {
    for (x, row) in cells.iter().enumerate() {
      for (z, cell) in row.iter().enumerate() {
        let rel_pos =
          pos + Pos::new(x as i32 - N as i32 / 2 + 1, height, z as i32 - M as i32 / 2 + 1);
        if *cell && world.get(rel_pos) == BlockState::AIR {
          world.set(rel_pos, self.leaves);
        }
      }
    }
  }
}
