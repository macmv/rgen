use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_llama::Structure;
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct AspenTree {
  pub place_above:  BlockFilter,
  pub trunk:        BlockState,
  pub leaves:       BlockState,
  pub avg_in_chunk: f64,
  pub drapes_short: Vec<Structure>,
  pub drapes_long:  Vec<Structure>,
}

impl AspenTree {
  pub fn new() -> Self {
    AspenTree {
      avg_in_chunk: 13.0, //40.0,
      place_above:  block![grass].into(),
      trunk:        block![log[2]],
      leaves:       block![rgen:leaves3[0]],
      drapes_short: vec![
        rgen_llama::parse(include_str!("structure/drape_aspen_s_0.ll")),
        rgen_llama::parse(include_str!("structure/drape_aspen_s_1.ll")),
        rgen_llama::parse(include_str!("structure/drape_aspen_s_2.ll")),
      ],
      drapes_long:  vec![
        rgen_llama::parse(include_str!("structure/drape_aspen_l_0.ll")),
        rgen_llama::parse(include_str!("structure/drape_aspen_l_1.ll")),
        rgen_llama::parse(include_str!("structure/drape_aspen_l_2.ll")),
      ],
    }
  }
}

impl Placer for AspenTree {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let height = rng.rand_inclusive(9, 11);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }

    // Builds the main body.
    for y in -3..=-1_i32 {
      for x in -2..=2_i32 {
        for z in -2..=2_i32 {
          // Remove the corners.
          if x.abs() == 2 && z.abs() == 2 && (y == -3 || y == -1 || rng.rand_inclusive(0, 4) != 0) {
            continue;
          }

          let pos = pos + Pos::new(x, y + height, z);
          if world.get(pos) == block![air] {
            world.set(pos, self.leaves);
          }
        }
      }
    }

    // Builds the peak.
    for y in 0..=1_i32 {
      for x in -1..=1_i32 {
        for z in -1..=1_i32 {
          // Remove the corners.
          if x.abs() == 1 && z.abs() == 1 && y == 1 {
            continue; // next loop
          }

          let pos = pos + Pos::new(x, y + height, z);
          if world.get(pos) == block![air] {
            world.set(pos, self.leaves);
          }
        }
      }
    }

    //Build drapes.
    self.build_drape(world, pos + Pos::new(0, height - 2, 0), rng);

    //Build capes.
    let shoulder_caps_y = height;
    let shoulder_caps = [
      Pos::new(-2, shoulder_caps_y, 0),
      Pos::new(-2, shoulder_caps_y, 0),
      Pos::new(0, shoulder_caps_y, -2),
      Pos::new(0, shoulder_caps_y, 2),
    ];
    for attempts in 0..=rng.rand_inclusive(0, 2) {
      world.set(pos + shoulder_caps[attempts as usize], self.leaves);
    }

    // Builds the trunk.
    for y in 0..=height {
      world.set(pos + Pos::new(0, y, 0), self.trunk);
    }

    Ok(())
  }
}

impl AspenTree {
  fn build_drape(&self, world: &mut PartialWorld, pos: Pos, rng: &mut Rng) {
    if self.drapes_long.is_empty() || self.drapes_short.is_empty() {
      return;
    }
    let pos_by_rotation =
      [Pos::new(-1, 0, 1), Pos::new(1, 0, -1), Pos::new(-1, 0, -1), Pos::new(-1, 0, -1)];

    let long_side_pos =
      [Pos::new(0, 0, 2), Pos::new(2, 0, 0), Pos::new(0, 0, -2), Pos::new(-2, 0, 0)];
    for rotation in 0..=3_i32 {
      //let rotation = 2;
      let mut is_long_drape = false;
      let mut drape;
      if rng.rand_inclusive(0, 2) == 0 {
        drape = rng.choose(&self.drapes_long).clone();
        is_long_drape = true;
      } else {
        drape = rng.choose(&self.drapes_short).clone();
      }

      drape.rotate(rotation);
      // Listen. I don't want to know why this works. I shouldn't need to know why
      // this works. But it does.
      if is_long_drape && rng.rand_inclusive(0, 2) != 0 {
        world.set(pos + long_side_pos[rotation as usize] + Pos::new(0, -2, 0), self.leaves);
      }

      world.place_structure(
        pos + pos_by_rotation[rotation as usize] + Pos::new(0, -(drape.height() as i32), 0),
        &drape,
      );
    }
  }
}
