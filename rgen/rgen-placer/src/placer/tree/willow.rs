use rgen_base::{BlockFilter, BlockState, Pos, block};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};
use crate::placer::wood_clearance::{any_wood_adjacent, is_wood_log};

/// Concrete test willow with a short core and three rotated branch types.
pub struct WillowTree {
  pub ground:        BlockFilter,
  pub plantable:     BlockFilter,
  pub plants:        BlockFilter,
  pub log:           BlockState,
  pub leaves:        BlockState,
  pub avg_per_chunk: f64,
}

impl Placer for WillowTree {
  fn radius(&self) -> u8 { 4 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let ground = pos + Pos::new(0, -1, 0);
    if !self.plantable.contains(world.get(ground)) { return Err(UndoError); }
    for y in 0..4 {
      let trunk_pos = pos + Pos::new(0, y, 0);
      if !self.replaceable(world, trunk_pos) || any_wood_adjacent(world, trunk_pos) {
        return Err(UndoError);
      }
    }
    // Concrete palette for later visualization: lime/green core; pink, brown,
    // red, orange A/B variants; white, black, blue, lightblue diagonal variants.
    let mut own_logs = Vec::new();
    for y in 0..4 {
      let trunk_pos = pos + Pos::new(0, y, 0);
      world.set(trunk_pos, self.log);
      own_logs.push(trunk_pos);
    }

    // A, B and 1 occupy a fixed diagonal layout, rotated as a group per tree.
    let rotation = rng.range(0..4);
    let a_variant = rng.range(0..4);
    let b_variant = (a_variant + rng.range(1..4)) % 4;
    let dirs = [(-1, 0), (0, -1), (1, 1)];
    let mut branch_ends = Vec::new();
    for i in 0..3 {
      let (mut dx, mut dz) = dirs[i];
      for _ in 0..rotation { (dx, dz) = (-dz, dx); }
      let origin = pos + Pos::new(0, 3, 0);
      let variant = if i == 0 { a_variant } else if i == 1 { b_variant } else { rng.range(0..4) };
      if i < 2 {
        let color = self.log;
        let start = origin + Pos::new(dx, if variant == 2 { 1 } else { 0 }, dz);
        self.place_branch_log(world, start, color, &mut own_logs);
        branch_ends.push(start);
        if variant == 3 {
          let (mut other_x, mut other_z) = dirs[1 - i];
          for _ in 0..rotation { (other_x, other_z) = (-other_z, other_x); }
          // Combine our outward direction with the direction away from the
          // other branch: both horizontal coordinates change for a diagonal.
          let away = Pos::new(dx - other_x, 0, dz - other_z);
          for branch_pos in [
            start + Pos::new(0, 1, 0),
            start + away + Pos::new(0, 2, 0),
            start + away + Pos::new(0, 3, 0),
          ] {
            self.place_branch_log(world, branch_pos, color, &mut own_logs);
            branch_ends.push(branch_pos);
          }
          continue;
        }
        let path: &[(i32, i32)] = match variant {
          0 => &[(0, 1), (0, 2), (1, 3)],
          1 => &[(1, 0), (2, 1)],
          2 => &[(0, 1), (1, 2)],
          _ => &[],
        };
        for &(side, up) in path {
          let branch_pos = start + Pos::new(dx * side, up, dz * side);
          self.place_branch_log(world, branch_pos, color, &mut own_logs);
          branch_ends.push(branch_pos);
        }
      } else {
        let colors = [self.log; 4];
        let elevated_start = variant >= 2;
        let start = origin + Pos::new(dx, i32::from(elevated_start), dz);
        self.place_branch_log(world, start, colors[variant as usize], &mut own_logs);
        branch_ends.push(start);
        let count = match variant {
          0 | 1 | 3 => 2,
          2 => 1,
          _ => unreachable!(),
        };
        for step in 1..=count {
          let branch_pos = start + Pos::new(dx * step, step, dz * step);
          self.place_branch_log(world, branch_pos, colors[variant as usize], &mut own_logs);
          branch_ends.push(branch_pos);
        }
      }
    }
    for end in branch_ends {
      if world.get(end + Pos::new(0, 1, 0)) != block![air] { continue; }
      for (x, z) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
        let leaf = end + Pos::new(x, 0, z);
        if self.replaceable(world, leaf) { world.set(leaf, self.leaves); }
      }
      for (x, z) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
        let corner = end + Pos::new(x, 0, z);
        if rng.range(0..5) == 0 && self.replaceable(world, corner) {
          world.set(corner, self.leaves);
        }
      }
      let top = end + Pos::new(0, 1, 0);
      if self.replaceable(world, top) { world.set(top, self.leaves); }
      for (x, z) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let hang = match rng.range(0..4) { 0 => 0, 1 | 2 => 1, _ => 3 };
        for depth in 1..=hang {
          let leaf = end + Pos::new(x, -depth, z);
          if self.replaceable(world, leaf) { world.set(leaf, self.leaves); } else { break; }
        }
        let upper_arm = end + Pos::new(x, 1, z);
        if rng.range(0..2) == 0 && self.replaceable(world, upper_arm) {
          world.set(upper_arm, self.leaves);
        }
      }
    }
    Ok(())
  }
}

impl WillowTree {
  fn place_branch_log(&self, world: &mut PartialWorld, pos: Pos, log: BlockState, own_logs: &mut Vec<Pos>) {
    if !self.replaceable(world, pos) {
      return;
    }
    for (x, z) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
      let neighbor = pos + Pos::new(x, 0, z);
      if !own_logs.contains(&neighbor) && is_wood_log(world.get(neighbor)) {
        return;
      }
    }
    world.set(pos, log);
    own_logs.push(pos);
  }

  fn replaceable(&self, world: &PartialWorld, pos: Pos) -> bool {
    let block = world.get(pos);
    block == block![air] || self.plants.contains(block)
  }
}

/* test tree
use rgen_base::{BlockFilter, BlockState, Pos, block};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

/// Builds a varied willow skeleton with leaf caps around the branch tips.
pub struct WillowTree {
  pub ground:        BlockFilter,
  pub plantable:     BlockFilter,
  pub plants:        BlockFilter,
  pub log:           BlockState,
  pub leaves:        BlockState,
  pub avg_per_chunk: f64,
}

impl Placer for WillowTree {
  fn radius(&self) -> u8 { 4 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let tall = rng.range(0..2) == 0;
    let height = if tall { rng.range(4..=5) } else { rng.range(2..=3) };
    if pos.y <= 1 || pos.y + height + 8 >= 255 {
      return Err(UndoError);
    }

    let ground_pos = pos + Pos::new(0, -1, 0);
    if !self.plantable.contains(world.get(ground_pos)) {
      return Err(UndoError);
    }

    // Unlike TallBush, a willow may root beside a channel. Surrounding
    // ground must be suitable land or water, but the trunk needs land.
    for x in -1_i32..=1 {
      for z in -1_i32..=1 {
        let ground = world.get(ground_pos + Pos::new(x, 0, z));
        if ground != block![water] && !self.ground.contains(ground) {
          return Err(UndoError);
        }
      }
    }

    // The upright trunk is required; skip the whole tree if it is blocked.
    for y in 0..height {
      if !self.can_replace(world, pos + Pos::new(0, y, 0)) {
        return Err(UndoError);
      }
    }
    for y in 0..height {
      world.set(pos + Pos::new(0, y, 0), block![concrete[color = "black"]]);
    }

    let mut directions = [
      (0, -1), (1, -1), (1, 0), (1, 1),
      (0, 1), (-1, 1), (-1, 0), (-1, -1),
    ];
    rng.shuffle(&mut directions);

    // Short, uneven roots sit at the foot of the trunk. Keep them on land.
    let root_count = if tall { rng.range(2..=3) } else { rng.range(1..=2) };
    for &(dx, dz) in directions.iter().take(root_count as usize) {
      let root = pos + Pos::new(dx, 0, dz);
      if self.ground.contains(world.get(root + Pos::new(0, -1, 0))) {
        self.place_block(world, root, block![concrete[color = "black"]]);
      }
    }

    // Arms climb partway along their outward run, ending in a bright tip.
    let branch_count = if tall { rng.range(2..=3) } else { rng.range(2..=4) };
    let mut endpoints: Vec<Pos> = Vec::new();
    let mut placed_directions: Vec<(i32, i32)> = Vec::new();
    let mut leaves: Vec<Pos> = Vec::new();
    let mut hangers = Vec::new();
    let branch_colors = [
      (block![concrete[color = "blue"]], block![concrete[color = "lightblue"]]),
      (block![concrete[color = "purple"]], block![concrete[color = "magenta"]]),
      (block![concrete[color = "green"]], block![concrete[color = "lime"]]),
      (block![concrete[color = "red"]], block![concrete[color = "orange"]]),
    ];
    for (branch_index, &(dx, dz)) in directions.iter().take(branch_count as usize).enumerate() {
      if placed_directions.len() == 1 {
        let (first_x, first_z) = placed_directions[0];
        if dx == -first_x && dz == -first_z {
          continue;
        }
      }
      let (arm_color, tip_color) = branch_colors[branch_index];
      let attach_y = if branch_index == 0 {
        height - 1
      } else if tall {
        rng.range(2..=height - 2)
      } else {
        1
      };
      let length = 2;
      let rise_step = rng.range(1..=2);
      let branch_height = |step: i32| {
        let rise = (step - rise_step + 1).max(0);
        attach_y + rise + rise / 2
      };
      let endpoint_y = branch_height(length);
      let endpoint = pos + Pos::new(dx * length, endpoint_y, dz * length);
      if endpoints.iter().any(|other| {
        let x = endpoint.x - other.x;
        let z = endpoint.z - other.z;
        x * x + z * z < 4
      }) {
        continue;
      }
      if (1..=length).any(|step| {
        let y = branch_height(step);
        let previous_y = if step == 1 { attach_y } else { branch_height(step - 1) };
        !self.can_replace(world, pos + Pos::new(dx * step, y, dz * step))
          || (y - previous_y > 1
            && !self.can_replace(world, pos + Pos::new(dx * (step - 1), previous_y + 1, dz * (step - 1))))
      }) {
        continue;
      }
      let mut branch_blocks = Vec::new();
      for step in 1..=length {
        let y = branch_height(step);
        let previous_y = if step == 1 { attach_y } else { branch_height(step - 1) };
        if y - previous_y > 1 {
          let bridge = pos + Pos::new(dx * (step - 1), previous_y + 1, dz * (step - 1));
          self.place_block(world, bridge, arm_color);
          branch_blocks.push(bridge);
        }
        let branch = pos + Pos::new(dx * step, y, dz * step);
        self.place_block(world, branch, if step == length { tip_color } else { arm_color });
        branch_blocks.push(branch);
      }
      endpoints.push(endpoint);
      placed_directions.push((dx, dz));
      let cores: Vec<_> = branch_blocks.iter().copied()
        .filter(|p| *p == endpoint || p.y == endpoint.y - 1).collect();
      let cap = finish_cap(cap_leaves(&branch_blocks, endpoint), &cores, || rng.range(0..100) < 45);
      hangers.extend(cap_edges(&cap, &cores).into_iter().map(|edge| (edge, rng.range(1..=2))));
      leaves.extend(cap);
    }

    let trunk_top = pos + Pos::new(0, height - 1, 0);
    let cap = finish_cap(cap_leaves(&[trunk_top], trunk_top), &[trunk_top], || rng.range(0..100) < 45);
    hangers.extend(cap_edges(&cap, &[trunk_top]).into_iter().map(|edge| (edge, rng.range(1..=2))));
    leaves.extend(cap);

    for leaf in leaves {
      self.place_block(world, leaf, self.leaves);
    }

    for (edge, length) in hangers {
      for depth in 1..=length {
        let leaf = edge + Pos::new(0, -depth, 0);
        if leaf.y <= 0 { break; }
        if world.get(leaf) == self.leaves { continue; }
        if !self.place_block(world, leaf, self.leaves) { break; }
      }
    }

    Ok(())
  }
}

impl WillowTree {
  fn can_replace(&self, world: &PartialWorld, pos: Pos) -> bool {
    let current = world.get(pos);
    current == block![air] || self.plants.contains(current)
  }

  fn place_block(&self, world: &mut PartialWorld, pos: Pos, state: BlockState) -> bool {
    if !self.can_replace(world, pos) {
      return false;
    }
    world.set(pos, state);
    true
  }
}

/// Union of the horizontal 3x3 neighborhoods of the tip and the actual
/// branch blocks exactly one level below it. Never invent a core below the tip.
fn cap_leaves(branch: &[Pos], tip: Pos) -> Vec<Pos> {
  let mut leaves = Vec::new();
  for core in branch.iter().copied().filter(|p| *p == tip || p.y == tip.y - 1) {
    for x in -1..=1 {
      for z in -1..=1 {
        let leaf = core + Pos::new(x, 0, z);
        if !branch.contains(&leaf) && !leaves.contains(&leaf) {
          leaves.push(leaf);
        }
      }
    }
  }
  leaves
}

// Treat a merged cap as one shape: a leaf beside any core is an edge,
// even when it is diagonal to a neighboring core. Only outer corners roll.
fn finish_cap(mut leaves: Vec<Pos>, cores: &[Pos], mut remove_corner: impl FnMut() -> bool) -> Vec<Pos> {
  leaves.retain(|leaf| {
    let edge = cores.iter().any(|core| {
      leaf.y == core.y && (leaf.x - core.x).abs() + (leaf.z - core.z).abs() <= 1
    });
    edge || !remove_corner()
  });
  for core in cores {
    for (x, z) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
      let top = *core + Pos::new(x, 1, z);
      if !leaves.contains(&top) { leaves.push(top); }
    }
  }
  leaves
}

/// Exposed edges of the cap's core layers; the plus-shaped tops do not hang.
fn cap_edges(leaves: &[Pos], cores: &[Pos]) -> Vec<Pos> {
  leaves.iter().copied().filter(|leaf| {
    !cores.contains(leaf) && cores.iter().any(|core| core.y == leaf.y)
      && [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().any(|(x, z)| {
        let neighbor = *leaf + Pos::new(x, 0, z);
        !leaves.contains(&neighbor) && !cores.contains(&neighbor)
      })
  }).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn merged_cap_removes_only_outer_corners_and_tops_every_core() {
    let a = Pos::new(0, 10, 0);
    let b = Pos::new(1, 10, 0);
    let mut footprint = cap_leaves(&[a], a);
    footprint.extend(cap_leaves(&[b], b));
    footprint.retain(|p| *p != a && *p != b);
    let mut unique = Vec::new();
    for p in footprint { if !unique.contains(&p) { unique.push(p); } }
    let mut rolls = 0;
    let leaves = finish_cap(unique, &[a, b], || { rolls += 1; true });
    assert_eq!(rolls, 4);
    assert_eq!(leaves.len(), 14);
    for core in [a, b] {
      for (x, z) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
        assert!(leaves.contains(&(core + Pos::new(x, 1, z))));
      }
      assert!(leaves.contains(&(core + Pos::new(0, 0, 1))));
      assert!(leaves.contains(&(core + Pos::new(0, 0, -1))));
    }
  }

  #[test]
  fn cap_matches_two_lower_cores_diagram() {
    let tip = Pos::new(0, 10, 0);
    let a = Pos::new(1, 9, 0);
    let b = Pos::new(2, 9, 0);
    let d = Pos::new(3, 8, 0);
    let leaves = cap_leaves(&[tip, a, b, d], tip);
    let mut expected = Vec::new();
    for x in -1..=1 {
      for z in -1..=1 {
        let p = Pos::new(x, 10, z);
        if p != tip { expected.push(p); }
      }
    }
    // Two adjacent cores give a 4x3 footprint, with their blocks preserved.
    for x in 0..=3 {
      for z in -1..=1 {
        let p = Pos::new(x, 9, z);
        if p != a && p != b { expected.push(p); }
      }
    }
    assert_eq!(leaves.len(), 18);
    assert!(expected.iter().all(|p| leaves.contains(p)));
  }

  #[test]
  fn block_two_levels_below_tip_is_not_a_lower_core() {
    let tip = Pos::new(0, 10, 0);
    let a = Pos::new(1, 9, 0);
    let b = Pos::new(2, 8, 0);
    let leaves = cap_leaves(&[tip, a, b], tip);
    assert_eq!(leaves.iter().filter(|p| p.y == 9).count(), 8);
    assert!(leaves.iter().all(|p| p.y >= 9));
    assert!(!leaves.contains(&Pos::new(3, 9, 0)));
  }
}

*/
