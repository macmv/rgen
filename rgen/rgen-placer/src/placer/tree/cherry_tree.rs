use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_llama::Structure;
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct Sakura {
  pub place_above:  BlockFilter,
  pub trunk:        BlockState,
  pub leaves:       BlockState,
  pub avg_in_chunk: f64,
  pub large_size:   bool,

  pub drapes: Vec<Structure>,
}

impl Sakura {
  pub fn new(blocks: &Blocks) -> Self {
    Sakura {
      avg_in_chunk: 4.0,
      place_above:  blocks.grass.default_state.into(),
      trunk:        blocks.rgen_log.with_data(2),
      leaves:       blocks.rgen_leaves.with_data(2),
      large_size:   false,

      drapes: vec![
        rgen_llama::parse(blocks, include_str!("structure/drape_1.ll")),
        rgen_llama::parse(blocks, include_str!("structure/drape_2.ll")),
        rgen_llama::parse(blocks, include_str!("structure/drape_3.ll")),
      ],
    }
  }
}

impl Placer for Sakura {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let height = rng.rand_inclusive(5, 8);

    // Checks if tree will breach build height
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return;
    }

    // Checks if tree will be built on air
    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos).block != Block::AIR {
      return;
    }

    if self.large_size {
      // Huge trees dude like the big ones man
      // Options are: tri, split_duo, duo, uno, uno_off
      self.tri_build(world, pos, rng);
    } else {
      for y in 0..=height {
        // Future options: split_duo_pint, duo, unod, uno_off_pint
        // Builds the trunk.
        world.set(pos + Pos::new(0, y, 0), self.trunk);
      }
      self.build_cannopy(world, pos + Pos::new(0, height, 0), rng);
    }
  }
}

impl Sakura {
  fn build_cannopy(&self, world: &mut PartialWorld, pos: Pos, rng: &mut Rng) {
    // Leaf box 2
    for rel_y in -1..=2_i32 {
      for rel_x in -2..=2_i32 {
        for rel_z in -2..=2_i32 {
          if world.get(pos + Pos::new(rel_x, rel_y, rel_z)) == BlockState::AIR {
            world.set(pos + Pos::new(rel_x, rel_y, rel_z), self.leaves);
          }
        }
      }
    }

    // Leaf rim
    for (rel_x, rel_z, rotation) in [(0, 1, 0), (-1, 0, 1), (0, -1, 2), (1, 0, 3)] {
      self.build_drape(world, pos, rel_x, rel_z, rotation, rng);
    }

    // Crown
    for x in -1..=1_i32 {
      for z in -1..=1_i32 {
        // Remove the corners.
        if (x.abs() == 1 && z.abs() == 1) && !(rng.rand_inclusive(0, 4) == 0) {
          continue;
        }

        if world.get(pos + Pos::new(x, 3, z)) == BlockState::AIR {
          world.set(pos + Pos::new(x, 3, z), self.leaves);
        }
      }
    }
  }

  fn build_drape(
    &self,
    world: &mut PartialWorld,
    pos: Pos,
    dx: i32,
    dz: i32,
    rotation: i32,
    rng: &mut Rng,
  ) {
    if self.drapes.is_empty() {
      return;
    }

    let mut drape = rng.choose(&self.drapes).clone();
    drape.rotate(rotation);
    // Listen. I don't want to know why this works. I shouldn't need to know why
    // this works. But it does.
    world.place_structure(pos + Pos::new(dx * 3 - dz.abs() * 2, -2, dz * 3 - dx.abs() * 2), &drape);
  }

  fn tri_build(&self, world: &mut PartialWorld, pos: Pos, rng: &mut Rng) {}
}
