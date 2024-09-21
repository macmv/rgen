use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_llama::Structure;
use rgen_world::PartialWorld;

use crate::{chunk, Placer, Random, Rng};

pub struct BetterTallerSnow {
  pub block:        BlockFilter,
  pub snow:         BlockState,
  pub debug:        BlockState,
  pub avg_in_chunk: f64,
}

impl BetterTallerSnow {
  pub fn new(blocks: &Blocks) -> Self {
    BetterTallerSnow {
      block:        [blocks.snow_layer.block].into(),
      snow:         blocks.snow_layer.default_state,
      debug:        blocks.concrete.with_data(5),
      avg_in_chunk: 1.0,
    }
  }
}

impl Placer for BetterTallerSnow {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return;
    }
    let chunk_pos = pos.chunk();
    for z in 0..16 {
      for x in 0..16 {
        for y in (0..256).rev() {
          let pos = chunk_pos.min_block_pos() + Pos::new(x, y, z);
          if self.block.contains(world.get(pos)) {
            'outer: for rel_x in -1..=1_i32 {
              for rel_z in -1..=1_i32 {
                if !(rel_x == 0 && rel_z == 0) || !(rel_x.abs() == 1 && rel_z.abs() == 1) {
                  let block_check = world.get(pos + Pos::new(rel_x, 0, rel_z));
                  if !self.block.contains(block_check) && block_check.block != Block::AIR {
                    let mut height = world.get(pos).state;
                    height += 5; //rng.rand_inclusive(3, 5) as u8;
                    println!("height: {}", height);
                    let mut level = 0;
                    while height > 7 {
                      height -= 7;
                      world.set(pos + Pos::new(0, level, 0), self.snow.with_data(7));
                      level += 1;
                    }
                    world.set(pos + Pos::new(0, level, 0), self.snow.with_data(height));
                    //rld.set(pos + Pos::new(0, 15, 0), self.debug);
                    break 'outer;
                  }
                }
              }
            }
            break;
          }
        }
      }
    }
  }
}
