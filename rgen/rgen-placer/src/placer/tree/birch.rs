use rgen_base::{BlockFilter, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};
use crate::placer::wood_clearance::any_wood_adjacent;

/// The original tall birch crown with clearance checks before placing blocks.
pub struct BasicBirch {
  pub place_above: BlockFilter,
  pub avg_per_chunk: f64,
}

impl Placer for BasicBirch {
  fn radius(&self) -> u8 { 3 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let height = rng.range(8..=9);
    if pos.y <= 1 || pos.y + height + 2 >= 255
      || !self.place_above.contains(world.get(pos + Pos::new(0, -1, 0)))
      || super::basic::trunk_in_3x3(world, pos, block![log[variant = "birch", axis = "y"]])
      || super::basic::boulder_in_3x3(world, pos) {
      return Err(UndoError);
    }
    let log = block![log[variant = "birch", axis = "y"]];
    let leaves = block![leaves[variant = "birch", check_decay = false, decayable = false]];
    let mut crown = Vec::new();
    for y in height - 3..=height - 2 {
      for x in -2_i32..=2 {
        for z in -2_i32..=2 {
          if x.abs() == 2 && z.abs() == 2 && rng.range(0..5) == 0 { continue; }
          crown.push(pos + Pos::new(x, y, z));
        }
      }
    }
    for y in height - 1..=height {
      for x in -1_i32..=1 {
        for z in -1_i32..=1 {
          if x.abs() == 1 && z.abs() == 1
            && (y == height || rng.range(0..5) == 0) { continue; }
          crown.push(pos + Pos::new(x, y, z));
        }
      }
    }
    for y in 0..height {
      let trunk_pos = pos + Pos::new(0, y, 0);
      if world.get(trunk_pos) != block![air]
        || any_wood_adjacent(world, trunk_pos) {
        return Err(UndoError);
      }
    }
    let blocked = crown.iter().filter(|&&p| {
      let state = world.get(p);
      state != block![air] && state.block_kind() != leaves.block
    }).count();
    if blocked * 100 > crown.len() * 15 { return Err(UndoError); }
    for p in crown {
      if world.get(p) == block![air] { world.set(p, leaves); }
    }
    for y in 0..height { world.set(pos + Pos::new(0, y, 0), log); }
    for y in 2..=4 {
      for (x, z, facing) in [(1, 0, "east"), (-1, 0, "west"), (0, 1, "south"), (0, -1, "north")] {
        let p = pos + Pos::new(x, y, z);
        if world.get(p) == block![air] && rng.range(0..9) < 3 {
          let mut shroom = block![rgen:polypore[type = "one"]];
          shroom.set_prop("type", *rng.choose(&["one", "two", "three"]));
          shroom.set_prop("facing", facing);
          world.set(p, shroom);
        }
      }
    }
    Ok(())
  }
}
