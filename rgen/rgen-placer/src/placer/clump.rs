use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockInfo, BlockState, Pos, block};
use rgen_world::PartialWorld;

use crate::{Placer, Result, Rng, rng::Random};

pub struct Clumps {
  pub place_above: BlockFilter,
  pub place:       BlockState,

  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

pub struct GrassClumps {
  pub place_above:      BlockFilter,
  pub place_short:      BlockState,
  pub place_tall_lower: BlockState,
  pub place_tall_upper: BlockState,

  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

/// Clumps dried grass while varying color, texture variant, and height
/// independently for each plant in the clump.
pub struct DriedGrassClumps {
  pub place_above: BlockFilter,
  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

/// Irregular persistent leaf-only clumps for low, shrub-like canopies.
pub struct LeafClumps {
  pub place_above: BlockFilter,
  pub leaves: BlockState,
  pub avg_per_chunk: f64,
}

/// Four short, downward-growing leaf arms rooted around a center leaf.
pub struct CreepingLeaves {
  /// Maximum outward steps per arm, including the first leaf.
  pub arm_length: u8,
  pub place_above: BlockFilter,
  pub leaves: BlockState,
  pub avg_per_chunk: f64,
}

impl Placer for Clumps {
  fn radius(&self) -> u8 { *self.radius.end() }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let radius = rng.range(*self.radius.start() as i32..=*self.radius.end() as i32);

    for _ in 0..self.attempts {
      let mut pos = pos;
      for _ in 0..radius {
        pos = pos + Pos::new(rng.range(-1..=1), 0, rng.range(-1..=1));
      }

      let below_pos = pos + Pos::new(0, -1, 0);

      if self.place_above.contains(world.get(below_pos)) && world.get(pos) == block![air] {
        world.set(pos, self.place);
      }
    }

    Ok(())
  }
}

impl GrassClumps {
  pub fn new() -> Self {
    GrassClumps {
      place_above:      block![grass].into(),
      place_short:      block![tallgrass[1]],     // Grass
      place_tall_lower: block![double_plant[2]],  // Tall grass lower
      place_tall_upper: block![double_plant[10]], // Tall grass upper

      radius:        4..=10,
      attempts:      60,
      avg_per_chunk: 3.0,
    }
  }
}

impl Placer for GrassClumps {
  fn radius(&self) -> u8 { *self.radius.end() }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let radius = rng.range(*self.radius.start() as i32..=*self.radius.end() as i32);

    for _ in 0..self.attempts {
      let mut pos = pos;
      for _ in 0..radius {
        pos = pos + Pos::new(rng.range(-1..=1), 0, rng.range(-1..=1));
      }

      let below_pos = pos + Pos::new(0, -1, 0);

      if self.place_above.contains(world.get(below_pos)) && world.get(pos) == block![air] {
        let height = *rng.choose(&[1, 1, 1, 1, 1, 1, 2]);

        if height == 1 {
          world.set(pos, self.place_short);
        } else {
          world.set(pos, self.place_tall_lower);
          world.set(pos + Pos::new(0, 1, 0), self.place_tall_upper);
        }
      }
    }

    Ok(())
  }
}

impl Placer for DriedGrassClumps {
  fn radius(&self) -> u8 { *self.radius.end() }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let short = [
      [
        block![rgen:dried_grass[variant = "brown_1"]],
        block![rgen:dried_grass[variant = "brown_2"]],
        block![rgen:dried_grass[variant = "brown_3"]],
      ],
      [
        block![rgen:dried_grass[variant = "green_1"]],
        block![rgen:dried_grass[variant = "green_2"]],
        block![rgen:dried_grass[variant = "green_3"]],
      ],
      [
        block![rgen:dried_grass[variant = "yellow_1"]],
        block![rgen:dried_grass[variant = "yellow_2"]],
        block![rgen:dried_grass[variant = "yellow_3"]],
      ],
    ];
    let tall_lower = [
      [
        block![rgen:double_tall_dried_grass[half = "lower", variant = "brown_1"]],
        block![rgen:double_tall_dried_grass[half = "lower", variant = "brown_2"]],
      ],
      [
        block![rgen:double_tall_dried_grass[half = "lower", variant = "green_1"]],
        block![rgen:double_tall_dried_grass[half = "lower", variant = "green_2"]],
      ],
      [
        block![rgen:double_tall_dried_grass[half = "lower", variant = "yellow_1"]],
        block![rgen:double_tall_dried_grass[half = "lower", variant = "yellow_2"]],
      ],
    ];
    let tall_upper = [
      [
        block![rgen:double_tall_dried_grass[half = "upper", variant = "brown_1"]],
        block![rgen:double_tall_dried_grass[half = "upper", variant = "brown_2"]],
      ],
      [
        block![rgen:double_tall_dried_grass[half = "upper", variant = "green_1"]],
        block![rgen:double_tall_dried_grass[half = "upper", variant = "green_2"]],
      ],
      [
        block![rgen:double_tall_dried_grass[half = "upper", variant = "yellow_1"]],
        block![rgen:double_tall_dried_grass[half = "upper", variant = "yellow_2"]],
      ],
    ];

    let radius = rng.range(*self.radius.start() as i32..=*self.radius.end() as i32);
    for _ in 0..self.attempts {
      let mut plant_pos = pos;
      for _ in 0..radius {
        plant_pos = plant_pos + Pos::new(rng.range(-1..=1), 0, rng.range(-1..=1));
      }

      let below_pos = plant_pos + Pos::new(0, -1, 0);
      if !self.place_above.contains(world.get(below_pos)) || world.get(plant_pos) != block![air] {
        continue;
      }

      let color = rng.range(0..3) as usize;
      if rng.range(0..2) == 0 {
        let variant = rng.range(0..3) as usize;
        world.set(plant_pos, short[color][variant]);
      } else {
        let above_pos = plant_pos + Pos::new(0, 1, 0);
        if world.get(above_pos) != block![air] {
          continue;
        }
        let variant = rng.range(0..2) as usize;
        world.set(plant_pos, tall_lower[color][variant]);
        world.set(above_pos, tall_upper[color][variant]);
      }
    }

    Ok(())
  }
}

impl Placer for LeafClumps {
  fn radius(&self) -> u8 { 4 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let below = pos + Pos::new(0, -1, 0);
    if world.get(below) == block![water]
      || !self.place_above.contains(world.get(below))
      || !leaf_clump_replaceable(world.get(pos))
    {
      return Ok(());
    }

    // Each core is an irregular, compact leaf mass. All variants are built
    // from the same anchor so the adjoining-core chance stays attached.
    let mut core = Vec::new();
    match rng.range(0..5) {
      0 => {
        for x in 0..2 {
          for z in 0..2 {
            core.push(Pos::new(x, 0, z));
          }
        }
        let mut corners = vec![Pos::new(0, 1, 0), Pos::new(1, 1, 0), Pos::new(0, 1, 1), Pos::new(1, 1, 1)];
        let remove = if rng.range(0..2) == 0 { 1 } else { 2 };
        for _ in 0..remove {
          let index = rng.range(0..corners.len() as i32) as usize;
          corners.remove(index);
        }
        core.extend(corners);
      }
      1 => {
        core.extend([Pos::new(0, 0, 0), Pos::new(1, 0, 0), Pos::new(0, 0, 1), Pos::new(0, 1, 0)]);
      }
      2 => {
        let arms = [Pos::new(-1, 0, 0), Pos::new(1, 0, 0), Pos::new(0, 0, -1), Pos::new(0, 0, 1)];
        let removed_arm = rng.range(0..4) as usize;
        core.extend([Pos::new(0, 0, 0), Pos::new(0, 1, 0)]);
        for (index, arm) in arms.iter().enumerate() {
          if index != removed_arm { core.push(*arm); }
        }
        let corners = [Pos::new(-1, 0, -1), Pos::new(1, 0, -1), Pos::new(-1, 0, 1), Pos::new(1, 0, 1)];
        let corner = corners[rng.range(0..4) as usize];
        if rng.range(0..4) == 0 { core.push(corner); }
        if rng.range(0..4) == 0 { core.push(corners[rng.range(0..4) as usize]); }
      }
      3 => {
        let along_x = rng.range(0..2) == 0;
        for step in -1..=1 {
          core.push(if along_x { Pos::new(step, 0, 0) } else { Pos::new(0, 0, step) });
        }
        core.push(Pos::new(0, 1, 0));
        let side = if rng.range(0..2) == 0 { -1 } else { 1 };
        let along = rng.range(-1..=1);
        core.push(if along_x { Pos::new(along, 0, side) } else { Pos::new(side, 0, along) });
      }
      _ => {
        let arms = [Pos::new(-1, 0, 0), Pos::new(1, 0, 0), Pos::new(0, 0, -1), Pos::new(0, 0, 1)];
        core.push(Pos::new(0, 0, 0));
        for arm in arms { core.push(arm); }
        let corners = [Pos::new(-1, 0, -1), Pos::new(1, 0, -1), Pos::new(-1, 0, 1), Pos::new(1, 0, 1)];
        for corner in corners {
          if rng.range(0..4) == 0 { core.push(corner); }
        }
        core.push(Pos::new(0, 1, 0));
        for arm in arms {
          if rng.range(0..2) == 0 { core.push(arm + Pos::new(0, 1, 0)); }
        }
      }
    }

    let mut cores = vec![(0, 0)];
    if rng.range(0..100) < 30 {
      let distance = rng.range(2..=3);
      match rng.range(0..4) {
        0 => cores.push((-distance, 0)),
        1 => cores.push((distance, 0)),
        2 => cores.push((0, -distance)),
        _ => cores.push((0, distance)),
      }
    }

    for (dx, dz) in cores {
      for offset in &core {
        let mut leaf_pos = pos + *offset + Pos::new(dx, 0, dz);
        if world.get(leaf_pos + Pos::new(0, -1, 0)) != self.leaves {
          leaf_pos = leaf_clump_settle(world, leaf_pos);
        }
        if world.get(leaf_pos + Pos::new(0, -1, 0)) != block![water]
          && leaf_clump_replaceable(world.get(leaf_pos))
        {
          leaf_clump_place(world, leaf_pos, self.leaves);
        }
      }
    }

    // A sparse fringe softens the core outline; the diagonal pair is a
    // separate occasional detail.
    for _ in 0..8 {
      let mut leaf_pos = pos + Pos::new(rng.range(-2..=2), rng.range(0..=1), rng.range(-2..=2));
      if leaf_clump_replaceable(world.get(leaf_pos)) && leaf_clump_has_neighbor(world, leaf_pos, self.leaves) {
        let below = world.get(leaf_pos + Pos::new(0, -1, 0));
        if below == block![air] || leaf_clump_is_plant(below) {
          leaf_pos = leaf_clump_settle(world, leaf_pos);
        }
        if !leaf_clump_replaceable(world.get(leaf_pos))
          || world.get(leaf_pos + Pos::new(0, -1, 0)) == block![water]
        { continue; }
        leaf_clump_place(world, leaf_pos, self.leaves);
      }
    }
    if rng.range(0..4) == 0 {
      let diagonal = Pos::new(if rng.range(0..2) == 0 { -1 } else { 1 }, 0, if rng.range(0..2) == 0 { -1 } else { 1 });
      for offset in [Pos::new(0, 0, 0), diagonal] {
        let mut leaf_pos = pos + offset;
        if leaf_clump_replaceable(world.get(leaf_pos)) && leaf_clump_has_neighbor(world, leaf_pos, self.leaves) {
          let below = world.get(leaf_pos + Pos::new(0, -1, 0));
          if below == block![air] || leaf_clump_is_plant(below) {
            leaf_pos = leaf_clump_settle(world, leaf_pos);
          }
          if !leaf_clump_replaceable(world.get(leaf_pos))
            || world.get(leaf_pos + Pos::new(0, -1, 0)) == block![water]
          { continue; }
          leaf_clump_place(world, leaf_pos, self.leaves);
        }
      }
    }

    Ok(())
  }
}

impl Placer for CreepingLeaves {
  fn radius(&self) -> u8 { 9 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let below = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below)) || world.get(pos) != block![air] {
      return Ok(());
    }

    // The center has no single arm to terminate, but it follows the same
    // dangling-leaf rule as every leaf placed by the arms.
    creeping_leaf_place(world, pos, self.leaves);
    let above_heart = pos + Pos::new(0, 1, 0);
    if world.get(above_heart) == block![air] {
      creeping_leaf_place(world, above_heart, self.leaves);
    }
    let directions = [
      Pos::new(0, 0, -1),
      Pos::new(1, 0, 0),
      Pos::new(0, 0, 1),
      Pos::new(-1, 0, 0),
    ];

    for direction in directions {
      let first = pos + direction;
      if world.get(first) != block![air] {
        continue;
      }
      if creeping_leaf_place(world, first, self.leaves) {
        continue;
      }
      let side = Pos::new(-direction.z, 0, direction.x);
      let mut tip = first;

      for _ in 1..self.arm_length {
        let down_forward = tip + direction + Pos::new(0, -1, 0);
        if world.get(down_forward) == block![air] {
          if creeping_leaf_place(world, down_forward, self.leaves) {
            break;
          }
          tip = down_forward;
          continue;
        }

        let down_sides = [
          tip + direction + side + Pos::new(0, -1, 0),
          tip + direction - side + Pos::new(0, -1, 0),
        ];
        let available: Vec<Pos> = down_sides.into_iter()
          .filter(|candidate| world.get(*candidate) == block![air])
          .collect();
        if !available.is_empty() {
          tip = *rng.choose(&available);
          if creeping_leaf_place(world, tip, self.leaves) {
            break;
          }
          continue;
        }

        let side_steps = [tip + direction + side, tip + direction - side];
        let available: Vec<Pos> = side_steps.into_iter()
          .filter(|candidate| world.get(*candidate) == block![air])
          .collect();
        if !available.is_empty() {
          let end = *rng.choose(&available);
          creeping_leaf_place(world, end, self.leaves);
        }
        break;
      }
    }

    Ok(())
  }
}

/// Place a leaf and return true when it also had to grow a hanging leaf,
/// which ends the current arm immediately.
fn creeping_leaf_place(world: &mut PartialWorld, pos: Pos, leaves: BlockState) -> bool {
  if world.get(pos) != block![air] {
    return false;
  }
  world.set(pos, leaves);
  let below = pos + Pos::new(0, -1, 0);
  if world.get(below) == block![air] {
    world.set(below, leaves);
    return true;
  }
  false
}

fn leaf_clump_settle(world: &PartialWorld, mut pos: Pos) -> Pos {
  loop {
    if pos.y <= 1 { break; }
    let below = pos + Pos::new(0, -1, 0);
    let below_state = world.get(below);
    if below_state == block![air] || leaf_clump_is_plant(below_state) {
      pos = below;
    } else {
      break;
    }
  }
  pos
}

fn leaf_clump_place(world: &mut PartialWorld, pos: Pos, leaves: BlockState) {
  let current = world.get(pos);
  if current.block_kind() == block![double_plant].block {
    let other_half = if current.meta() >= 8 {
      pos + Pos::new(0, -1, 0)
    } else {
      pos + Pos::new(0, 1, 0)
    };
    if world.get(other_half).block_kind() == block![double_plant].block {
      world.set(other_half, block![air]);
    }
  }
  world.set(pos, leaves);
}

fn leaf_clump_replaceable(state: BlockInfo<'_>) -> bool {
  state == block![air] || leaf_clump_is_plant(state)
}

fn leaf_clump_has_neighbor(world: &PartialWorld, pos: Pos, leaves: BlockState) -> bool {
  [
    Pos::new(-1, 0, 0),
    Pos::new(1, 0, 0),
    Pos::new(0, -1, 0),
    Pos::new(0, 1, 0),
    Pos::new(0, 0, -1),
    Pos::new(0, 0, 1),
  ]
  .iter()
  .any(|offset| world.get(pos + *offset) == leaves)
}

fn leaf_clump_is_plant(state: BlockInfo<'_>) -> bool {
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
    || kind == block![rgen:plant].block
    || kind == block![rgen:dune_grass].block
    || kind == block![rgen:yellow_wildflower_shrub].block
    || kind == block![rgen:flower].block
    || kind == block![rgen:lavender_plant].block
    || kind == block![rgen:double_tall_lavender_plant].block
    || kind == block![rgen:glasswort].block
    || kind == block![rgen:mossy_carpet].block
    || kind == block![rgen:glow_vine].block
    || kind == block![rgen:pink_orchid].block
    || kind == block![rgen:passion_flower].block
    || kind == block![rgen:heliconia].block
    || kind == block![rgen:pink_heart].block
    || kind == block![rgen:torch_ginger].block
    || kind == block![rgen:orchidaceae].block
    || kind == block![rgen:ipomoea].block
    || kind == block![rgen:bromeliads].block
    || kind == block![rgen:ficus_elastica].block
    || kind == block![rgen:yellow_jungle_rose].block
    || kind == block![rgen:bird_of_paradise].block
    || kind == block![rgen:jungle_bush].block
}
