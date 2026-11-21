use rgen_base::{BlockFilter, BlockInfo, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};
use crate::placer::wood_clearance::any_wood_adjacent;

pub struct AcaciaTree {
  pub place_above:   BlockFilter,
  pub avg_per_chunk: f64,
  pub grass_pants:   bool,
}

/// Leafless Acacia-like branches with covered bark on every exposed log end.
pub struct FloodedAcaciaTree {
  pub place_above:   BlockFilter,
  pub avg_per_chunk: f64,
}

/// A two-block stump with a covered top log.
pub struct TallStump {
  pub place_above:   BlockFilter,
  pub log:           BlockState,
  pub covered_log:   BlockState,
  pub avg_per_chunk: f64,
}

impl Default for AcaciaTree {
  fn default() -> Self {
    AcaciaTree { place_above: block![grass].into(), avg_per_chunk: 6.0, grass_pants: false }
  }
}

impl Default for FloodedAcaciaTree {
  fn default() -> Self {
    FloodedAcaciaTree { place_above: block![grass].into(), avg_per_chunk: 1.0 }
  }
}

impl Default for TallStump {
  fn default() -> Self {
    TallStump {
      place_above: block![grass].into(),
      log: block![log2[variant = "acacia", axis = "y"]],
      covered_log: block![rgen:covered_acacia_log[axis = "y"]],
      avg_per_chunk: 0.9,
    }
  }
}

impl Placer for AcaciaTree {
  fn radius(&self) -> u8 { 4 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    place_acacia_tree(world, rng, pos, &self.place_above, 1, 6, self.grass_pants, false, true)
  }
}

impl Placer for FloodedAcaciaTree {
  fn radius(&self) -> u8 { 4 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    place_acacia_tree(world, rng, pos, &self.place_above, 3, 5, false, true, false)
  }
}

impl Placer for TallStump {
  fn radius(&self) -> u8 { 1 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, _rng: &mut Rng, pos: Pos) -> Result {
    let below = pos + Pos::new(0, -1, 0);
    let above = pos + Pos::new(0, 1, 0);
    if pos.y <= 1
      || pos.y + 1 >= 255
      || !self.place_above.contains(world.get(below))
      || world.get(pos) != block![air]
      || world.get(above) != block![air]
      || any_wood_adjacent(world, pos)
      || any_wood_adjacent(world, above)
    {
      return Err(UndoError);
    }

    world.set(pos, self.log);
    world.set(above, self.covered_log);
    Ok(())
  }
}

fn place_acacia_tree(
  world: &mut PartialWorld,
  rng: &mut Rng,
  pos: Pos,
  place_above: &BlockFilter,
  min_trunk_blocks: u8,
  max_trunk_blocks: u8,
  grass_pants: bool,
  cover_all_exposed_logs: bool,
  place_leaves: bool,
) -> Result {
    let vertical_log = block![log2[variant = "acacia", axis = "y"]];

    if !place_above.contains(world.get(pos + Pos::new(0, -1, 0))) || pos.y <= 1 {
      return Err(UndoError);
    }

    let directions = [
      Pos::new(1, 0, 0),
      Pos::new(-1, 0, 0),
      Pos::new(0, 0, 1),
      Pos::new(0, 0, -1),
    ];
    let has_surface_in_plus = directions.iter().any(|direction| {
      world.get(pos + *direction) != block![air]
    });

    // The base block is always present; this height is the offset of the top
    // block above it. Nearby surface blocks require a taller trunk to lift the
    // diagonal branches clear of the ground.
    // The selected value counts the trunk's base block, so the stored offset
    // to its top is one less.
    let mut trunk_height = rng.range(i32::from(min_trunk_blocks)..=i32::from(max_trunk_blocks)) - 1;
    if has_surface_in_plus {
      trunk_height = trunk_height.max(3);
    }
    if trunk_height > 9 || pos.y + 10 >= 255 {
      return Err(UndoError);
    }

    let mut pieces = Vec::new();
    for y in 0..=trunk_height {
      if !queue_piece(&mut pieces, pos + Pos::new(0, y, 0), vertical_log) {
        return Err(UndoError);
      }
    }
    let trunk_top = pos + Pos::new(0, trunk_height, 0);

    // Branch A always grows diagonally upward for one to three blocks. Short
    // trunks use its maximum diagonal and the full two-block upshoot.
    let direction_a = *rng.choose(&directions);
    let maximize_a = trunk_height <= 1;
    let a_diagonal = if maximize_a { 3 } else { rng.range(1..=3) };
    let mut a_height = trunk_height;
    let mut a_end = trunk_top;
    for _ in 0..a_diagonal {
      let next = a_end + direction_a + Pos::new(0, 1, 0);
      if next.y - pos.y > 9 {
        break;
      }
      if !queue_piece(&mut pieces, next, vertical_log) {
        return Err(UndoError);
      }
      a_height = next.y - pos.y;
      a_end = next;
    }

    if a_diagonal > 1 && (maximize_a || rng.range(0..=99) < 30) {
      for _ in 0..2 {
        let next = a_end + Pos::new(0, 1, 0);
        if a_height + 1 > 9 {
          break;
        }
        if !queue_piece(&mut pieces, next, vertical_log) {
          return Err(UndoError);
        }
        a_height += 1;
        a_end = next;
      }
    }

    // Ensure branch A rises at least three blocks above its trunk start.
    // This minimum-height extension is independent of the random upshoot.
    if a_height - trunk_height < 3 {
      for _ in 0..2 {
        let next = a_end + Pos::new(0, 1, 0);
        if a_height + 1 > 9 {
          break;
        }
        if !queue_piece(&mut pieces, next, vertical_log) {
          return Err(UndoError);
        }
        a_height += 1;
        a_end = next;
      }
    }
    // Branch B is omitted 30% of the time and never grows from a short trunk.
    let mut b_end_point = None;
    if trunk_height > 1 && rng.range(0..=99) >= 30 {
      let other_directions: Vec<Pos> = directions.iter().copied().filter(|d| *d != direction_a).collect();
      let direction_b = *rng.choose(&other_directions);
      let b_diagonal = rng.range(1..=2);
      let start_y = rng.range(1..=trunk_height - 1);
      let mut b_height = start_y;
      let mut b_end = pos + Pos::new(0, start_y, 0);

      for _ in 0..b_diagonal {
        let next = b_end + direction_b + Pos::new(0, 1, 0);
        if b_height + 1 > 9 {
          break;
        }
        if !queue_piece(&mut pieces, next, vertical_log) {
          return Err(UndoError);
        }
        b_height += 1;
        b_end = next;
      }

      if b_diagonal > 1 && rng.range(0..=99) < 25 {
        let next = b_end + Pos::new(0, 1, 0);
        if b_height + 1 <= 9 {
          if !queue_piece(&mut pieces, next, vertical_log) {
            return Err(UndoError);
          }
          b_height += 1;
          b_end = next;
        }
      }
      b_end_point = Some(b_end);
    }

    if cover_all_exposed_logs {
      let exposed_log_ends: Vec<Pos> = pieces.iter()
        .filter_map(|(piece_pos, _)| {
          let above = *piece_pos + Pos::new(0, 1, 0);
          let generated_log_above = pieces.iter().any(|(candidate, _)| *candidate == above);
          (!generated_log_above && !is_log_block(world.get(above))).then_some(*piece_pos)
        })
        .collect();
      let covered_log = block![rgen:covered_acacia_log[axis = "y"]];
      for (piece_pos, state) in &mut pieces {
        if exposed_log_ends.contains(piece_pos) {
          *state = covered_log;
        }
      }
    } else if grass_pants {
      let covered_log = block![rgen:covered_acacia_log[axis = "y"]];
      let trunk_pos = pos;
      let above = trunk_pos + Pos::new(0, 1, 0);
      let tree_above = pieces.iter().any(|(piece_pos, _)| *piece_pos == above);
      if !tree_above && world.get(above) == block![air] {
        if let Some((_, state)) = pieces.iter_mut().find(|(piece_pos, _)| *piece_pos == trunk_pos) {
          *state = covered_log;
        }
      }
    }

    // Validate every trunk and branch before writing anything. The
    // tree may replace existing leaves or trunk blocks, but nothing else.
    if pieces.iter().any(|(piece_pos, _)| {
      !is_replaceable_tree_block(world.get(*piece_pos))
        || any_wood_adjacent(world, *piece_pos)
    }) {
      return Err(UndoError);
    }
    for (piece_pos, state) in pieces {
      world.set(piece_pos, state);
    }

    if place_leaves {
      let leaves = block![leaves2[variant = "acacia", check_decay = false]];
      build_branch_a_crown(world, a_end, leaves);
      if let Some(end_point) = b_end_point {
        build_branch_b_crown(world, end_point, leaves);
      }
    }

    Ok(())
}

fn build_branch_a_crown(world: &mut PartialWorld, end_point: Pos, leaves: BlockState) {
  // Broad 7x7 layer at the branch endpoint, with diagonal corners removed.
  for x in -3..=3_i32 {
    for z in -3..=3_i32 {
      if x.abs() == 3 && z.abs() == 3 {
        continue;
      }
      place_leaf_if_air(world, end_point + Pos::new(x, 0, z), leaves);
    }
  }

  // A 3x3 layer above the endpoint, plus one leaf at each edge midpoint.
  for x in -1..=1_i32 {
    for z in -1..=1_i32 {
      place_leaf_if_air(world, end_point + Pos::new(x, 1, z), leaves);
    }
  }
  for edge in [
    Pos::new(-2, 1, 0),
    Pos::new(2, 1, 0),
    Pos::new(0, 1, -2),
    Pos::new(0, 1, 2),
  ] {
    place_leaf_if_air(world, end_point + edge, leaves);
  }
}

fn build_branch_b_crown(world: &mut PartialWorld, end_point: Pos, leaves: BlockState) {
  // Broad 5x5 layer around the endpoint, with diagonal corners removed.
  for x in -2..=2_i32 {
    for z in -2..=2_i32 {
      if x.abs() == 2 && z.abs() == 2 {
        continue;
      }
      place_leaf_if_air(world, end_point + Pos::new(x, 0, z), leaves);
    }
  }

  // The smaller top layer sits directly above the endpoint.
  for x in -1..=1_i32 {
    for z in -1..=1_i32 {
      place_leaf_if_air(world, end_point + Pos::new(x, 1, z), leaves);
    }
  }
}

fn place_leaf_if_air(world: &mut PartialWorld, pos: Pos, leaves: BlockState) {
  if world.get(pos) == block![air] {
    world.set(pos, leaves);
  }
}

fn queue_piece(pieces: &mut Vec<(Pos, BlockState)>, pos: Pos, state: BlockState) -> bool {
  if pieces.iter().any(|(existing, _)| *existing == pos) {
    return false;
  }
  pieces.push((pos, state));
  true
}

fn is_replaceable_tree_block(state: BlockInfo<'_>) -> bool {
  let block = state.block_kind();
  block == block![air].block
    || block == block![log].block
    || block == block![log2].block
    || block == block![leaves].block
    || block == block![leaves2].block
    || block == block![rgen:log].block
    || block == block![rgen:log2].block
    || block == block![rgen:covered_acacia_log].block
    || block == block![rgen:leaves].block
    || block == block![rgen:leaves2].block
    || block == block![rgen:leaves3].block
}

fn is_log_block(state: BlockInfo<'_>) -> bool {
  let block = state.block_kind();
  block == block![log].block
    || block == block![log2].block
    || block == block![rgen:log].block
    || block == block![rgen:log2].block
    || block == block![rgen:covered_acacia_log].block
}
