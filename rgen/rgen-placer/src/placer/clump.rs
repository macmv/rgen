use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, Pos, block};
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

#[allow(dead_code)]
pub struct PlantClumps {
  pub place_above: BlockFilter,

  pub radius:   RangeInclusive<u8>,
  pub attempts: u32,
}

pub struct BushClumps {
  pub place_above: BlockFilter,
  pub log:         BlockState,
  pub leaves:      BlockState,

  pub radius:        RangeInclusive<u8>,
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

impl Placer for BushClumps {
  fn radius(&self) -> u8 { *self.radius.end() }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    if self.place_above.contains(world.get(pos + Pos::new(0, -1, 0)))
      && world.get(pos) == block![air]
    {
      world.set(pos, self.log);

      for offset in [
        // surround the log in leaves
        Pos::new(-1, 0, 0),
        Pos::new(0, 0, -1),
        Pos::new(0, 0, 1),
        Pos::new(1, 0, 0),
        // and build a few leaves on top
        Pos::new(0, 1, 0),
      ] {
        let side = pos + offset;
        if world.get(side) == block![air] {
          world.set(side, self.leaves);
        }
      }

      // now sprink a few more leaves around
      for _ in 0..10 {
        let side_below = pos + Pos::new(rng.range(-2..=2), rng.range(0..=1), rng.range(-2..=2));
        let side = side_below + Pos::new(0, 1, 0);
        if world.get(side_below) != block![air] && world.get(side) == block![air] {
          world.set(side, self.leaves);
        }
      }
    }

    Ok(())
  }
}

impl Placer for PlantClumps {
  fn radius(&self) -> u8 { *self.radius.end() }
  fn avg_per_chunk(&self) -> f64 { 3.0 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let radius = rng.range(*self.radius.start() as i32..=*self.radius.end() as i32);

    for _ in 0..self.attempts {
      let mut pos = pos;
      for _ in 0..radius {
        pos = pos + Pos::new(rng.range(-1..=1), 0, rng.range(-1..=1));
      }

      let above_pos = pos + Pos::new(0, 1, 0);

      if self.place_above.contains(world.get(pos)) && world.get(above_pos) == block![air] {
        //let block = *rng.choose(self.place_plants);

        //world.set(above_pos, self.place_plants);
      }
    }

    Ok(())
  }
}
