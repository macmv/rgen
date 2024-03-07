use rgen_base::{Block, BlockSet, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct LogAndStump {
  pub log:            BlockState,
  pub moss_log:       BlockState,
  pub plants:         BlockSet,
  pub avg_per_chunk:  f64,
  pub chance_of_moss: i32,
}

impl Placer for LogAndStump {
  //log_moss
  //chance_of_moss
  //plants

  fn radius(&self) -> u8 { 9 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    if (self.place_stump(world, rng, pos)) {
      self.place_log()
    }

    //call log fn
  }
}

impl LogAndStump {
  fn lenght(&self, rng: &mut Rng) -> u8 { rng.rand_inclusive(2, 5) as u8 }
  fn place_stump(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> bool {
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if world.get(pos + Pos::new(rel_x, 1, rel_z)) != BlockState::AIR {
          //println!("false  =========================== {pos:?}");
          return false;
        }
      }
    }
    if rng.rand_exclusive(0, 100) > self.chance_of_moss {
      world.set(pos + Pos::new(0, 1, 0), self.log);
    } else {
      world.set(pos + Pos::new(0, 1, 0), self.moss_log);
    }

    return true;
  }
  fn place_log(&self) {}
}
