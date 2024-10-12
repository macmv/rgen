use rgen_base::{Block, BlockState, Blocks, Pos};
use rgen_world::PartialWorld;
use std::ops::RangeInclusive;

use crate::{Placer, Random, Rng};

pub struct WaterResources {
  pub placement:          BlockState,
  pub tool_placement:     BlockState,
  pub tool_placement_two: BlockState,
  pub avg_in_chunk:       f64,
  pub size:               RangeInclusive<u8>,
  pub multiplyer:         i32,
}

impl WaterResources {
  pub fn new(blocks: &Blocks) -> Self {
    WaterResources {
      avg_in_chunk:       1.0,
      placement:          blocks.clay.default_state.into(),
      tool_placement:     blocks.gold_block.default_state.into(),
      tool_placement_two: blocks.iron_ore.default_state.into(),
      size:               2..=4,
      multiplyer:         3,
    }
  }
}

impl Placer for WaterResources {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return;
    }
    for _ in 1..=self.multiplyer {
      self.find_water_depth(
        world,
        pos
          + Pos::new(
            (rng.rand_inclusive((*self.size.start()).into(), (*self.size.end()).into()) / 2) + 1,
            0,
            (rng.rand_inclusive((*self.size.start()).into(), (*self.size.end()).into()) / 2) + 1,
          ),
        rng,
      );
    }
  }
}

impl WaterResources {
  fn find_water_depth(&self, world: &mut PartialWorld, pos: Pos, rng: &mut Rng) {
    if world.get(pos + Pos::new(0, -1, 0)).block == Block::WATER {
      //world.set(pos, self.tool_placement);
    } else {
      return;
    }
    let mut depth_pos = pos;
    for y in 1..=10 {
      depth_pos = depth_pos + Pos::new(0, -1, 0);
      if world.get(depth_pos).block != Block::WATER {
        break;
      }
      //world.set(depth_pos, self.placement);

      if y > 9 {
        return;
      }
    }

    //
    self.build_clump(world, depth_pos, rng);
  }
  fn build_clump(&self, world: &mut PartialWorld, pos: Pos, rng: &mut Rng) {
    let radius = rng.rand_inclusive((*self.size.start()).into(), (*self.size.end()).into());

    let r2 = radius.pow(2);

    for x in -radius..=radius {
      for y in -radius..=radius {
        for z in -radius..=radius {
          let pos = pos + Pos::new(x, y, z);

          let dist2 = x.pow(2) + y.pow(2) + z.pow(2);
          if dist2 > r2 {
            continue;
          }

          if dist2 > rng.rand_inclusive(r2 / 2, r2) {
            continue;
          }

          if world.get(pos).block != Block::WATER
            && world.get(pos + Pos::new(0, 1, 0)).block != Block::AIR
            && pos.y < 63
          {
            world.set(pos, self.placement);
          }
        }
      }
    }
  }
}
