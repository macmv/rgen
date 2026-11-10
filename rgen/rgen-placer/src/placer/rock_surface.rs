use rgen_base::{BlockState, Pos, block};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Result, Rng};

/// Makes a small, irregular rock outcrop with a gravel cap and a shallow
/// mixed-rock layer beneath it.
pub struct RockSurface {
  pub avg_per_chunk: f64,
  /// Width and length of the patch footprint in blocks.
  pub size:          u8,
  /// Materials mixed into the surface and the stone layer below it.
  pub mix_in:        Vec<BlockState>,
}

impl Placer for RockSurface {
  fn radius(&self) -> u8 { self.size / 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if self.mix_in.is_empty() {
      return Ok(());
    }

    let half_size = (self.size / 2) as i32;
    let center_limit = (half_size / 3).max(1);
    let lobe_radius = (half_size - 1).max(2);

    // A short, wandering chain of overlapping circles makes a connected,
    // lumpy footprint close to the configured width and length.
    let mut lobes = Vec::with_capacity(6);
    let mut cx = rng.range(-center_limit..=center_limit);
    let mut cz = rng.range(-center_limit..=center_limit);
    for _ in 0..6 {
      lobes.push((cx, cz, rng.range(2..=lobe_radius)));
      cx = (cx + rng.range(-2..=2)).clamp(-center_limit, center_limit);
      cz = (cz + rng.range(-2..=2)).clamp(-center_limit, center_limit);
    }

    for dx in -half_size..(half_size + i32::from(self.size % 2)) {
      for dz in -half_size..(half_size + i32::from(self.size % 2)) {
        let inside = lobes.iter().any(|&(x, z, radius)| {
          let x = dx - x;
          let z = dz - z;
          x * x + z * z <= radius * radius
        });
        if !inside {
          continue;
        }

        let x = pos.x + dx;
        let z = pos.z + dz;
        let surface_y = world
          .surfaces(Pos::new(x, 0, z))
          .iter()
          .copied()
          .max()
          .map(i32::from);
        let Some(surface_y) = surface_y else { continue };
        if surface_y < 2 || surface_y > 252 {
          continue;
        }

        let surface = Pos::new(x, surface_y, z);
        let current = world.get(surface);
        if current == block![air] || current == block![water] {
          continue;
        }

        // Surface plus one or two blocks beneath it.
        let gravel_depth = rng.range(2..=3);
        let stone_depth = rng.range(2..=3);

        // Gravel remains the dominant surface material, with occasional
        // exposed inclusions selected from the biome-provided mix.
        let surface_material = if rng.range(0..100) < 60 {
          block![gravel]
        } else {
          *rng.choose(&self.mix_in)
        };

        for depth in 0..gravel_depth {
          let block = if depth == 0 || surface_material != block![gravel] {
            surface_material
          } else {
            block![gravel]
          };
          world.set(surface + Pos::new(0, -depth, 0), block);
        }

        // Continue the same material palette below the gravel cap. Natural
        // stone is included alongside every configured surface inclusion.
        let mut stone_materials = Vec::with_capacity(self.mix_in.len() + 1);
        stone_materials.push(block![stone]);
        stone_materials.extend(self.mix_in.iter().copied());
        for depth in 0..stone_depth {
          let y = surface_y - gravel_depth - depth;
          world.set(
            Pos::new(x, y, z),
            *rng.choose(&stone_materials),
          );
        }
      }
    }

    Ok(())
  }
}
