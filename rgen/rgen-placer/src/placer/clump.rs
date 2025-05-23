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
