use rgen_base::{BlockFilter, BlockState, Pos, block};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};
use crate::placer::wood_clearance::{WoodSpecies, any_wood_adjacent, different_wood_adjacent};

/// Hardcoded Lebanon cedar forms, based on the supplied glass study.
pub struct CedarTree {
  pub ground_mix:    BlockFilter,
  pub growable_mix:  BlockFilter,
  pub plant_mix:     BlockFilter,
  pub trunk:         BlockState,
  pub leaves:        BlockState,
  pub large_chance:  u8,
  pub medium_chance: u8,
  pub small_chance:  u8,
  pub avg_per_chunk: f64,
}

impl Default for CedarTree {
  fn default() -> Self {
    Self {
      ground_mix:    [block![grass], block![dirt], block![gravel]].into(),
      growable_mix:  [block![grass], block![dirt]].into(),
      plant_mix:     [block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower]].into(),
      trunk:         block![rgen:log[variant = "cedar"]],
      leaves:        block![rgen:leaves[variant = "cedar", check_decay = false, decayable = true]],
      large_chance:  50,
      medium_chance: 50,
      small_chance:  0,
      avg_per_chunk: 12.0,
    }
  }
}

impl Placer for CedarTree {
  fn radius(&self) -> u8 { 6 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let roll = rng.range(0..100) as u8;
    let small = (roll as u16) >= self.large_chance as u16 + self.medium_chance as u16;
    let medium = !small && roll >= self.large_chance;
    if (roll as u16) >= self.large_chance as u16 + self.medium_chance as u16 + self.small_chance as u16 {
      return Err(UndoError);
    }

    let height = if small { rng.range(5..=6) } else if medium { rng.range(7..=9) } else { rng.range(10..=12) };
    let head_layer = if medium { height - 1 } else if height <= 11 { 9 } else { 10 };
    if pos.y + 18 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    let center_ground = pos + Pos::new(0, -1, 0);
    if !self.growable_mix.contains(world.get(center_ground)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }

    // Check the immediate rooting area: it must be land in the supplied
    // ground palette, with no water beneath the tree.
    for dx in -1_i32..=1 {
      for dz in -1_i32..=1 {
        let soil = world.get(center_ground + Pos::new(dx, 0, dz));
        if soil == block![water] || !self.ground_mix.contains(soil) {
          return Err(UndoError);
        }
      }
    }

    // The small form keeps its canopy proportions but has one fewer log.
    let trunk_height = if small { height - 1 } else { height };

    // Place a straight cedar pole. Plants may be cleared by the trunk.
    for y in 0..trunk_height {
      let trunk_pos = pos + Pos::new(0, y, 0);
      let current = world.get(trunk_pos);
      if (current != block![air] && !self.plant_mix.contains(current))
        || any_wood_adjacent(world, trunk_pos) {
        return Err(UndoError);
      }
    }

    // One 5x5 base with three leaves centered along each side.
    // The trunk occupies the center cell.
    const BASE: [&str; 7] = [
      "  LLL  ",
      " LLLLL ",
      "LLLLLLL",
      "LLLXLLL",
      "LLLLLLL",
      " LLLLL ",
      "  LLL  ",
    ];
    for y in 0..trunk_height {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }
    place_cedar_roots(self, world, rng, pos);
    if small {
      place_small_cedar(self, world, rng, pos + Pos::new(0, -1, 0), height);
      return Ok(());
    }
    if medium {
      // The medium cedar uses a 5x5 base four logs below the tip.
      let base_y = height - 4;
      for dz in -2_i32..=2 {
        for dx in -2_i32..=2 {
          if (dx == 0 && dz == 0) || (dx.abs() == 2 && dz.abs() == 2) {
            continue;
          }
          let leaf_pos = pos + Pos::new(dx, base_y, dz);
          let existing = world.get(leaf_pos);
          if existing == block![air] || self.plant_mix.contains(existing) {
            world.set(leaf_pos, self.leaves);
          }
        }
      }
      // Each three-leaf base edge continues as ?L? one layer higher.
      // The middle is certain, each end is 50%, and a successful end
      // earns that side a further middle tip on the following layer.
      let mut medium_tips = [false; 4];
      for side in 0..4 {
        for offset in -1..=1 {
          if offset != 0 && rng.range(0..100) >= 50 {
            continue;
          }
          let (dx, dz) = match side {
            0 => (offset, -2),
            1 => (2, offset),
            2 => (offset, 2),
            _ => (-2, offset),
          };
          let leaf_pos = pos + Pos::new(dx, base_y + 1, dz);
          let existing = world.get(leaf_pos);
          if existing == block![air] || self.plant_mix.contains(existing) {
            world.set(leaf_pos, self.leaves);
            if offset != 0 {
              medium_tips[side] = true;
            }
          }
        }
      }
      for side in 0..4 {
        if !medium_tips[side] {
          continue;
        }
        let (dx, dz) = match side {
          0 => (0, -2),
          1 => (2, 0),
          2 => (0, 2),
          _ => (-2, 0),
        };
        let leaf_pos = pos + Pos::new(dx, base_y + 2, dz);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
        }
      }
      // The -1 layer is a full 3x3, with an independent 55% hanging
      // leaf below the middle of each base siding.
      for dz in -1_i32..=1 {
        for dx in -1_i32..=1 {
          if dx == 0 && dz == 0 {
            continue;
          }
          let leaf_pos = pos + Pos::new(dx, base_y - 1, dz);
          let existing = world.get(leaf_pos);
          if existing == block![air] || self.plant_mix.contains(existing) {
            world.set(leaf_pos, self.leaves);
          }
        }
      }
      for (dx, dz) in [(0, -2), (2, 0), (0, 2), (-2, 0)] {
        if rng.range(0..100) >= 55 {
          continue;
        }
        let leaf_pos = pos + Pos::new(dx, base_y - 1, dz);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
        }
      }
      // The large cedar's lower cross, scaled to this smaller canopy.
      let arm_count = if rng.range(0..2) == 0 { 2 } else { 3 };
      let mut arms = [(0, -1), (1, 0), (0, 1), (-1, 0)];
      for index in 0..arm_count {
        let chosen = rng.range(index..4);
        arms.swap(index as usize, chosen as usize);
        let (dx, dz) = arms[index as usize];
        let leaf_pos = pos + Pos::new(dx, base_y - 2, dz);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
        }
      }
      for (dx, dz) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        if rng.range(0..100) >= 10 {
          continue;
        }
        let leaf_pos = pos + Pos::new(dx, base_y - 3, dz);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
        }
      }
      // Above the base, the medium canopy narrows to 3x3.
      for layer_y in [base_y + 1, base_y + 2, head_layer] {
        for dz in -1_i32..=1 {
          for dx in -1_i32..=1 {
            if dx == 0 && dz == 0 {
              continue;
            }
            if dx.abs() == 1 && dz.abs() == 1
              && layer_y == head_layer && rng.range(0..100) < 15 {
              continue;
            }
            let leaf_pos = pos + Pos::new(dx, layer_y, dz);
            let existing = world.get(leaf_pos);
            if existing == block![air] || self.plant_mix.contains(existing) {
              world.set(leaf_pos, self.leaves);
            }
          }
        }
      }
      for (dx, dz) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let leaf_pos = pos + Pos::new(dx, height, dz);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
          // Each arm has a 20% chance to continue to a second level.
          if rng.range(0..100) < 20 {
            let upper_pos = pos + Pos::new(dx, height + 1, dz);
            let upper_existing = world.get(upper_pos);
            if upper_existing == block![air] || self.plant_mix.contains(upper_existing) {
              world.set(upper_pos, self.leaves);
            }
          }
        }
      }
      let crown_height = if rng.range(0..2) == 0 { 3 } else { 4 };
      for y in height..height + crown_height {
        let leaf_pos = pos + Pos::new(0, y, 0);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
        }
      }
      return Ok(());
    }
    // `pos` is ground + 1, so local Y=4 is five above the ground.
    for (z, row) in BASE.iter().enumerate() {
      for (x, cell) in row.chars().enumerate() {
        if cell != 'L' {
          continue;
        }
        let leaf_pos = pos + Pos::new(x as i32 - 3, 4, z as i32 - 3);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
        }
      }
    }

    // The layer below the base is 5x5. Each corner is omitted 75% of
    // the time; all other cells are leaves apart from the trunk.
    for dz in -2_i32..=2 {
      for dx in -2_i32..=2 {
        if dx == 0 && dz == 0 {
          continue;
        }
        if dx.abs() == 2 && dz.abs() == 2 && rng.range(0..100) < 75 {
          continue;
        }
        let leaf_pos = pos + Pos::new(dx, 3, dz);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
        }
      }
    }

    // A hanging leaf below the middle of each three-leaf outer edge.
    for (dx, dz) in [(0, -3), (3, 0), (0, 3), (-3, 0)] {
      if rng.range(0..100) >= 75 {
        continue;
      }
      let leaf_pos = pos + Pos::new(dx, 3, dz);
      let existing = world.get(leaf_pos);
      if existing == block![air] || self.plant_mix.contains(existing) {
        world.set(leaf_pos, self.leaves);
      }
    }

    // Two blocks below the base, keep two or three of the four arms
    // around the trunk. Every choice is independent for each tree.
    let arm_count = if rng.range(0..2) == 0 { 2 } else { 3 };
    let mut arms = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    for index in 0..arm_count {
      let chosen = rng.range(index..4);
      arms.swap(index as usize, chosen as usize);
      let (dx, dz) = arms[index as usize];
      let leaf_pos = pos + Pos::new(dx, 2, dz);
      let existing = world.get(leaf_pos);
      if existing == block![air] || self.plant_mix.contains(existing) {
        world.set(leaf_pos, self.leaves);
      }
    }

    // Three blocks below the base, each arm has its own 10% chance.
    for (dx, dz) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
      if rng.range(0..100) >= 10 {
        continue;
      }
      let leaf_pos = pos + Pos::new(dx, 1, dz);
      let existing = world.get(leaf_pos);
      if existing == block![air] || self.plant_mix.contains(existing) {
        world.set(leaf_pos, self.leaves);
      }
    }

    // Layer 1 has a 5x5 body centered on the trunk.
    for dz in -2..=2 {
      for dx in -2..=2 {
        if dx == 0 && dz == 0 {
          continue;
        }
        let leaf_pos = pos + Pos::new(dx, 5, dz);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
        }
      }
    }

    // Layer 1 has a three-position strip above every base edge. Each
    // strip's middle leaf is certain; its two side leaves are 50% each.
    // A side earns a layer-2 tip if either side leaf actually places.
    let mut tips = [false; 4];
    for side in 0..4 {
      for offset in -1..=1 {
        if offset != 0 && rng.range(0..100) >= 50 {
          continue;
        }
        let (dx, dz) = match side {
          0 => (offset, -3),
          1 => (3, offset),
          2 => (offset, 3),
          _ => (-3, offset),
        };
        let leaf_pos = pos + Pos::new(dx, 5, dz);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
          if offset != 0 {
            tips[side] = true;
          }
        }
      }
    }

    // The 3x3 body ends at the head: layer 5 for 10- or 11-block
    // trunks, layer 6 for 12-block trunks. Layer 2 and the head
    // independently remove each corner 15% of the time.
    for layer_y in 6..=head_layer {
      for dz in -1_i32..=1 {
        for dx in -1_i32..=1 {
          if dx == 0 && dz == 0 && layer_y < height {
            continue;
          }
          let trim_corners = layer_y == 6 || layer_y == head_layer;
          if trim_corners && dx.abs() == 1 && dz.abs() == 1 && rng.range(0..100) < 15 {
            continue;
          }
          let leaf_pos = pos + Pos::new(dx, layer_y, dz);
          let existing = world.get(leaf_pos);
          if existing == block![air] || self.plant_mix.contains(existing) {
            world.set(leaf_pos, self.leaves);
          }
        }
      }
    }

    // Layer 2 sidings: three leaves along each side of the 3x3 center.
    // Together they form a 5x5 footprint without its outer corners.
    for offset in -1..=1 {
      for (dx, dz) in [(offset, -2), (2, offset), (offset, 2), (-2, offset)] {
        let leaf_pos = pos + Pos::new(dx, 6, dz);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
        }
      }
    }

    // A 10-block trunk keeps only the middle of each siding on layer 3.
    // Taller trunks keep three-leaf strips, with 15% chance to omit one end.
    for side in 0..4 {
      let missing_end = if height > 10 && rng.range(0..100) < 15 {
        if rng.range(0..2) == 0 { -1 } else { 1 }
      } else {
        2
      };
      for offset in -1..=1 {
        if (height == 10 && offset != 0) || offset == missing_end {
          continue;
        }
        let (dx, dz) = match side {
          0 => (offset, -2),
          1 => (2, offset),
          2 => (offset, 2),
          _ => (-2, offset),
        };
        let leaf_pos = pos + Pos::new(dx, 7, dz);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
        }
      }
    }

    // For 10-block trunks, each siding middle has a 50% layer-4 leaf and
    // stops there. Taller trunks always get layer 4 and may get layer 5.
    for (dx, dz) in [(0, -2), (2, 0), (0, 2), (-2, 0)] {
      if height == 10 && rng.range(0..2) == 0 {
        continue;
      }
      let lower_pos = pos + Pos::new(dx, 8, dz);
      let existing = world.get(lower_pos);
      if existing != block![air] && !self.plant_mix.contains(existing) {
        continue;
      }
      world.set(lower_pos, self.leaves);
      if height == 10 || rng.range(0..2) == 0 {
        continue;
      }
      let upper_pos = pos + Pos::new(dx, 9, dz);
      let existing = world.get(upper_pos);
      if existing == block![air] || self.plant_mix.contains(existing) {
        world.set(upper_pos, self.leaves);
      }
    }

    // Each outer middle grows when that side earned a tip on layer 1.
    for side in 0..4 {
      if !tips[side] {
        continue;
      }
      let (dx, dz) = match side {
        0 => (0, -3),
        1 => (3, 0),
        2 => (0, 3),
        _ => (-3, 0),
      };
      let leaf_pos = pos + Pos::new(dx, 6, dz);
      let existing = world.get(leaf_pos);
      if existing == block![air] || self.plant_mix.contains(existing) {
        world.set(leaf_pos, self.leaves);
      }
    }

    // Two identical plus-shaped layers rise above the head. Their
    // centers are log where the pole reaches and crown leaves above it.
    for layer_y in [head_layer + 1, head_layer + 2] {
      for (dx, dz) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let leaf_pos = pos + Pos::new(dx, layer_y, dz);
        let existing = world.get(leaf_pos);
        if existing == block![air] || self.plant_mix.contains(existing) {
          world.set(leaf_pos, self.leaves);
        }
      }
    }

    // Each arm may continue one block higher, independently, making
    // that arm three leaves tall above the head.
    for (dx, dz) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
      if rng.range(0..100) >= 25 {
        continue;
      }
      let leaf_pos = pos + Pos::new(dx, head_layer + 3, dz);
      let existing = world.get(leaf_pos);
      if existing == block![air] || self.plant_mix.contains(existing) {
        world.set(leaf_pos, self.leaves);
        // A three-high arm gets one leaf beside its first layer.
        // Rotate the arm direction left or right with equal odds.
        let side = if rng.range(0..2) == 0 { -1 } else { 1 };
        let side_pos = pos + Pos::new(dx - side * dz, head_layer + 1, dz + side * dx);
        let side_existing = world.get(side_pos);
        if side_existing == block![air] || self.plant_mix.contains(side_existing) {
          world.set(side_pos, self.leaves);
        }
      }
    }

    // A narrow crown rises three or four leaves above the actual log tip.
    let crown_height = if rng.range(0..2) == 0 { 3 } else { 4 };
    for y in height..height + crown_height {
      let leaf_pos = pos + Pos::new(0, y, 0);
      let existing = world.get(leaf_pos);
      if existing == block![air] || self.plant_mix.contains(existing) {
        world.set(leaf_pos, self.leaves);
      }
    }

    Ok(())
  }
}

fn small_leaf(tree: &CedarTree, world: &mut PartialWorld, pos: Pos) -> bool {
  let existing = world.get(pos);
  if existing == block![air] || tree.plant_mix.contains(existing) {
    world.set(pos, tree.leaves);
    true
  } else {
    false
  }
}

fn supported_small_leaf(tree: &CedarTree, world: &mut PartialWorld, pos: Pos) -> bool {
  let below = world.get(pos + Pos::new(0, -1, 0));
  if below != tree.leaves && below != tree.trunk {
    return false;
  }
  small_leaf(tree, world, pos)
}

fn place_small_cedar(tree: &CedarTree, world: &mut PartialWorld, rng: &mut Rng, pos: Pos, height: i32) {
  const SIDES: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

  // Layer 0 is a solid 3x3 around the third log, with four optional
  // points two blocks from the trunk.
  for dz in -1..=1 {
    for dx in -1..=1 {
      if dx != 0 || dz != 0 {
        small_leaf(tree, world, pos + Pos::new(dx, 3, dz));
      }
    }
  }
  let mut base_sides = [false; 4];
  for (side, (dx, dz)) in SIDES.iter().copied().enumerate() {
    if rng.range(0..100) < 55 {
      base_sides[side] = small_leaf(tree, world, pos + Pos::new(dx * 2, 3, dz * 2));
    }
  }

  // One layer below the base, each existing side can grow an arm 50%.
  // Two layers below, every arm has its own 20% chance.
  for (side, (dx, dz)) in SIDES.iter().copied().enumerate() {
    if base_sides[side] && rng.range(0..100) < 50 {
      small_leaf(tree, world, pos + Pos::new(dx, 2, dz));
    }
    if rng.range(0..100) < 20 {
      small_leaf(tree, world, pos + Pos::new(dx, 1, dz));
    }
  }

  // Layer 1 is a 3x3 with 20% corner removal.
  for dz in -1_i32..=1 {
    for dx in -1_i32..=1 {
      if dx == 0 && dz == 0 {
        continue;
      }
      if dx.abs() == 1 && dz.abs() == 1 && rng.range(0..100) < 20 {
        continue;
      }
      supported_small_leaf(tree, world, pos + Pos::new(dx, 4, dz));
    }
  }

  // Layer 2 is a plus; its center is a crown leaf above a 5-log pole.
  if height == 5 {
    supported_small_leaf(tree, world, pos + Pos::new(0, 5, 0));
  }
  for (dx, dz) in SIDES {
    supported_small_leaf(tree, world, pos + Pos::new(dx, 5, dz));
  }

  if height == 5 {
    // Layer 3 is a single crown leaf. Layer 4 extends it 50% of the
    // time. A side leaf on layer 3 has its own 50% roll and needs the
    // matching layer-2 leaf underneath.
    supported_small_leaf(tree, world, pos + Pos::new(0, 6, 0));
    if rng.range(0..100) < 50 && supported_small_leaf(tree, world, pos + Pos::new(0, 7, 0)) {
      let (dx, dz) = SIDES[rng.range(0..4) as usize];
      if rng.range(0..100) < 50 && world.get(pos + Pos::new(dx, 5, dz)) == tree.leaves {
        supported_small_leaf(tree, world, pos + Pos::new(dx, 6, dz));
      }
    }
  } else {
    // The 6-log form has a plus on layer 3. Each arm can be omitted
    // independently 20% of the time.
    supported_small_leaf(tree, world, pos + Pos::new(0, 6, 0));
    for (dx, dz) in SIDES {
      if rng.range(0..100) >= 20 {
        supported_small_leaf(tree, world, pos + Pos::new(dx, 6, dz));
      }
    }

    // Layer 4 has the crown center and one randomly oriented arm.
    supported_small_leaf(tree, world, pos + Pos::new(0, 7, 0));
    let first_side = rng.range(0..4) as usize;
    let (dx, dz) = SIDES[first_side];
    supported_small_leaf(tree, world, pos + Pos::new(dx, 7, dz));

    // Layer 5 is optional. Its extra layer-4 side has a separate 50%
    // roll and only places if the matching layer-3 leaf exists.
    if rng.range(0..100) < 50 && supported_small_leaf(tree, world, pos + Pos::new(0, 8, 0)) {
      let extra_side = (first_side + rng.range(1..4) as usize) % 4;
      let (dx, dz) = SIDES[extra_side];
      if rng.range(0..100) < 50 && world.get(pos + Pos::new(dx, 6, dz)) == tree.leaves {
        supported_small_leaf(tree, world, pos + Pos::new(dx, 7, dz));
      }
    }
  }
}

fn place_cedar_roots(tree: &CedarTree, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
  let ground_y = -1;
  for (dx, dz) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
    if rng.range(0..100) >= 28 {
      continue;
    }
    let buried = rng.range(0..100) < 70;
    let root_pos = pos + Pos::new(dx, if buried { -1 } else { 0 }, dz);
    let soil = world.get(pos + Pos::new(dx, -1, dz));
    let below = world.get(pos + Pos::new(dx, -2, dz));
    if !tree.ground_mix.contains(soil)
      || below == block![air]
      || below == block![water]
      || different_wood_adjacent(world, root_pos, WoodSpecies::Cedar)
      || (!buried && world.get(root_pos) != block![air] && !tree.plant_mix.contains(world.get(root_pos))) {
      continue;
    }

    let axis = if dx != 0 { "x" } else { "z" };
    let mut root = if rng.range(0..100) < 35 {
      block![rgen:covered_cedar_log[axis = "none"]]
    } else {
      tree.trunk
    };
    root.set_prop("axis", axis);
    world.set(root_pos, root);

    // A few roots grow two or three blocks from the trunk. They turn
    // diagonally rather than always following a straight cardinal line.
    if rng.range(0..100) < 24 {
      let bend = if rng.range(0..2) == 0 { -1 } else { 1 };
      let (turn_x, turn_z) = (-dz * bend, dx * bend);
      let length = if rng.range(0..2) == 0 { 2 } else { 3 };
      for step in 2..=length {
        let outer = pos + Pos::new(dx * step + turn_x, -1, dz * step + turn_z);
        let outer_soil = world.get(outer);
        let below_outer = world.get(outer + Pos::new(0, -1, 0));
        if !tree.ground_mix.contains(outer_soil)
          || below_outer == block![air]
          || below_outer == block![water]
          || different_wood_adjacent(world, outer, WoodSpecies::Cedar)
        {
          break;
        }
        let mut tip = if rng.range(0..100) < 70 {
          block![rgen:covered_cedar_log[axis = "none"]]
        } else {
          tree.trunk
        };
        tip.set_prop("axis", axis);
        world.set(outer, tip);
      }
    }
  }

  // Patch a few exposed soil blocks close to the roots with podzol.
  for dz in -1..=1 {
    for dx in -1..=1 {
      if dx == 0 && dz == 0 || rng.range(0..100) >= 30 {
        continue;
      }
      let soil_pos = pos + Pos::new(dx, ground_y, dz);
      let soil = world.get(soil_pos);
      if soil == block![grass] || soil == block![dirt] {
        world.set(soil_pos, block![dirt[variant = "podzol"]]);
      }
    }
  }
}
