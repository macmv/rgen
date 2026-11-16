use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

/// A layered bush with an upright log core and a broad, irregular canopy.
pub struct TallBush {
  /// Ground accepted around the planting point and beneath outer leaves.
  pub ground: BlockFilter,
  /// Ground required directly beneath the trunk.
  pub plantable: BlockFilter,
  /// Existing plants that the bush is allowed to replace.
  pub plants: BlockFilter,
  pub log: BlockState,
  pub leaves: BlockState,
  /// Allow water beneath canopy positions during the surrounding-ground check.
  pub allow_water_under_leaves: bool,
  pub avg_per_chunk: f64,
}

impl TallBush {
  /// Restrict this bush to banks with exposed water within three blocks.
  pub fn near_water(self) -> impl Placer { ShorelineTallBush(self) }
}

struct ShorelineTallBush(TallBush);

impl Placer for ShorelineTallBush {
  fn radius(&self) -> u8 { self.0.radius().max(3) }
  fn avg_per_chunk(&self) -> f64 { self.0.avg_per_chunk() }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if pos.y < 4 || pos.y + 5 >= 255 {
      return Err(UndoError);
    }
    let mut near_water = false;
    'search: for x in -3_i32..=3 {
      for z in -3_i32..=3 {
        if x * x + z * z > 9 { continue; }
        for y in -3..=0 {
          let water = pos + Pos::new(x, y, z);
          if world.get(water) == block![water]
            && world.get(water + Pos::new(0, 1, 0)) == block![air]
          {
            near_water = true;
            break 'search;
          }
        }
      }
    }
    if !near_water { return Err(UndoError); }
    self.0.place(world, rng, pos)
  }
}

impl Placer for TallBush {
  fn radius(&self) -> u8 { 3 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if pos.y <= 1 || pos.y + 5 >= 255 {
      return Err(UndoError);
    }

    let ground_pos = pos + Pos::new(0, -1, 0);
    if !self.plantable.contains(world.get(ground_pos)) {
      return Err(UndoError);
    }

    // Require acceptable ground throughout the one-block radius. Riparian
    // bushes may extend their canopy over water while still rooting on land.
    for x in -1_i32..=1 {
      for z in -1_i32..=1 {
        let ground = world.get(ground_pos + Pos::new(x, 0, z));
        if (ground == block![water] && !self.allow_water_under_leaves)
          || (ground != block![water] && !self.ground.contains(ground))
        {
          return Err(UndoError);
        }
      }
    }

    let layer1 = pos;
    if world.get(layer1) != block![air] {
      return Err(UndoError);
    }

    let mut placements = vec![(layer1, self.log)];
    let mut add_leaves = |pos: Pos| {
      let existing = world.get(pos);
      if existing == block![air] || self.plants.contains(existing) {
        placements.push((pos, self.leaves));
      }
    };

    // Layer 0: occasional leaves at ground level around the trunk.
    for x in -1_i32..=1 {
      for z in -1_i32..=1 {
        let leaf_pos = ground_pos + Pos::new(x, 0, z);
        if rng.range(0..=3) == 0 {
          add_leaves(leaf_pos);
        }
      }
    }

    // Layer 1: central upright log with a 3x3 leaf ring.
    for x in -1_i32..=1 {
      for z in -1_i32..=1 {
        if x != 0 || z != 0 {
          add_leaves(layer1 + Pos::new(x, 0, z));
        }
      }
    }

    // Occasional mid-edge leaves extend layer 1 into a four-block-wide cross.
    for (x, z) in [(-2, 0), (2, 0), (0, -2), (0, 2)] {
      let soil = world.get(ground_pos + Pos::new(x, 0, z));
      if self.ground.contains(soil) && rng.range(0..=3) == 0 {
        add_leaves(layer1 + Pos::new(x, 0, z));
      }
    }

    // Layer 2: 3x3 crown, with one randomly chosen corner removed 1-in-5.
    let layer2 = pos + Pos::new(0, 1, 0);
    let remove_corner = rng.range(0..5) == 0;
    let removed_corner = rng.range(0..4) as usize;
    let corners = [(-1, -1), (1, -1), (1, 1), (-1, 1)];
    for x in -1_i32..=1 {
      for z in -1_i32..=1 {
        if remove_corner && corners[removed_corner] == (x, z) {
          continue;
        }
        add_leaves(layer2 + Pos::new(x, 0, z));
      }
    }

    // Layer 3: plus, with a 65% chance to remove one arm and 25% for a second.
    let layer3 = pos + Pos::new(0, 2, 0);
    let mut sides = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    for i in (1..sides.len()).rev() {
      let j = rng.range(0..=i as i32) as usize;
      sides.swap(i, j);
    }
    let remove_one = rng.range(0..100) < 65;
    let remove_two = remove_one && rng.range(0..100) < 25;
    let removed = if remove_two { 2 } else if remove_one { 1 } else { 0 };
    for (i, (x, z)) in sides.into_iter().enumerate() {
      if i >= removed {
        add_leaves(layer3 + Pos::new(x, 0, z));
      }
    }
    add_leaves(layer3); // The core is never removed.

    // Layer 4: crown tip, plus a 25% continuation above each surviving arm.
    let layer4 = pos + Pos::new(0, 3, 0);
    add_leaves(layer4);
    for (i, (x, z)) in sides.into_iter().enumerate() {
      if i >= removed && rng.range(0..=3) == 0 {
        add_leaves(layer4 + Pos::new(x, 0, z));
      }
    }

    // Layer 5: a single top leaf has a 35% chance to grow above the crown.
    if rng.range(0..100) < 35 {
      add_leaves(pos + Pos::new(0, 4, 0));
    }

    for (pos, state) in placements {
      world.set(pos, state);
    }
    Ok(())
  }
}
