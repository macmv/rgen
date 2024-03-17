use rgen_base::{Block, BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct Sakura {
  pub place_above:  BlockFilter,
  pub trunk:        BlockState,
  pub leaves:       BlockState,
  pub avg_in_chunk: f64,
  pub large_size:   bool,
}

impl Placer for Sakura {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(5, 8);

    // Checks if tree will breach build height
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return;
    }

    // Checks if tree will be built on air
    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos).block != Block::AIR {
      return;
    }

    // Builds the trunk.
    for y in 0..=height {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }

    self.build_cannopy(world, pos + Pos::new(0, height, 0), rng);
  }
}

impl Sakura {
  fn build_cannopy(&self, world: &mut PartialWorld, pos: Pos, rng: &mut Rng) {
    // Leaf box 2
    for rel_y in -1..=2_i32 {
      for rel_x in -2..=2_i32 {
        for rel_z in -2..=2_i32 {
          if world.get(pos + Pos::new(rel_x, rel_y, rel_z)) == BlockState::AIR {
            world.set(pos + Pos::new(rel_x, rel_y, rel_z), self.leaves);
          }
        }
      }
    }

    // Leaf rim
    for &a in [-3, 3].iter() {
      for b in -2..=2_i32 {
        let rel_x = a;
        let rel_z = b;
        self.build_drape(world, pos + Pos::new(rel_x, 1, rel_z), rng);

        let rel_x = b;
        let rel_z = a;
        self.build_drape(world, pos + Pos::new(rel_x, 1, rel_z), rng);
      }
    }

    // Crown
    for x in -1..=1_i32 {
      for z in -1..=1_i32 {
        // Remove the corners.
        if (x.abs() == 1 && z.abs() == 1) && !(rng.rand_inclusive(0, 4) == 0) {
          continue;
        }

        if world.get(pos + Pos::new(x, 3, z)) == BlockState::AIR {
          world.set(pos + Pos::new(x, 3, z), self.leaves);
        }
      }
    }
  }

  fn build_drape(&self, world: &mut PartialWorld, pos: Pos, rng: &mut Rng) {
    let mut low = [0, 0, 0, 0, 1, 1, 2, 3];
    rng.shuffle(&mut low);

    for rel_y in 0..=low[0] {
      if world.get(pos + Pos::new(0, rel_y * -1, 0)) == BlockState::AIR {
        world.set(pos + Pos::new(0, rel_y * -1, 0), self.leaves);
      }
    }
  }
}
