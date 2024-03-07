use rgen_base::{Block, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct BasicBirch {
  pub trunk:            BlockState,
  pub leaves:           BlockState,
  pub avg_per_chunk:    f64,
  pub is_shrooms:       bool,
  pub chance_of_shroom: f64,
  pub shroom:           BlockState,
  pub ground:           BlockState,
}

impl Placer for BasicBirch {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(7, 8);
    let min_y = rng.rand_inclusive(-2, -1);

    // Checks if outside world boundry
    if pos.y as i32 + height as i32 + 2 >= 255 || pos.y <= 1 {
      return;
    }

    // Checks to make sure is in open space
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if world.get(pos + Pos::new(rel_x, 1, rel_z)) != BlockState::AIR {
          return;
        }
      }
    }

    // Checks if on ground
    if world.get(pos) != self.ground {
      return;
    }

    // Builds tree
    for y in min_y..=2_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if rng.rand_inclusive(0, 4) == 1 && x.abs() == 2 && z.abs() == 2 {
            continue;
          }

          // Make the top layer smaller.
          if (y == 1 || y == 2) && (x.abs() == 2 || z.abs() == 2) {
            continue;
          }
          world.set(pos + Pos::new(x, (y + height) as u8, z), self.leaves);
        }
      }
    }

    // Builds trunk
    for y in 0..height as u8 {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }

    // Builds polypores
    if self.is_shrooms {}
  }
}
