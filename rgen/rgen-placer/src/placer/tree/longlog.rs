use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct LongLog {
  pub log:           BlockState,
  pub ground:        BlockFilter,
  pub avg_per_chunk: f64,
}

impl Placer for LongLog {
  //log_moss
  //chance_of_moss
  //plants

  fn radius(&self) -> u8 { 9 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let pos = pos + Pos::new(rng.rand_inclusive(-4, 4), 0, rng.rand_inclusive(-4, 4));

    // Checks to make sure is in open space from other woods
    for rel_x in -2..=2_i32 {
      for rel_z in -2..=2_i32 {
        if world.get(pos + Pos::new(rel_x, 0, rel_z)).block == self.log.block {
          return;
        }
        //world.set(pos + Pos::new(rel_x, 0, rel_z), self.log);
      }
    }

    // Checks to make sure is in open space
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if self.ground.contains(world.get(pos + Pos::new(rel_x, 0, rel_z))) {
          return;
        }
      }
    }

    // Checks if on ground
    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.ground.contains(world.get(below_pos)) {
      return;
    }

    // Builds log
    let mut dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    rng.shuffle(&mut dirs);

    let Some((dx, dz, length)) = dirs.iter().find_map(|&(dx, dz)| {
      let length = rng.rand_inclusive(4, 5);
      if self.is_buildable(world, pos, dx, dz, length) {
        Some((dx, dz, length))
      } else {
        None
      }
    }) else {
      return;
    };

    for i in 2..=length {
      let i_pos = pos + Pos::new(i * dx, 0, i * dz);

      let mut log_type = self.log;

      log_type.state &= 0b0011; //reset

      if dx != 0 {
        // x axis be it (5, 6)
        log_type.state |= 0b0100;
      } else {
        // z axis be it (9, 10)
        log_type.state |= 0b1000;
      }

      world.set(i_pos, log_type);
    }

    world.set(pos, self.log);
  }
}

impl LongLog {
  fn is_buildable(&self, world: &PartialWorld, pos: Pos, dx: i32, dz: i32, length: i32) -> bool {
    let pos_st = pos + Pos::new(dx * 2, -1, dz * 2);
    let pos_nd = pos + Pos::new(dx * length, -1, dz * length);
    if !self.ground.contains(world.get(pos_st)) || !self.ground.contains(world.get(pos_nd)) {
      return false;
    }

    for i in 1..=length {
      let i_pos = pos + Pos::new(i * dx, 0, i * dz);
      if world.get(i_pos) != BlockState::AIR {
        return false;
      }
    }

    true
  }
}
