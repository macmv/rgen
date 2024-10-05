use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct JungleTree {
  place_above:  BlockFilter,
  trunk:        BlockState,
  leaves:       BlockState,
  avg_in_chunk: f64,
}

impl JungleTree {
  pub fn new(blocks: &Blocks) -> Self {
    Self {
      avg_in_chunk: 8.0,
      place_above:  [blocks.grass.block].into(),
      trunk:        blocks.log.with_data(3),
      leaves:       blocks.leaves.with_data(3),
    }
  }
}

impl Placer for JungleTree {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(15, 20);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return;
    }

    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos).block != Block::AIR {
      return;
    }

    // First block of the trunk. This adds a bit of horizontal variation at the
    // start, which is nice to have.
    world.set(pos, self.trunk);

    for _ in 0..rng.rand_inclusive(1, 3) {
      self.place_trunk(world, rng, pos, height);
    }

    for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
      for i in 0..rng.rand_inclusive(1, 3) {
        let pos = pos + Pos::new(dx, i, dz);
        if world.get(pos).block == Block::AIR {
          world.set(pos, self.leaves);
        }
      }
    }
  }
}

impl JungleTree {
  fn place_trunk(&self, world: &mut PartialWorld, rng: &mut Rng, mut pos: Pos, height: i32) {
    let sway_x = rng.rand_inclusive(-80, 80) as f64 / 100.0;
    let sway_z = rng.rand_inclusive(-80, 80) as f64 / 100.0;

    // Trunk.
    let mut x = pos.x as f64;
    let mut z = pos.z as f64;
    let mut next_leaves = rng.rand_inclusive(4, 7);
    for i in 0..height {
      let mut dx = sway_x * rng.rand_inclusive(-10, 20) as f64 / 10.0;
      let mut dz = sway_z * rng.rand_inclusive(-10, 20) as f64 / 10.0;

      while dx.abs() > 1.0 || dz.abs() > 1.0 {
        world.set(pos + Pos::new((x + dx) as i32, 0, (z + dz) as i32), self.trunk);
        dx /= 2.0;
        dz /= 2.0;
      }
      x += dx;
      z += dz;

      // Connect trunks on the edges, not on corners.
      if x as i32 != pos.x || z as i32 != pos.z {
        world.set(pos, self.trunk);
      }

      // Set these, so that leaves at the end go in the right spot.
      pos.x = x as i32;
      pos.z = z as i32;
      world.set(pos, self.trunk);

      pos.y += 1;

      if i == next_leaves {
        self.place_leaves(world, rng, pos);
        next_leaves += rng.rand_inclusive(4, 7);
      }
    }

    self.place_leaves(world, rng, pos);
  }

  fn place_leaves(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let radius = 3_i32;
    for x in -radius..=radius {
      for z in -radius..=radius {
        let dist = x.pow(2) + z.pow(2);

        if dist <= radius.pow(2) {
          let pos = pos + Pos::new(x, 0, z);
          if world.get(pos).block == Block::AIR {
            world.set(pos, self.leaves);
          }
        }
      }
    }

    // Add a couple more leaves down the trunk.
    for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
      for i in 1..rng.rand_inclusive(1, 3) {
        let pos = pos + Pos::new(dx, -i, dz);
        if world.get(pos).block == Block::AIR {
          world.set(pos, self.leaves);
        }
      }
    }
  }
}
