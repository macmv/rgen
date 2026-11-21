use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};
use crate::placer::wood_clearance::any_wood_adjacent;

/// Cherry tree with stepped limbs and vanilla cherry crowns.
pub struct CherryV2 {
  pub place_above: BlockFilter,
  pub avg_per_chunk: f64,
}

impl Default for CherryV2 {
  fn default() -> Self {
    Self { place_above: [block![grass], block![dirt]].into(), avg_per_chunk: 1.0 }
  }
}

impl Placer for CherryV2 {
  fn radius(&self) -> u8 { 9 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if pos.y <= 1 || pos.y + 15 >= 255 || !self.place_above.contains(world.get(pos + Pos::new(0,-1,0))) {
      return Err(UndoError);
    }
    let log = block![rgen:log[variant = "cherry", axis = "y"]];
    let leaves = block![rgen:leaves[variant = "cherry", check_decay = false, decayable = true]];
    let mut planned: Vec<(Pos, BlockState)> = Vec::new();
    let height = rng.range(4..=5);
    for y in 0..height {
      planned.push((pos + Pos::new(0, y, 0), log));
    }
    let top = pos + Pos::new(0, height - 1, 0);
    let sign = *rng.choose(&[-1, 1]);
    let (dx, dz) = if rng.range(0..2) == 0 { (sign, 0) } else { (0, sign) };
    let sideways_log = if dx != 0 {
      block![rgen:log[variant = "cherry", axis = "x"]]
    } else {
      block![rgen:log[variant = "cherry", axis = "z"]]
    };
    let (b, c) = match rng.range(0..8) {
      0..=1 => (false, false),
      2..=4 => { let b = rng.range(0..2) == 0; (b, !b) }
      _ => (true, true),
    };
    for (start, x, z) in [(top, dx, dz), (top + Pos::new(0, -1, 0), -dx, -dz)]
      .into_iter().take(if b { 2 } else { 1 }) {
      let mut tip = start;
      for (step, length) in [(Pos::new(x,0,z), rng.range(1..=4)),
        (Pos::new(0,1,0), rng.range(1..=3)), (Pos::new(x,0,z), rng.range(1..=2)),
        (Pos::new(0,1,0), rng.range(0..=1))] {
        let segment_log = if step.y == 0 { sideways_log } else { log };
        // A solo arm takes one sideways step before and after its rise.
        let length = if !b && !c && step.y == 0 { 1 } else { length };
        for _ in 0..length {
          tip = tip + step;
          planned.push((tip, segment_log));
        }
      }
      crown(&mut planned, rng, tip, leaves);
    }
    if c {
      for y in 1..=3 { planned.push((top + Pos::new(0,y,0), log)); }
      crown(&mut planned, rng, top + Pos::new(0,3,0), leaves);
    }
    // Resolve overlapping crowns and logs before measuring obstructions.
    planned.sort_by_key(|(p,state)| (p.x,p.y,p.z, state.block != log.block));
    planned.dedup_by_key(|(p,_)| (p.x,p.y,p.z));
    if planned.iter().any(|&(p, state)| state.block == log.block
      && any_wood_adjacent(world, p)) {
      return Err(UndoError);
    }
    let mut blocked = 0;
    for &(target, state) in &planned {
      let existing = world.get(target);
      if existing.block_kind() == block![rgen:bamboo].block { return Err(UndoError); }
      if state.block == log.block && existing != block![air] && existing.block_kind() != leaves.block {
        return Err(UndoError);
      }
      if state == leaves && existing != block![air] && existing.block_kind() != leaves.block {
        blocked += 1;
      }
    }
    if blocked * 100 > planned.len() * 15 { return Err(UndoError); }
    for (target,state) in planned {
      let existing = world.get(target);
      if existing == block![air] || (state.block == log.block && existing.block_kind() == leaves.block) {
        world.set(target,state);
      }
    }
    Ok(())
  }
}

/// Vanilla cherry uses a foliage node one block above the final branch log.
/// These rows follow its height 5, radius 4, single-trunk configuration.
fn crown(plan: &mut Vec<(Pos, BlockState)>, rng: &mut Rng, tip: Pos, leaves: BlockState) {
  let node = tip + Pos::new(0, 1, 0);
  for (y, radius) in [(2, 1_i32), (1, 2), (0, 3), (-1, 3), (-2, 2)] {
    for x in -radius..=radius {
      for z in -radius..=radius {
        let rim = x.abs() == radius || z.abs() == radius;
        if y == -1 && rim && rng.range(0..4) == 0 { continue; }
        let corner = x.abs() == radius && z.abs() == radius;
        if radius > 2 && corner { continue; }
        let round = if radius > 2 { x.abs() + z.abs() > 2*radius - 2 } else { corner };
        if round && rng.range(0..4) == 0 { continue; }
        plan.push((node + Pos::new(x, y, z), leaves));
      }
    }

    if y >= 0 { continue; }
    // Walk the perimeter once. A leaf placed by an overlapping crown can
    // also support a hanger, as in the vanilla foliage placement pass.
    for (dx, dz) in [(1,0), (0,1), (-1,0), (0,-1)] {
      for offset in -radius..radius {
        let x = dx*radius - dz*offset;
        let z = dz*radius + dx*offset;
        let support = node + Pos::new(x, y, z);
        if !plan.iter().any(|&(p, state)| p == support && state == leaves) { continue; }
        for (drop, odds) in [(1, 6), (2, 3)] {
          let distance = x.abs() + z.abs() + (y - drop + 1).abs();
          if distance >= 7 || rng.range(0..odds) != 0 { break; }
          plan.push((node + Pos::new(x, y-drop, z), leaves));
        }
      }
    }
  }
}
