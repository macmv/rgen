use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_llama::Structure;
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

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
  pub border_types:                     BlockFilter,
  pub water_cause_there_is_no_constant: BlockState,
  pub avg_in_chunk:                     f64,
  pub moss:                             BlockState,
  pub moss_carpet:                      BlockState,
}

impl Pool {
  pub fn new(blocks: &Blocks) -> Self {
    Pool {
      border_types:                     [
        blocks.stone.block,
        blocks.dirt.block,
        blocks.rgen_mossy_cobblestone.block,
      ]
      .into(),
      water_cause_there_is_no_constant: blocks.water.default_state,
      avg_in_chunk:                     40.0,
      moss:                             blocks.rgen_moss.default_state,
      moss_carpet:                      blocks.rgen_mossy_carpet.default_state,
    }
  }
}

impl Placer for Pool {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
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
      return;
    }

    //let min_y = rng.rand_inclusive(4, 6);

    // Check to see is on surface
    let pool_options: i32 = rng.rand_exclusive(0, pool_types.len() as i32);

    println!("pool selection: {}", pool_options);

    let pool_map = pool_types[pool_options as usize];
    //let pool_map = rng.choose(pool_types);
    for (x, row) in pool_map.iter().enumerate() {
      for (z, cell) in row.iter().enumerate() {
        //
        if *cell == 0 {
          if world.get(pos + Pos::new(x as i32, 1, z as i32)) != BlockState::AIR {
            return;
          }
          if !self.border_types.contains(world.get(pos + Pos::new(x as i32, -1, z as i32))) {
            return;
          }
          //water
        } else if *cell == 1 {
          //land
          if !self.border_types.contains(world.get(pos + Pos::new(x as i32, 0, z as i32))) {
            return;
          }
          if !self.border_types.contains(world.get(pos + Pos::new(x as i32, -1, z as i32))) {
            return;
          }
        }
      }
    }

    // Build pool
    let pool_map = pool_types[0];
    for (x, row) in pool_map.iter().enumerate() {
      for (z, cell) in row.iter().enumerate() {
        //
        if *cell == 0 {
          world.set(pos + Pos::new(x as i32, 0, z as i32), self.water_cause_there_is_no_constant);
          //water
        } else if *cell == 1 {
          world.set(pos + Pos::new(x as i32, 0, z as i32), self.moss);
          //border
        } else if *cell == 2 {
          //neither
        }
      }
    }
  }
}

impl Pool {}
