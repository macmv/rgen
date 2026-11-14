use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Result, Rng, rng::Random};

/// Replaces exposed ground in irregular patches within two blocks of water.
pub struct ShorelineSplotch {
  pub replace:       BlockFilter,
  pub place:         BlockState,
  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

impl Placer for ShorelineSplotch {
  fn radius(&self) -> u8 { 8 + *self.radius.end() }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    for _ in 0..self.attempts {
      let x = pos.x + rng.range(-8..=8);
      let z = pos.z + rng.range(-8..=8);
      let Some(center) = exposed_ground(world, x, z, pos.y + 6, pos.y - 6, &self.replace) else {
        continue;
      };
      if !near_water(world, center) {
        continue;
      }

      let radius = rng.range(*self.radius.start() as i32..=*self.radius.end() as i32);
      let r2 = radius * radius;
      for dx in -radius..=radius {
        for dz in -radius..=radius {
          let dist2 = dx * dx + dz * dz;
          if dist2 > r2 || dist2 > rng.range(r2 / 2..=r2) {
            continue;
          }
          let Some(ground) = exposed_ground(
            world,
            center.x + dx,
            center.z + dz,
            center.y + 2,
            center.y - 2,
            &self.replace,
          ) else {
            continue;
          };
          if near_water(world, ground) {
            world.set(ground, self.place);
          }
        }
      }
    }
    Ok(())
  }
}

fn exposed_ground(
  world: &PartialWorld,
  x: i32,
  z: i32,
  top: i32,
  bottom: i32,
  replace: &BlockFilter,
) -> Option<Pos> {
  for y in (bottom.max(1)..=top.min(254)).rev() {
    let ground = Pos::new(x, y, z);
    if replace.contains(world.get(ground)) && world.get(ground + Pos::new(0, 1, 0)) == block![air] {
      return Some(ground);
    }
  }
  None
}

fn near_water(world: &PartialWorld, ground: Pos) -> bool {
  for dx in -2_i32..=2 {
    for dz in -2_i32..=2 {
      if (dx == 0 && dz == 0) || dx * dx + dz * dz > 4 {
        continue;
      }
      for dy in -1..=1 {
        if world.get(ground + Pos::new(dx, dy, dz)) == block![water] {
          return true;
        }
      }
    }
  }
  false
}
