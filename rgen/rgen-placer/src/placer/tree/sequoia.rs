use rgen_base::{BlockFilter, BlockState, Pos};
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
  pub place_above:  BlockFilter,
  pub trunk:        BlockState,
  pub leaves:       BlockState,
  pub avg_in_chunk: f64,
}

impl Placer for Sequoia {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    if !self.place_above.contains(world.get(pos + Pos::new(0, -1, 0))) {
      return;
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
    for (x, row) in LEVEL_II.iter().enumerate() {
      for (z, cell) in row.iter().enumerate() {
        self.place_leaf_slice(world, rng, pos, *cell, x as i32 - 2, height, z as i32 - 2)
      }
      self.place_trunk_slice(world, pos, height);
    }

    // Creates the grade 'A's
    for _ in 1..rng.rand_inclusive(2, 3) {
      height += 1;
      for (x, row) in LEVEL_A.iter().enumerate() {
        for (z, cell) in row.iter().enumerate() {
          self.place_leaf_slice(world, rng, pos, *cell, x as i32 - 4, height, z as i32 - 4)
        }
      }
      self.place_wide_trunk_slice(world, pos, height);

      height += 1;
      for (x, row) in LEVEL_I.iter().enumerate() {
        for (z, cell) in row.iter().enumerate() {
          self.place_leaf_slice(world, rng, pos, *cell, x as i32 - 1, height, z as i32 - 1)
        }
      }
      self.place_trunk_slice(world, pos, height);
    }

    // Grade 'B's
    for _ in 1..rng.rand_inclusive(3, 4) {
      height += 1;
      for (x, row) in LEVEL_B.iter().enumerate() {
        for (z, cell) in row.iter().enumerate() {
          self.place_leaf_slice(world, rng, pos, *cell, x as i32 - 3, height, z as i32 - 3)
        }
      }
      self.place_trunk_slice(world, pos, height);

      height += 1;
      for (x, row) in LEVEL_I.iter().enumerate() {
        for (z, cell) in row.iter().enumerate() {
          self.place_leaf_slice(world, rng, pos, *cell, x as i32 - 1, height, z as i32 - 1)
        }
      }
      self.place_trunk_slice(world, pos, height);
    }

    // Grade 'C's
    for _ in 1..rng.rand_inclusive(2, 3) {
      height += 1;
      for (x, row) in LEVEL_C.iter().enumerate() {
        for (z, cell) in row.iter().enumerate() {
          self.place_leaf_slice(world, rng, pos, *cell, x as i32 - 3, height, z as i32 - 3)
        }
      }
      self.place_trunk_slice(world, pos, height);

      height += 1;
      for (x, row) in LEVEL_I.iter().enumerate() {
        for (z, cell) in row.iter().enumerate() {
          self.place_leaf_slice(world, rng, pos, *cell, x as i32 - 1, height, z as i32 - 1)
        }
      }
      self.place_trunk_slice(world, pos, height);
    }

    // Crown.
    height += 1;
    for (x, row) in LEVEL_II.iter().enumerate() {
      for (z, cell) in row.iter().enumerate() {
        self.place_leaf_slice(world, rng, pos, *cell, x as i32 - 2, height, z as i32 - 2)
      }
    }
    self.place_trunk_slice(world, pos, height);

    height += 1;
    for (x, row) in LEVEL_I.iter().enumerate() {
      for (z, cell) in row.iter().enumerate() {
        self.place_leaf_slice(world, rng, pos, *cell, x as i32 - 1, height, z as i32 - 1)
      }
    }
    self.place_trunk_slice(world, pos, height);

    // Pointy top.
    for rel_x in 0..=1_i32 {
      for rel_z in 0..=1_i32 {
        world.set(pos + Pos::new(rel_x, height, rel_z), self.leaves);
        world.set(pos + Pos::new(rel_x, height + 1, rel_z), self.leaves);
      }
    }
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

  fn place_leaf_slice(
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
