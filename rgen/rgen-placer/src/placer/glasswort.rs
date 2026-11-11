use rgen_base::{Pos, block};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Result, Rng};

/// Scatters glasswort on mud near water.
pub struct Glasswort {
  pub avg_per_chunk: f64,
  pub attempts: u32,
}

impl Default for Glasswort {
  fn default() -> Self { Glasswort { avg_per_chunk: 1.0, attempts: 320 } }
}

impl Placer for Glasswort {
  fn radius(&self) -> u8 { 8 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    for _ in 0..self.attempts {
      let plant_pos = pos + Pos::new(rng.range(-8..=8), rng.range(-4..=4), rng.range(-8..=8));

      // Reject almost all candidates before doing the neighborhood water scan.
      if world.get(plant_pos + Pos::new(0, -1, 0)) != block![rgen:mud]
        || world.get(plant_pos) != block![air]
      {
        continue;
      }

      if water_within_three_blocks(world, plant_pos) {
        let variant = rng.range(0..=5) as u8;
        world.set(plant_pos, block![rgen:glasswort].with_data(variant));
      }
    }

    Ok(())
  }
}

/// Searches nearest positions first and stops on the first water block.
/// The distance is horizontal (Euclidean); checking both plant height and
/// ground height handles shorelines with a one-block level difference.
fn water_within_three_blocks(world: &PartialWorld, plant_pos: Pos) -> bool {
  for ring in 1_i32..=3 {
    for dx in -ring..=ring {
      for dz in -ring..=ring {
        if dx.abs().max(dz.abs()) != ring || dx * dx + dz * dz > 9 {
          continue;
        }

        let neighbor = plant_pos + Pos::new(dx, 0, dz);
        if world.get(neighbor) == block![water]
          || world.get(neighbor + Pos::new(0, -1, 0)) == block![water]
        {
          return true;
        }
      }
    }
  }

  false
}
