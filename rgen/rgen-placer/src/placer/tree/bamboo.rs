use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct Bamboo {
  pub place_above:   BlockFilter,
  pub stalk:         BlockState,
  pub pint_size:     bool,
  pub avg_per_chunk: f64,
}

pub struct BambooClump {
  pub bamboo: Bamboo,

  pub place_above:   BlockFilter,
  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

impl Default for Bamboo {
  fn default() -> Self {
    Bamboo {
      place_above:   block![grass].into(),
      stalk:         block![rgen:bamboo],
      pint_size:     true,
      avg_per_chunk: 0.8,
    }
  }
}

impl Default for BambooClump {
  fn default() -> Self {
    BambooClump {
      bamboo:        Bamboo::default(),
      place_above:   block![grass].into(),
      attempts:      10,
      radius:        1..=4,
      avg_per_chunk: 3.0,
    }
  }
}

impl Placer for Bamboo {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let height = if self.pint_size { rng.range(8..=14) } else { rng.range(15..=20) };

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }

    rng.range(15..=20);
    let mut shoot = 0;

    // Sets rotation
    shoot &= 0b1100;
    let rand = rng.range(0..=3);
    if rand == 0 {
      // 0
      shoot |= 0b0000;
    } else if rand == 1 {
      // 1
      shoot |= 0b0001;
    } else if rand == 2 {
      // 2
      shoot |= 0b0010;
    } else if rand == 3 {
      // 3
      shoot |= 0b0011;
    }

    let mut leaf = shoot;
    // Sets leaf type
    leaf &= 0b0011;
    leaf |= 0b0100;

    for y in 0..=height {
      let pos = pos + Pos::new(0, y, 0);

      if world.get(pos) == block![air] {
        world.set(pos, self.stalk.with_data(if y > height - 3 { leaf } else { shoot }));
      } else {
        return Ok(());
      }
    }

    Ok(())
  }
}

impl Placer for BambooClump {
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
        world.attempt(|world| self.bamboo.place(world, rng, pos));
      }
    }

    Ok(())
  }
}
