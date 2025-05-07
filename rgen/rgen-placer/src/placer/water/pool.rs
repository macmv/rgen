use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

macro_rules! bool {
  (w) => {
    0
  };
  (b) => {
    1
  };
  (.) => {
    2
  };
}
macro_rules! bools {
  ($($x:tt)*) => {
    [$( bool!($x) ),*]
  };
}
pub struct Pool {
  pub border_types:  BlockFilter,
  pub avg_per_chunk: f64,
  pub moss:          BlockState,
  pub moss_carpet:   BlockState,
  pub temp_filer:    BlockState,
  pub stone:         BlockState,
  pub clay:          BlockState,
}

impl Pool {
  pub fn new() -> Self {
    Pool {
      border_types:  [block![stone], block![dirt], block![rgen:mossy_cobblestone_rgen]].into(),
      avg_per_chunk: 12.0,
      moss:          block![rgen:mossy_block],
      moss_carpet:   block![rgen:mossy_carpet],
      temp_filer:    block![concrete[12]],
      stone:         block![stone],
      clay:          block![clay],
    }
  }
}

impl Placer for Pool {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    // w = Water
    // b = Border
    // . = Neither
    let pool_types = [
      [bools!(. b . .), bools!(b w b .), bools!(. b w b), bools!(. . b .)],
      [bools!(. . b .), bools!(. b w b), bools!(b w w b), bools!(. b b .)],
      [bools!(. b . .), bools!(b w b .), bools!(b w w b), bools!(. b b .)],
      [bools!(. b . .), bools!(b w b .), bools!(b w b .), bools!(. b . .)],
      [bools!(. b b .), bools!(b w w b), bools!(b w w b), bools!(. b b .)],
      [bools!(. b . .), bools!(b w b .), bools!(. b . .), bools!(. . . .)],
      [bools!(. b b .), bools!(b w w b), bools!(. b b .), bools!(. b . .)],
    ];

    if pos.y + 20 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    let level_pos = pos + Pos::new(0, -1, 0);

    // Check to see is on surface
    let pool_map = rng.choose(&pool_types);
    for (x, row) in pool_map.iter().enumerate() {
      for (z, cell) in row.iter().enumerate() {
        //
        if *cell == 0 {
          if world.get(level_pos + Pos::new(x as i32, 1, z as i32)) != BlockState::AIR {
            return Err(UndoError);
          }
          if !self.border_types.contains(world.get(level_pos + Pos::new(x as i32, -1, z as i32))) {
            return Err(UndoError);
          }
          //water
        } else if *cell == 1 {
          //land
          if !self.border_types.contains(world.get(level_pos + Pos::new(x as i32, 0, z as i32))) {
            return Err(UndoError);
          }
          if !self.border_types.contains(world.get(level_pos + Pos::new(x as i32, -1, z as i32))) {
            return Err(UndoError);
          }
          if world.get(level_pos + Pos::new(x as i32, 1, z as i32)) != BlockState::AIR
            && rng.range(0..=8) == 0
          {
            return Err(UndoError);
          }
        }
      }
    }

    // Build pool
    // world.set(level_pos, self.temp_filer);

    for (x, row) in pool_map.iter().enumerate() {
      for (z, cell) in row.iter().enumerate() {
        //
        if *cell == 0 {
          world.set(level_pos + Pos::new(x as i32, 0, z as i32), block![water]);
          if rng.range(0..=1) == 0 {
            world.set(level_pos + Pos::new(x as i32, -1, z as i32), self.clay);
          }
          //water
        } else if *cell == 1 {
          if rng.range(0..=5) == 0 {
            world.set(level_pos + Pos::new(x as i32, 0, z as i32), self.moss);
            if rng.range(0..=2) == 0
              && world.get(level_pos + Pos::new(x as i32, 1, z as i32)) == BlockState::AIR
            {
              world.set(level_pos + Pos::new(x as i32, 1, z as i32), self.moss_carpet);
            }
          } else {
            world.set(level_pos + Pos::new(x as i32, 0, z as i32), self.stone);
          }
          if rng.range(0..=8) == 0
            && world.get(level_pos + Pos::new(x as i32, 1, z as i32)) == BlockState::AIR
          {
            world.set(level_pos + Pos::new(x as i32, 1, z as i32), self.moss_carpet);
          }
          //border
        } else if *cell == 2 {
          //neither
        }
      }
    }

    Ok(())
  }
}

impl Pool {}
