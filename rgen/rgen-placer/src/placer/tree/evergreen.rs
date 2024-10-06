use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_llama::Structure;
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct EverGreen {
  pub place_above:  BlockFilter,
  pub trunk:        BlockState,
  pub leaves:       BlockState,
  pub avg_in_chunk: f64,
  pub is_spruce:    bool,
  pub size:         EvergreenSize,
}

pub enum EvergreenSize {
  Standard,
  Tall,
  Fat,
}

impl EverGreen {
  pub fn new(blocks: &Blocks) -> Self {
    EverGreen {
      avg_in_chunk: 13.0, //40.0,
      place_above:  blocks.grass.default_state.into(),
      trunk:        blocks.log.with_data(2),
      leaves:       blocks.rgen_leaves3.with_data(0),
      is_spruce:    true,
      size:         EvergreenSize::Standard,
    }
  }
}

impl Placer for EverGreen {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(9, 11);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return;
    }

    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos).block != Block::AIR {
      return;
    }

    for x in -1..=1 {
      for z in -1..=1 {
        if world.get(pos + Pos::new(x, 0, z)) == self.trunk {
          return;
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
        EvergreenSize::Fat => self.build_standard_fir(world, pos, rng, false),
        EvergreenSize::Tall => self.build_standard_fir(world, pos, rng, true),
      }
    }
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
    for y in 0..=1 {
      world.set(pos, self.trunk);
      pos = pos + Pos::new(0, 1, 0);
    }

    // Adds a small bottom ring to the bottom of the standard spruce
    if rng.rand_inclusive(1, 5) == 1 {
      pos = pos + Pos::new(0, -1, 0);
      self.build_disk(world, &mut pos, rng, 1);
    }
    // Builds the main standrd rings
    let height = 2;

    if is_tall {
      let height = 3;
    }

    for ring in 1..=height {
      self.build_disk(world, &mut pos, rng, 2);
      self.build_disk(world, &mut pos, rng, 1);
    }
    self.build_crown(world, pos, rng);
  }

  fn build_fat_spruce(&self, world: &mut PartialWorld, mut pos: Pos, rng: &mut Rng) {
    for y in 0..=rng.rand_inclusive(0, 1) {
      world.set(pos, self.trunk);
      pos = pos + Pos::new(0, 1, 0);
    }

    // Adds a small bottom ring to the bottom of the standard spruce
    if rng.rand_inclusive(1, 5) == 1 {
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
    for y in 0..=rng.rand_inclusive(1, 2) {
      world.set(pos, self.trunk);
      pos = pos + Pos::new(0, 1, 0);
    }

    // Builds the main standrd rings
    let height = 2;

    if is_tall {
      let height = 3;
    }

    for ring in 1..=height {
      self.build_disk(world, &mut pos, rng, 2);
      self.build_fir_spacer(world, &mut pos, rng);
    }
    self.build_fir_top_disk(world, &mut pos, rng);
    pos = pos + Pos::new(0, 0, 0);
    self.build_fir_crown(world, pos, rng);
  }

  //BUILD CROWN
  fn build_fir_crown(&self, world: &mut PartialWorld, mut pos: Pos, rng: &mut Rng) {
    for x in -1i32..=1 {
      for z in -1i32..=1 {
        if !(x.abs() == 1 && (z.abs() == 1)) {
          if world.get(pos + Pos::new(x, 0, z)).block == Block::AIR {
            world.set(pos + Pos::new(x, 0, z), self.leaves);
          }
        } else {
          if rng.rand_inclusive(0, 7) == 0 {
            if world.get(pos + Pos::new(x, 0, z)).block == Block::AIR {
              world.set(pos + Pos::new(x, 0, z), self.leaves);
            }
          }
        }
      }
    }
    pos = pos + Pos::new(0, 1, 0);

    for x in -1i32..=1 {
      for z in -1i32..=1 {
        if !(x.abs() == 1 && (z.abs() == 1)) {
          if rng.rand_inclusive(0, 1) == 0 {
            if world.get(pos + Pos::new(x, 0, z)).block == Block::AIR {
              world.set(pos + Pos::new(x, 0, z), self.leaves);
            }
          }
        }
      }
    }
    if world.get(pos).block == Block::AIR {
      world.set(pos, self.leaves);
    }
    pos = pos + Pos::new(0, 1, 0);
    if world.get(pos).block == Block::AIR {
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
  fn build_disk(&self, world: &mut PartialWorld, pos: &mut Pos, rng: &mut Rng, size: i32) {
    for x in (size * -1)..=size {
      for z in (size * -1)..=size {
        if (x.abs() + z.abs()) <= (size + (size / 2)) {
          if world.get(*pos + Pos::new(x, 0, z)).block == Block::AIR {
            world.set(*pos + Pos::new(x, 0, z), self.leaves);
          }
        }
      }
    }
    world.set(*pos, self.trunk);
    *pos = *pos + Pos::new(0, 1, 0);
  }
  fn build_fir_top_disk(&self, world: &mut PartialWorld, pos: &mut Pos, rng: &mut Rng) {
    for x in -2i32..=2 {
      for z in -2i32..=2 {
        if !(x.abs() + z.abs() > 2) {
          if world.get(*pos + Pos::new(x, 0, z)).block == Block::AIR {
            world.set(*pos + Pos::new(x, 0, z), self.leaves);
          }
        }
      }
    }
    world.set(*pos, self.trunk);
    *pos = *pos + Pos::new(0, 1, 0);
  }
  fn build_fir_spacer(&self, world: &mut PartialWorld, pos: &mut Pos, rng: &mut Rng) {
    for x in -1..=1 {
      for z in -1..=1 {
        if rng.rand_inclusive(0, 1) == 0 {
          if world.get(*pos + Pos::new(x, 0, z)).block == Block::AIR {
            world.set(*pos + Pos::new(x, 0, z), self.leaves);
          }
        }
      }
    }
    world.set(*pos, self.trunk);
    *pos = *pos + Pos::new(0, 1, 0);
  }
}
