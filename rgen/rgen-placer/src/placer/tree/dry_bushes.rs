use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Result, Rng};

pub struct BasicDryBush {
  pub place_above:   BlockFilter,
  /// Ground that must surround the planting point and support the canopy.
  pub ground:        BlockFilter,
  /// Plants that leaves may replace when filling the bush canopy.
  pub plants:        BlockFilter,
  pub trunk:         BlockState,
  pub leaves:        BlockState,
  pub avg_per_chunk: f64,
}

impl Default for BasicDryBush {
  fn default() -> Self {
    BasicDryBush {
      avg_per_chunk: 1.0,
      place_above:   block![grass].into(),
      ground:        [block![grass], block![dirt]].into(),
      plants:        [block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower]].into(),
      trunk:         block![log[variant = "birch"]],
      leaves:        block![rgen:leaves3],
    }
  }
}

impl Placer for BasicDryBush {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, _rng: &mut Rng, pos: Pos) -> Result {
    // Checks if outside world boundry.
    if pos.y + 2 + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    let ground_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(ground_pos)) {
      return Err(UndoError);
    }

    // Keep the shrub on a stable, dry 3x3 patch, as TallBush does.
    for x in -1_i32..=1 {
      for z in -1_i32..=1 {
        let soil = world.get(ground_pos + Pos::new(x, 0, z));
        if soil == block![water] || !self.ground.contains(soil) {
          return Err(UndoError);
        }
      }
    }

    if world.get(pos) != block![air] {
      return Err(UndoError);
    }

    // Place the core only after the ground and trunk position pass validation.
    world.set(pos, self.trunk);

    for y in 0..=1_i32 {
      for x in -1..=1_i32 {
        for z in -1..=1_i32 {
          // Remove the corners.
          if x.abs() == 1 && z.abs() == 1 && y == 1 {
            continue; // next loop
          }

          if y == 0 && x == 0 && z == 0 {
            continue;
          }

          let leaf_pos = pos + Pos::new(x, y, z);
          let existing = world.get(leaf_pos);
          if existing == block![air] || self.plants.contains(existing) {
            world.set(leaf_pos, self.leaves);
          }
        }
      }
    }

    Ok(())
  }
}
