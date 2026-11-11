use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Result, Rng};

/// A small surface deposit with a stone-rich substrate beneath it.
pub struct SubsurfaceSplotch {
  pub surface_replace: BlockFilter,
  pub below_replace: BlockFilter,
  pub place: BlockState,
  pub below_mix: Vec<BlockState>,
  pub radius: RangeInclusive<u8>,
  pub depth: RangeInclusive<u8>,
  pub avg_per_chunk: f64,
  /// Chance that the patch stretches along one horizontal axis, from 0 to 100.
  pub stretch_chance: u8,
}

impl Placer for SubsurfaceSplotch {
  fn radius(&self) -> u8 { self.radius.end().saturating_mul(2) }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if self.below_mix.is_empty() {
      return Ok(());
    }

    let radius = rng.range(*self.radius.start() as i32..=*self.radius.end() as i32);
    let stretched = rng.range(0..100) < i32::from(self.stretch_chance);
    let stretch_x = stretched && rng.range(0..2) == 0;
    let stretch_z = stretched && !stretch_x;
    let radius_x = radius * if stretch_x { 2 } else { 1 };
    let radius_z = radius * if stretch_z { 2 } else { 1 };
    let radius_product = radius_x * radius_z;

    for dx in -radius_x..=radius_x {
      for dz in -radius_z..=radius_z {
        let ellipse_distance = dx * dx * radius_z * radius_z + dz * dz * radius_x * radius_x;
        if ellipse_distance > radius_product * radius_product {
          continue;
        }
        if (dx != 0 || dz != 0) && rng.range(0..5) == 0 {
          continue;
        }

        let x = pos.x + dx;
        let z = pos.z + dz;
        let Some(surface_y) = world.surfaces(Pos::new(x, 0, z)).iter().copied().max().map(i32::from) else {
          continue;
        };
        if !(1..=254).contains(&surface_y) {
          continue;
        }

        let surface = Pos::new(x, surface_y, z);
        if !self.surface_replace.contains(world.get(surface)) {
          continue;
        }

        let deposit_depth = rng.range(*self.depth.start() as i32..=*self.depth.end() as i32);
        world.set(surface, self.place);
        for layer in 1..deposit_depth {
          let layer_pos = surface + Pos::new(0, -layer, 0);
          if !self.below_replace.contains(world.get(layer_pos)) {
            break;
          }
          world.set(layer_pos, self.place);
        }

        // Turn the remaining dirt column beneath the deposit into the supplied
        // stone palette, stopping when the natural dirt layer ends.
        let mut layer = deposit_depth;
        loop {
          let layer_pos = surface + Pos::new(0, -layer, 0);
          if layer_pos.y <= 0 || !self.below_replace.contains(world.get(layer_pos)) {
            break;
          }
          world.set(layer_pos, *rng.choose(&self.below_mix));
          layer += 1;
        }
      }
    }
    Ok(())
  }
}
