use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, Pos, block};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Result, Rng};

/// Makes irregular patches on the submerged floor of shallow water.
pub struct WaterbedPatches {
  pub replace:       BlockFilter,
  pub place:         BlockState,
  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

impl Placer for WaterbedPatches {
  fn radius(&self) -> u8 { 8 + *self.radius.end() }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    for _ in 0..self.attempts {
      let x = pos.x + rng.range(-8..=8);
      let z = pos.z + rng.range(-8..=8);
      let y = pos.y + rng.range(-4..=4);
      let Some(center) = water_floor(world, x, z, y) else { continue };
      let radius = rng.range(*self.radius.start() as i32..=*self.radius.end() as i32);

      for dx in -radius..=radius {
        for dz in -radius..=radius {
          let distance = dx * dx + dz * dz;
          if distance > radius * radius || rng.range(0..=radius * radius) < distance {
            continue;
          }

          if let Some(floor) = water_floor(world, center.x + dx, center.z + dz, center.y + 3) {
            let above = floor + Pos::new(0, 1, 0);
            if self.replace.contains(world.get(floor)) && world.get(above) == block![water] {
              world.set(floor, self.place);
            }
          }
        }
      }
    }

    Ok(())
  }
}

fn water_floor(world: &PartialWorld, x: i32, z: i32, y: i32) -> Option<Pos> {
  let top = (y + 8).min(254);
  let bottom = (y - 20).max(1);
  for scan_y in (bottom..=top).rev() {
    let water = Pos::new(x, scan_y, z);
    if world.get(water) != block![water] {
      continue;
    }

    let mut floor = water;
    while floor.y > 1 && world.get(floor + Pos::new(0, -1, 0)) == block![water] {
      floor = floor + Pos::new(0, -1, 0);
    }
    return Some(floor + Pos::new(0, -1, 0));
  }
  None
}
