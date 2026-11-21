use rgen_base::{BlockFilter, BlockInfo, BlockState, Pos, block};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

/// Builds a compact, irregular boulder from selectable rock, slab, and stair
/// materials.
pub struct MossBoulder {
  /// Allowed ground directly beneath the starting core.
  pub place_above: BlockFilter,
  /// Material beneath and around the boulder arms.
  pub footing: BlockState,
  pub structure_mix: Vec<BlockState>,
  pub slab_mix:      Vec<BlockState>,
  pub stair_mix:     Vec<BlockState>,
  pub small_weight:  u8,
  pub medium_weight: u8,
  pub medium_plus_weight: u8,
  pub avg_per_chunk: f64,
  /// Reject sites with sand beneath the boulder's possible footprint.
  pub avoid_sand_under_footprint: bool,
}

impl Default for MossBoulder {
  fn default() -> Self {
    Self {
      place_above: BlockFilter::All,
      footing: block![gravel],
      structure_mix: vec![
        block![stone],
        block![stone[variant = "andesite"]],
        block![cobblestone],
        block![mossy_cobblestone],
        block![rgen:mossy_stone],
      ],
      slab_mix: vec![block![stone_slab[variant = "cobblestone", half = "bottom"]]],
      stair_mix: vec![block![stone_stairs[half = "bottom", shape = "straight"]]],
      small_weight: 50,
      medium_weight: 35,
      medium_plus_weight: 15,
      avg_per_chunk: 2.0,
      avoid_sand_under_footprint: false,
    }
  }
}

impl Placer for MossBoulder {
  fn radius(&self) -> u8 { 3 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if pos.y <= 1
      || pos.y + 3 >= 255
      || self.structure_mix.is_empty()
      || self.slab_mix.is_empty()
      || self.stair_mix.is_empty()
      || (self.small_weight as u16 + self.medium_weight as u16 + self.medium_plus_weight as u16) == 0
      || !self.place_above.contains(world.get(pos + Pos::new(0, -1, 0)))
      || world.get(pos) != block![air]
      || world.get(pos + Pos::new(0, -1, 0)) == block![air]
      || world.get(pos + Pos::new(0, -1, 0)) == block![water]
    {
      return Err(UndoError);
    }

    if self.avoid_sand_under_footprint {
      for dx in -3..=3 {
        for dz in -3..=3 {
          if world.get(pos + Pos::new(dx, -1, dz)) == block![sand] {
            return Err(UndoError);
          }
        }
      }
    }

    // Leave room around existing tree trunks, including trunks whose base is
    // beside the rock but whose lower branches cross its footprint.
    for dx in -3..=3 {
      for dz in -3..=3 {
        for dy in -2..=2 {
          if is_log(world.get(pos + Pos::new(dx, dy, dz))) {
            return Err(UndoError);
          }
        }
      }
    }

    let total = self.small_weight as i32 + self.medium_weight as i32 + self.medium_plus_weight as i32;
    let shape_roll = rng.range(0..total);
    let small = shape_roll < self.small_weight as i32;
    let medium = !small && shape_roll < self.small_weight as i32 + self.medium_weight as i32;
    let mut first_layer = vec![Pos::new(0, -1, 0), Pos::new(0, 1, 0)];
    let mut core_positions = vec![Pos::new(0, 0, 0)];
    let mut arm_positions = Vec::new();
    let mut stair_corners = Vec::new();
    if small {
      first_layer.extend([
        Pos::new(0, 0, 0),
        Pos::new(-1, 0, 0),
        Pos::new(1, 0, 0),
        Pos::new(0, 0, -1),
        Pos::new(0, 0, 1),
      ]);
      // Keep the original small boulder's extension and corner detail.
      if rng.range(0..4) == 0 {
        let axis_x = rng.range(0..2) == 0;
        let sign = if rng.range(0..2) == 0 { -1 } else { 1 };
        first_layer.push(if axis_x { Pos::new(sign * 2, 0, 0) } else { Pos::new(0, 0, sign * 2) });
      }
      for &(x, z) in &[(-1, -1), (-1, 1), (1, -1), (1, 1)] {
        match rng.range(0..4) {
          0 => first_layer.push(Pos::new(x, 0, z)),
          1 => stair_corners.push((x, z)),
          _ => {}
        }
      }
    } else {
      // A is an arm and C is a core. The medium shape has two cores;
      // medium-plus adds a third core and its surrounding arms.
      let rows: &[&str] = if medium {
        &[" aa ", "acca", " aa "]
      } else {
        &[" aa ", "acca", " aca", "  a "]
      };
      let rotation = rng.range(0..4);
      for (z, row) in rows.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
          if cell != 'a' && cell != 'c' {
            continue;
          }
          let (mut dx, mut dz) = (x as i32 - 1, z as i32 - 1);
          for _ in 0..rotation {
            (dx, dz) = (-dz, dx);
          }
          first_layer.push(Pos::new(dx, 0, dz));
          if cell == 'c' {
            core_positions.push(Pos::new(dx, 0, dz));
          }
          // Every added core gets the same raised rock top as the
          // original core. The shared top-decoration pass can then
          // place a slab or a stair above it.
          if cell == 'c' && (dx != 0 || dz != 0) {
            first_layer.push(Pos::new(dx, 1, dz));
          }
        }
      }
    }

    for offset in &first_layer {
      world.set(pos + *offset, *rng.choose(&self.structure_mix));
    }

    for &(x, z) in &stair_corners {
      let mut stair = *rng.choose(&self.stair_mix);
      let facing = match (x, z) {
        (-1, -1) => if rng.range(0..2) == 0 { "east" } else { "south" },
        (-1, 1) => if rng.range(0..2) == 0 { "east" } else { "north" },
        (1, -1) => if rng.range(0..2) == 0 { "west" } else { "south" },
        (1, 1) => if rng.range(0..2) == 0 { "west" } else { "north" },
        _ => unreachable!(),
      };
      // The stair rises toward the core. Its open front is on the
      // opposite side; leave it out when that space is obstructed.
      let front_offset = match facing {
        "east" => Pos::new(-1, 0, 0),
        "west" => Pos::new(1, 0, 0),
        "north" => Pos::new(0, 0, 1),
        "south" => Pos::new(0, 0, -1),
        _ => unreachable!(),
      };
      let front = world.get(pos + Pos::new(x, 0, z) + front_offset);
      if front != block![air] && !is_plant(front) {
        continue;
      }
      stair.set_prop("facing", facing);
      world.set(pos + Pos::new(x, 0, z), stair);
      arm_positions.push(Pos::new(x, 0, z));
    }

    arm_positions.extend(first_layer.iter().copied().filter(|offset| {
      offset.y == 0 && !core_positions.contains(offset)
    }));
    let mut footings = Vec::new();
    for arm in &arm_positions {
      let footing = Pos::new(arm.x, -1, arm.z);
      if footings.contains(&footing) {
        continue;
      }
      footings.push(footing);
      let three_air_below = (1..=3).all(|depth| world.get(pos + *arm + Pos::new(0, -depth, 0)) == block![air]);
      if three_air_below {
        world.set(pos + footing, *rng.choose(&self.structure_mix));
      } else {
        place_footing_column(world, rng, pos + footing, self.footing, self.footing);
      }
    }
    // Scatter the footing material around the arms only where supported.
    // One in five supported spots uses the rock mix instead.
    let mut surrounding = Vec::new();
    for arm in &arm_positions {
      for (dx, dz) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let offset = Pos::new(arm.x + dx, -1, arm.z + dz);
        if !surrounding.contains(&offset) {
          surrounding.push(offset);
        }
      }
    }
    for offset in surrounding {
      if footings.contains(&offset)
        || core_positions.iter().any(|core| core.x == offset.x && core.z == offset.z)
        || world.get(pos + offset + Pos::new(0, -1, 0)) == block![air]
      {
        continue;
      }
      let material = if rng.range(0..5) == 0 { *rng.choose(&self.structure_mix) } else { self.footing };
      place_footing_column(world, rng, pos + offset, material, self.footing);
    }

    // Each exposed rock top has a one-in-nine decoration roll. One in
    // nine of those decorations is a stair instead of a slab.
    for offset in first_layer {
      let above = pos + offset + Pos::new(0, 1, 0);
      if world.get(above) == block![air] && rng.range(0..9) == 0 {
        let overhead = world.get(above + Pos::new(0, 1, 0));
        if is_ground(overhead) || self.structure_mix.iter().any(|rock| overhead == *rock) {
          world.set(above, *rng.choose(&self.structure_mix));
          continue;
        }
        if rng.range(0..9) == 0 {
          let facing = if offset.x < 0 {
            "east"
          } else if offset.x > 0 {
            "west"
          } else if offset.z < 0 {
            "south"
          } else if offset.z > 0 {
            "north"
          } else {
            ["north", "east", "south", "west"][rng.range(0..4) as usize]
          };
          let front_offset = match facing {
            "east" => Pos::new(-1, 0, 0),
            "west" => Pos::new(1, 0, 0),
            "north" => Pos::new(0, 0, 1),
            _ => Pos::new(0, 0, -1),
          };
          let front = world.get(above + front_offset);
          if front == block![air] || is_plant(front) {
            let mut stair = *rng.choose(&self.stair_mix);
            stair.set_prop("facing", facing);
            world.set(above, stair);
          }
        } else {
          world.set(above, *rng.choose(&self.slab_mix));
        }
      }
    }

    Ok(())
  }
}

fn place_footing_column(world: &mut PartialWorld, rng: &mut Rng, top: Pos, top_material: BlockState, below_material: BlockState) {
  let depth = rng.range(2..=3);
  world.set(top, top_material);
  for step in 1..depth {
    let below = top + Pos::new(0, -step, 0);
    if world.get(below) == block![air] {
      break;
    }
    world.set(below, below_material);
  }
}

fn is_ground(state: BlockInfo<'_>) -> bool {
  let kind = state.block_kind();
  kind == block![grass].block
    || kind == block![dirt].block
    || kind == block![sand].block
    || kind == block![gravel].block
    || kind == block![clay].block
    || kind == block![stone].block
    || kind == block![cobblestone].block
    || kind == block![mossy_cobblestone].block
}

fn is_log(state: BlockInfo<'_>) -> bool {
  let kind = state.block_kind();
  kind == block![log].block
    || kind == block![log2].block
    || kind == block![rgen:log].block
    || kind == block![rgen:log2].block
}

fn is_plant(state: BlockInfo<'_>) -> bool {
  let kind = state.block_kind();
  kind == block![tallgrass].block
    || kind == block![double_plant].block
    || kind == block![red_flower].block
    || kind == block![yellow_flower].block
    || kind == block![brown_mushroom].block
    || kind == block![rgen:dried_grass].block
    || kind == block![rgen:double_tall_dried_grass].block
    || kind == block![rgen:salt_marsh_grass].block
    || kind == block![rgen:reeds].block
}
