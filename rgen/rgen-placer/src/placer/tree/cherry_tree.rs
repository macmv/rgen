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
      avg_in_chunk: 1.0,
      place_above:  blocks.grass.default_state.into(),
      trunk:        blocks.rgen_log.with_data(2),
      leaves:       blocks.rgen_leaves.with_data(2),
      large_size:   true,

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
      let height = rng.rand_inclusive(5, 8);
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

  fn build_limb(
    &self,
    world: &mut PartialWorld,
    start_pos: Pos,
    offset: i32,
    distance: i32,
    height: i32,
    multiplyer: i32,
    x_axis: bool,
  ) {
    let (x1, y1, x2, y2) = (0, -1, (distance + 1) * multiplyer, height - offset);

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    while x != x2 || y != y2 {
      if x_axis {
        if world.get(start_pos + Pos::new(x, y + offset, 0)) == BlockState::AIR {
          world.set(start_pos + Pos::new(x, y + offset, 0), self.trunk);
        }
      } else {
        if world.get(start_pos + Pos::new(0, y + offset, x)) == BlockState::AIR {
          world.set(start_pos + Pos::new(0, y + offset, x), self.trunk);
        }
      }

      let e2 = 2 * err;
      if e2 > -dy {
        err -= dy;
        x += sx;
      }
      if e2 < dx {
        err += dx;
        y += sy;
      }
    }
  }

  fn tri_build(&self, world: &mut PartialWorld, pos: Pos, rng: &mut Rng) {
    //let base_height = 3;
    let top = 8;
    let a_start: i32;
    let b_start: i32;
    if 0 == rng.rand_inclusive(0, 1) {
      a_start = 4;
      b_start = 5;
    } else {
      a_start = 5;
      b_start = 4;
    }
    let a = rng.rand_inclusive(2, 4);
    let b = rng.rand_inclusive(2, 6 - a);

    let a_height = top - rng.rand_inclusive(0, 1);
    let b_height = top - rng.rand_inclusive(0, 1);

    let x_axis = true;

    let a_pos: Pos;
    let b_pos: Pos;

    if x_axis {
      a_pos = pos + Pos::new(a + 1, a_height, 0);
      b_pos = pos + Pos::new((b + 1) * -1, b_height, 0);
    } else {
      a_pos = pos + Pos::new(0, a_height, a + 1);
      b_pos = pos + Pos::new(0, b_height, (b + 1) * -1);
    }
    let top_pos = pos + Pos::new(0, top, 0);

    let a_start_pos = pos + Pos::new(0, a_start, 0);
    let b_start_pos = pos + Pos::new(0, b_start, 0);

    for y in 0..=top {
      if world.get(pos + Pos::new(0, y, 0)) == BlockState::AIR
        || world.get(pos + Pos::new(0, y, 0)) == self.leaves
      {
        world.set(pos + Pos::new(0, y, 0), self.trunk);
      }
    }

    if world.get(a_pos) == BlockState::AIR {
      world.set(a_pos, self.trunk);
    }
    if world.get(b_pos) == BlockState::AIR {
      world.set(b_pos, self.trunk);
    }

    self.build_limb(world, pos, a_start, a, a_height, 1, x_axis);
    self.build_limb(world, pos, b_start, b, b_height, -1, x_axis);

    self.build_cannopy(world, a_pos, rng);
    self.build_cannopy(world, b_pos, rng);
    self.build_cannopy(world, top_pos, rng);
  }
}
