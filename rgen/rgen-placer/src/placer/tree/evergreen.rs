use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct EverGreen {
  pub place_above:   BlockFilter,
  pub trunk:         BlockState,
  pub leaves:        BlockState,
  pub avg_per_chunk: f64,
  pub is_spruce:     bool,
  pub size:          EvergreenSize,
}

pub enum EvergreenSize {
  Standard,
  Tall,
  Fat,
}

impl Default for EverGreen {
  fn default() -> Self {
    EverGreen {
      avg_per_chunk: 13.0, //40.0,
      place_above:   block![grass].into(),
      trunk:         block![log[2]],
      leaves:        block![rgen:leaves3[0]],
      is_spruce:     true,
      size:          EvergreenSize::Standard,
    }
  }
}

impl Placer for EverGreen {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let height = rng.range(9..=11);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }

    for x in -1..=1 {
      for z in -1..=1 {
        if world.get(pos + Pos::new(x, 0, z)) == self.trunk {
          return Err(UndoError);
        }
      }
    }
    //self.build_disk(world, &mut pos.clone(), rng, 4);

    if self.is_spruce {
      match self.size {
        EvergreenSize::Standard => self.build_standard_spruce(world, pos, rng, false),
        EvergreenSize::Fat => self.build_fat_spruce(world, pos, rng),
        EvergreenSize::Tall => self.build_standard_spruce(world, pos, rng, true),
      }
    } else {
      match self.size {
        EvergreenSize::Standard => self.build_standard_fir(world, pos, rng, false),
        EvergreenSize::Fat => self.build_fat_fir(world, pos, rng),
        EvergreenSize::Tall => self.build_tall_fir(world, pos, rng),
      }
    }

    Ok(())
  }
}

impl EverGreen {
  fn build_standard_spruce(
    &self,
    world: &mut PartialWorld,
    mut pos: Pos,
    rng: &mut Rng,
    is_tall: bool,
  ) {
    for _ in 0..=1 {
      world.set(pos, self.trunk);
      pos = pos + Pos::new(0, 1, 0);
    }

    // Adds a small bottom ring to the bottom of the standard spruce
    if rng.range(1..=5) == 1 {
      pos = pos + Pos::new(0, -1, 0);
      self.build_disk(world, &mut pos, rng, 1);
    }
    // Builds the main standrd rings
    let mut height = 2;

    if is_tall {
      height = 3;
    }

    for _ in 1..=height {
      self.build_disk(world, &mut pos, rng, 2);
      self.build_disk(world, &mut pos, rng, 1);
    }
    self.build_crown(world, pos, rng);
  }

  fn build_fat_spruce(&self, world: &mut PartialWorld, mut pos: Pos, rng: &mut Rng) {
    for _ in 0..=rng.range(0..=1) {
      world.set(pos, self.trunk);
      pos = pos + Pos::new(0, 1, 0);
    }

    // Adds a small bottom ring to the bottom of the standard spruce
    if rng.range(1..=5) == 1 {
      self.build_disk(world, &mut pos, rng, 1);
    }
    // Builds the main standrd rings
    self.build_disk(world, &mut pos, rng, 3);
    self.build_disk(world, &mut pos, rng, 2);
    self.build_disk(world, &mut pos, rng, 1);
    self.build_disk(world, &mut pos, rng, 2);
    self.build_disk(world, &mut pos, rng, 1);

    self.build_crown(world, pos, rng);
  }

  fn build_standard_fir(
    &self,
    world: &mut PartialWorld,
    mut pos: Pos,
    rng: &mut Rng,
    is_tall: bool,
  ) {
    for _ in 0..=rng.range(1..=2) {
      world.set(pos, self.trunk);
      pos = pos + Pos::new(0, 1, 0);
    }

    // Builds the main standrd rings
    let mut height = 2;

    if is_tall {
      height = 3;
    }

    for _ in 1..=height {
      self.build_disk(world, &mut pos, rng, 2);
      self.build_fir_spacer(world, &mut pos, rng);
    }
    self.build_fir_top_disk(world, &mut pos, rng);
    pos = pos + Pos::new(0, 0, 0);
    self.build_fir_crown(world, pos, rng);
  }

  fn build_tall_fir(&self, world: &mut PartialWorld, mut pos: Pos, rng: &mut Rng) {
    for _ in 0..=rng.range(1..=2) {
      world.set(pos, self.trunk);
      pos = pos + Pos::new(0, 1, 0);
    }

    for _ in 1..=3 {
      self.build_fir_top_disk(world, &mut pos, rng);
      self.build_fir_spacer(world, &mut pos, rng);
    }
    self.build_fir_top_disk(world, &mut pos, rng);
    pos = pos + Pos::new(0, 0, 0);
    self.build_fir_crown(world, pos, rng);
  }

  fn build_fat_fir(&self, world: &mut PartialWorld, mut pos: Pos, rng: &mut Rng) {
    for _ in 0..=rng.range(1..=2) {
      world.set(pos, self.trunk);
      pos = pos + Pos::new(0, 1, 0);
    }

    self.build_disk(world, &mut pos, rng, 3);
    self.build_fir_spacer(world, &mut pos, rng);

    self.build_disk(world, &mut pos, rng, 2);
    self.build_fir_spacer(world, &mut pos, rng);

    self.build_fir_top_disk(world, &mut pos, rng);
    pos = pos + Pos::new(0, 0, 0);
    self.build_fir_crown(world, pos, rng);
  }

  //BUILD CROWN
  fn build_fir_crown(&self, world: &mut PartialWorld, mut pos: Pos, rng: &mut Rng) {
    for x in -1i32..=1 {
      for z in -1i32..=1 {
        if !(x.abs() == 1 && (z.abs() == 1)) {
          if world.get(pos + Pos::new(x, 0, z)) == block![air] {
            world.set(pos + Pos::new(x, 0, z), self.leaves);
          }
        } else if rng.range(0..=7) == 0 && world.get(pos + Pos::new(x, 0, z)) == block![air] {
          world.set(pos + Pos::new(x, 0, z), self.leaves);
        }
      }
    }
    pos = pos + Pos::new(0, 1, 0);

    for x in -1i32..=1 {
      for z in -1i32..=1 {
        if !(x.abs() == 1 && (z.abs() == 1))
          && rng.range(0..=1) == 0
          && world.get(pos + Pos::new(x, 0, z)) == block![air]
        {
          world.set(pos + Pos::new(x, 0, z), self.leaves);
        }
      }
    }
    if world.get(pos) == block![air] {
      world.set(pos, self.leaves);
    }
    pos = pos + Pos::new(0, 1, 0);
    if world.get(pos) == block![air] {
      world.set(pos, self.leaves);
    }
  }

  fn build_crown(&self, world: &mut PartialWorld, mut pos: Pos, rng: &mut Rng) {
    enum CrownType {
      FlatHat,
      CrownInset,
      Crown,
      Bishop,
      BishopInset,
    }
    let chosen_crown = rng.choose(&[
      CrownType::FlatHat,
      CrownType::CrownInset,
      CrownType::Crown,
      CrownType::Bishop,
      CrownType::BishopInset,
    ]);

    match chosen_crown {
      CrownType::FlatHat => {
        world.set(pos, self.trunk);
        pos = pos + Pos::new(0, 1, 0);
        self.build_disk(world, &mut pos, rng, 1);
        world.set(pos + Pos::new(0, -1, 0), self.leaves);
      }
      CrownType::Crown => {
        world.set(pos, self.trunk);
        pos = pos + Pos::new(0, 1, 0);

        self.build_disk(world, &mut pos, rng, 1);
        world.set(pos + Pos::new(0, -1, 0), self.leaves);
        world.set(pos, self.leaves);
      }
      CrownType::CrownInset => {
        world.set(pos, self.trunk);
        pos = pos + Pos::new(0, 1, 0);

        self.build_disk(world, &mut pos, rng, 1);
        world.set(pos, self.leaves);
      }
      CrownType::Bishop => {
        // goes back to remove inset
        world.set(pos + Pos::new(0, -1, 0), self.leaves);
        // adds leaf middle
        world.set(pos, self.leaves);
        pos = pos + Pos::new(0, 1, 0);
        //adds top crown and removes inset
        self.build_disk(world, &mut pos, rng, 1);
        world.set(pos + Pos::new(0, -1, 0), self.leaves);
      }
      CrownType::BishopInset => {
        // adds leaf middle
        world.set(pos, self.trunk);
        pos = pos + Pos::new(0, 1, 0);
        //adds top crown and removes inset
        self.build_disk(world, &mut pos, rng, 1);
        world.set(pos + Pos::new(0, -1, 0), self.leaves);
      }
    }
  }

  // BUILD DISK
  fn build_disk(&self, world: &mut PartialWorld, pos: &mut Pos, _rng: &mut Rng, size: i32) {
    for x in -size..=size {
      for z in -size..=size {
        if (x.abs() + z.abs()) <= (size + (size / 2))
          && world.get(*pos + Pos::new(x, 0, z)) == block![air]
        {
          world.set(*pos + Pos::new(x, 0, z), self.leaves);
        }
      }
    }
    world.set(*pos, self.trunk);
    *pos = *pos + Pos::new(0, 1, 0);
  }
  fn build_fir_top_disk(&self, world: &mut PartialWorld, pos: &mut Pos, _rng: &mut Rng) {
    for x in -2i32..=2 {
      for z in -2i32..=2 {
        if x.abs() + z.abs() <= 2 && world.get(*pos + Pos::new(x, 0, z)) == block![air] {
          world.set(*pos + Pos::new(x, 0, z), self.leaves);
        }
      }
    }
    world.set(*pos, self.trunk);
    *pos = *pos + Pos::new(0, 1, 0);
  }
  fn build_fir_spacer(&self, world: &mut PartialWorld, pos: &mut Pos, rng: &mut Rng) {
    for x in -1..=1 {
      for z in -1..=1 {
        if rng.range(0..=1) == 0 && world.get(*pos + Pos::new(x, 0, z)) == block![air] {
          world.set(*pos + Pos::new(x, 0, z), self.leaves);
        }
      }
    }
    world.set(*pos, self.trunk);
    *pos = *pos + Pos::new(0, 1, 0);
  }
}
