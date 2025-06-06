use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct JungleTree {
  place_above: BlockFilter,
  trunk:       BlockState,
  leaves:      BlockState,
  cocoa:       BlockState,

  avg_per_chunk: f64,
}

impl Default for JungleTree {
  fn default() -> Self {
    Self {
      avg_per_chunk: 8.0,
      place_above:   block![grass].into(),
      trunk:         block![log[3]],
      leaves:        block![leaves[3]],
      cocoa:         block![cocoa[0]],
    }
  }
}

impl Placer for JungleTree {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let height = rng.range(15..=20);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }

    // First block of the trunk. This adds a bit of horizontal variation at the
    // start, which is nice to have.
    world.set(pos, self.trunk);

    for _ in 0..rng.range(1..=3) {
      self.place_trunk(world, rng, pos, height);
    }

    for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
      for i in 0..rng.range(1..=3) {
        let pos = pos + Pos::new(dx, i, dz);
        if world.get(pos) == block![air] {
          world.set(pos, self.leaves);
        }
      }
    }

    Ok(())
  }
}

impl JungleTree {
  fn place_trunk(&self, world: &mut PartialWorld, rng: &mut Rng, mut pos: Pos, height: i32) {
    let sway_x = rng.range(-0.8..=0.8);
    let sway_z = rng.range(-0.8..=0.8);

    // Trunk.
    let mut x = pos.x as f64;
    let mut z = pos.z as f64;
    let mut next_leaves = rng.range(4..=7);
    for i in 0..height {
      let mut dx = sway_x * rng.range(-1.0..=2.0);
      let mut dz = sway_z * rng.range(-1.0..=2.0);

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
        next_leaves += rng.range(4..=7);
      }

      self.place_cocoa_beans(world, rng, pos);
    }

    self.place_leaves(world, rng, pos);
    self.place_top_leaves(world, rng, pos + Pos::new(0, 1, 0));
  }

  fn place_leaves(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let radius = 3_i32;
    for x in -radius..=radius {
      for z in -radius..=radius {
        let dist = x.pow(2) + z.pow(2);

        if dist <= radius.pow(2) {
          let pos = pos + Pos::new(x, 0, z);
          if world.get(pos) == block![air] {
            world.set(pos, self.leaves);
          }
        }
      }
    }

    // Add a couple more leaves down the trunk.
    for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
      for i in 1..rng.range(1..=3) {
        let pos = pos + Pos::new(dx, -i, dz);
        if world.get(pos) == block![air] {
          world.set(pos, self.leaves);
        }
      }
    }
  }

  fn place_top_leaves(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    for x in -1..=1_i32 {
      for z in -1..=1_i32 {
        if (x.abs() + z.abs()) >= 2 && rng.range(0..=1) == 0 {
          continue;
        }

        let pos = pos + Pos::new(x, 0, z);
        if world.get(pos) == block![air] {
          world.set(pos, self.leaves);
        }
      }
    }
  }

  fn place_cocoa_beans(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    if rng.range(0..16) != 0 {
      return;
    }

    let mut dirs = [
      (Pos::new(1, 0, 0), 1),  // East, so place facing west.
      (Pos::new(-1, 0, 0), 3), // West, so place facing east.
      (Pos::new(0, 0, 1), 2),  // South, so place facing north.
      (Pos::new(0, 0, -1), 0), // North, so place facing south.
    ];
    rng.shuffle(&mut dirs);

    for (dir, data) in dirs {
      if world.get(pos + dir) == block![air] {
        // Place fully grown.
        world.set(pos + dir, self.cocoa.with_data(data | 8));
        return;
      }
    }
  }
}
