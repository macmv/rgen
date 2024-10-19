use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct PalmTree {
  pub place_above:  BlockFilter,
  pub trunk:        BlockState,
  pub leaves:       BlockState,
  pub avg_in_chunk: f64,
}

impl PalmTree {
  pub fn new() -> Self {
    PalmTree {
      avg_in_chunk: 2.0,
      place_above:  block![sand].into(),
      trunk:        block![rgen:log[1]],
      leaves:       block![rgen:leaves[1]],
    }
  }
}

impl Placer for PalmTree {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, mut pos: Pos) -> Result {
    let height = rng.rand_inclusive(8, 13);

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

    let sway_x = rng.rand_inclusive(-10, 10) as f64 / 10.0;
    let sway_z = rng.rand_inclusive(-10, 10) as f64 / 10.0;

    // Trunk.
    let mut x = pos.x as f64;
    let mut z = pos.z as f64;
    for y in 0..height {
      x += sway_x * (1.0 - y as f64 / height as f64);
      z += sway_z * (1.0 - y as f64 / height as f64);

      // Connect trunks on the edges, not on corners.
      if x as i32 != pos.x || z as i32 != pos.z {
        world.set(pos, self.trunk);
      }

      pos.x = x as i32;
      pos.z = z as i32;
      world.set(pos, self.trunk);

      pos.y += 1;
    }

    // `pos` is now the top of the trunk.

    // Leaves.
    for i in 0..5_i32 {
      let y_offset = if i <= 2 { i } else { 4 - i };

      let chance = match i {
        0 => 10,
        1 => 9,
        2 => 7,
        3 => 5,
        4 => 1,
        _ => unreachable!(),
      };

      let radius = i + 1;
      for x in -radius..=radius {
        for z in -radius..=radius {
          let dist = x.pow(2) + z.pow(2);

          if dist > radius.pow(2) - 4
            && dist <= radius.pow(2) + 3
            && rng.rand_inclusive(0, 10) < chance
          {
            let pos = pos + Pos::new(x, y_offset, z);
            if world.get(pos) == block![air] {
              world.set(pos, self.leaves);
            }
          }
        }
      }
    }

    // Add a couple more leaves down the trunk.
    for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
      for i in 1..rng.rand_inclusive(1, 3) {
        let pos = pos + Pos::new(dx, -i, dz);
        if world.get(pos) == block![air] {
          world.set(pos, self.leaves);
        }
      }
    }

    Ok(())
  }
}
