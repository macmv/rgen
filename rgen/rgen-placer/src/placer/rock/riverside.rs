use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct RiverSide {
  pub ground:        BlockFilter,
  pub material:      Vec<BlockState>,
  pub avg_per_chunk: f64,
  pub fluid:         BlockState,
}

impl Default for RiverSide {
  fn default() -> Self {
    RiverSide {
      ground:        [block![dirt], block![grass]].into(),
      material:      vec![
        block![gravel],
        block![gravel],
        block![rgen:mossy_cobblestone_rgen],
        block![cobblestone],
      ],
      avg_per_chunk: 3.0,
      fluid:         block![lava],
    }
  }
}

impl Placer for RiverSide {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, mut pos: Pos) -> Result {
    pos = pos + Pos::new(0, -1, 0);
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        self.build_siding(rng, pos + Pos::new(rel_x, 0, rel_z), world);
      }
    }

    Ok(())
  }
}

impl RiverSide {
  fn build_siding(&self, rng: &mut Rng, pos: Pos, world: &mut PartialWorld) {
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        if !(rel_x == 0 && rel_z == 0) {
          let rel_pos = pos + Pos::new(rel_x, 0, rel_z);
          if world.get(rel_pos) == block![water] && self.ground.contains(world.get(pos)) {
            world.set(pos, *rng.choose(&self.material));
          }
        }
      }
    }
  }
}
