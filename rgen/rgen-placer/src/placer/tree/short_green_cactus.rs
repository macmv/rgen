use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

/// Places short green cactus trunks, with an occasional branched flowering form.
pub struct ShortGreenCactus {
  pub place_above: BlockFilter,
  pub body: BlockState,
  pub arms: BlockState,
  pub large_flower: BlockState,
  pub small_flower: BlockState,
  pub avg_per_chunk: f64,
}

impl Default for ShortGreenCactus {
  fn default() -> Self {
    Self {
      place_above: block![sand].into(),
      body: block![rgen:cactus],
      arms: block![rgen:cactus_arm],
      large_flower: block![rgen:large_cactus_flower],
      small_flower: block![rgen:small_cactus_flower],
      avg_per_chunk: 3.0,
    }
  }
}

impl Placer for ShortGreenCactus {
  fn radius(&self) -> u8 { 1 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let below = pos + Pos::new(0, -1, 0);
    if pos.y <= 1 || !self.place_above.contains(world.get(below)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }

    // Three percent grow as a three-block trunk with one flowered arm.
    let branched = rng.range(0..100) < 3;
    let height = if branched { 3 } else { rng.range(1..=2) };
    let trunk_flower_pos = pos + Pos::new(0, height, 0);
    if pos.y + height >= 255 || world.get(trunk_flower_pos) != block![air] {
      return Err(UndoError);
    }

    let (arm_pos, arm_flower_pos, arm_face) = if branched {
      let (offset, face) = match rng.range(0..4) {
        0 => (Pos::new(0, 2, -1), 2), // north; arm face points back to trunk
        1 => (Pos::new(1, 2, 0), 3),
        2 => (Pos::new(0, 2, 1), 0),
        _ => (Pos::new(-1, 2, 0), 1),
      };
      let arm = pos + offset;
      let flower = arm + Pos::new(0, 1, 0);
      if pos.y + 3 >= 255 || world.get(arm) != block![air] || world.get(flower) != block![air] {
        return Err(UndoError);
      }
      (Some(arm), Some(flower), face)
    } else {
      (None, None, 0)
    };

    for y in 0..height {
      world.set(pos + Pos::new(0, y, 0), self.body);
    }
    if branched {
      let arm = arm_pos.expect("branched cactus has an arm position");
      let flower = arm_flower_pos.expect("branched cactus has an arm flower position");
      world.set(arm, self.arms.with_data(arm_face));
      world.set(flower, self.small_flower.with_data(arm_face));
      world.set(trunk_flower_pos, self.large_flower);
    } else if rng.range(0..100) < 80 {
      world.set(trunk_flower_pos, self.large_flower);
    }

    Ok(())
  }
}
