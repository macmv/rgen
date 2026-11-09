use rgen_base::{BlockInfo, Pos, block};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Result, Rng};

/// Places fully grown, two-block reeds rooted in water.
pub struct Reeds {
  pub avg_per_chunk: f64,
  pub attempts: u32,
  /// If true, require exactly one water block above the supporting soil.
  pub must_be_one_block_deep: bool,
}

impl Default for Reeds {
  fn default() -> Self {
    Reeds { avg_per_chunk: 1.0, attempts: 64, must_be_one_block_deep: false }
  }
}

impl Placer for Reeds {
  fn radius(&self) -> u8 { 6 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    for _ in 0..self.attempts {
      // Repeated short random walks from one center form irregular reed beds
      // instead of scattering individual plants evenly across the area.
      let mut candidate = pos + Pos::new(0, rng.range(-4..=4), 0);
      for _ in 0..rng.range(2..=6) {
        candidate = candidate + Pos::new(rng.range(-1..=1), 0, rng.range(-1..=1));
      }
      let water = world.get(candidate);
      if water != block![water] {
        continue;
      }

      let surface_water = if self.must_be_one_block_deep {
        let soil = world.get(candidate + Pos::new(0, -1, 0));
        if soil == block![water] || !is_reed_soil(soil)
          || world.get(candidate + Pos::new(0, 1, 0)) == block![water]
        {
          continue;
        }
        candidate
      } else {
        let mut top = candidate;
        while top.y + 1 < 255 && world.get(top + Pos::new(0, 1, 0)) == block![water] {
          top = top + Pos::new(0, 1, 0);
        }

        let mut bottom = candidate;
        while bottom.y > 0 && world.get(bottom + Pos::new(0, -1, 0)) == block![water] {
          bottom = bottom + Pos::new(0, -1, 0);
        }
        let soil = world.get(bottom + Pos::new(0, -1, 0));
        if !is_reed_soil(soil) {
          continue;
        }
        top
      };

      if surface_water.y + 2 >= 255 {
        continue;
      }
      let lower = surface_water + Pos::new(0, 1, 0);
      let upper = surface_water + Pos::new(0, 2, 0);
      if world.get(lower) != block![air] || world.get(upper) != block![air] {
        continue;
      }

      // Metadata 3 is mature lower reeds; metadata 7 is its mature upper half.
      let reeds = block![rgen:reeds];
      world.set(lower, reeds.with_data(3));
      world.set(upper, reeds.with_data(7));
    }

    Ok(())
  }
}

fn is_reed_soil(soil: BlockInfo<'_>) -> bool {
  soil == block![dirt]
    || soil == block![grass]
    || soil == block![clay]
    || soil == block![gravel]
    || soil == block![sand]
    || soil == block![rgen:mud]
}
