use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};
use crate::placer::wood_clearance::any_wood_adjacent;

pub enum MountainOakShape { Stressed, Starved }

pub struct MountainOak {
  pub shape: MountainOakShape,
  pub place_above: BlockFilter,
  pub avg_per_chunk: f64,
}

impl Placer for MountainOak {
  fn radius(&self) -> u8 { 3 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if pos.y <= 1 || pos.y + 8 >= 255
      || !self.place_above.contains(world.get(pos + Pos::new(0, -1, 0)))
      || super::basic::trunk_in_3x3(world, pos, block![log[variant = "oak", axis = "y"]])
      || super::basic::boulder_in_3x3(world, pos) {
      return Err(UndoError);
    }
    let log = block![log[variant = "oak", axis = "y"]];
    let leaves = block![leaves[variant = "oak", check_decay = false, decayable = false]];
    let stressed = matches!(self.shape, MountainOakShape::Stressed);
    let trunk_height = if stressed { rng.range(2..=3) } else { 2 };
    let mut plan = Vec::new();
    for y in 0..trunk_height {
      plan.push((pos + Pos::new(0, y, 0), log));
    }
    let base = trunk_height;
    if stressed {
      add_plus(&mut plan, pos, base, leaves, 0, rng);
      plan.push((pos + Pos::new(0, base, 0), log));
      for layer in 1..=3 {
        add_square(&mut plan, pos, base + layer, leaves, 25, rng);
        if layer <= 2 { plan.push((pos + Pos::new(0, base + layer, 0), log)); }
      }
      add_plus(&mut plan, pos, base + 4, leaves, 0, rng);
      for x in [-1, 1] {
        for z in [-1, 1] {
          if rng.range(0..100) < 15 {
            plan.push((pos + Pos::new(x, base + 4, z), leaves));
          }
        }
      }
    } else {
      add_plus(&mut plan, pos, base, leaves, 25, rng);
      plan.push((pos + Pos::new(0, base, 0), log));
      add_square(&mut plan, pos, base + 1, leaves, 0, rng);
      plan.push((pos + Pos::new(0, base + 1, 0), log));
      for (x, z) in [(2, 0), (-2, 0), (0, 2), (0, -2)] {
        if rng.range(0..100) < 10 {
          plan.push((pos + Pos::new(x, base + 1, z), leaves));
        }
      }
      for layer in 2..=3 { add_square(&mut plan, pos, base + layer, leaves, 25, rng); }
      add_plus(&mut plan, pos, base + 4, leaves, 0, rng);
      plan.push((pos + Pos::new(0, base + 5, 0), leaves));
    }

    plan.sort_by_key(|(p, state)| (p.x, p.y, p.z, state.block != log.block));
    plan.dedup_by_key(|(p, _)| (p.x, p.y, p.z));
    if plan.iter().any(|&(p, state)| state.block == log.block
      && any_wood_adjacent(world, p)) {
      return Err(UndoError);
    }
    let blocked = plan.iter().filter(|(p, _)| world.get(*p) != block![air]).count();
    if blocked * 100 >= plan.len() * 5 { return Err(UndoError); }
    for (p, state) in plan {
      if world.get(p) == block![air] { world.set(p, state); }
    }
    Ok(())
  }
}

fn add_plus(plan: &mut Vec<(Pos, BlockState)>, pos: Pos, y: i32, leaf: BlockState, remove: i32, rng: &mut Rng) {
  plan.push((pos + Pos::new(0, y, 0), leaf));
  for (x, z) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
    if rng.range(0..100) >= remove { plan.push((pos + Pos::new(x, y, z), leaf)); }
  }
}

fn add_square(plan: &mut Vec<(Pos, BlockState)>, pos: Pos, y: i32, leaf: BlockState, corner_remove: i32, rng: &mut Rng) {
  for x in -1_i32..=1 {
    for z in -1_i32..=1 {
      if x.abs() == 1 && z.abs() == 1 && rng.range(0..100) < corner_remove { continue; }
      plan.push((pos + Pos::new(x, y, z), leaf));
    }
  }
}
