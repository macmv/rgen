use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_llama::Structure;
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

#[derive(PartialEq, Debug, Clone, Copy)]
enum SplitTree {
  Tri,
  Duo,
  Uno,
}
pub struct skyJungleTree {
  pub place_above:   BlockFilter,
  pub ground:        BlockFilter,
  pub trunk:         BlockState,
  pub covered_trunk: BlockState,
  pub test1:         BlockState,
  pub test2:         BlockState,
  pub leaves:        BlockState,
  pub hanging_vines: BlockState,
  pub avg_in_chunk:  f64,
  pub large_size:    bool,
}

impl skyJungleTree {
  pub fn new() -> Self {
    skyJungleTree {
      avg_in_chunk:  8.0, //8.0
      place_above:   block![grass].into(),
      ground:        [block![grass], block![stone], block![dirt], block![log[3]]].into(),
      trunk:         block![log[3]],
      covered_trunk: block![rgen:covered_jungle_log],
      test1:         block!(iron_block),
      test2:         block!(stone),
      leaves:        block![leaves[3]],
      hanging_vines: block![rgen:hanging_vines],
      large_size:    true,
    }
  }
}

impl Placer for skyJungleTree {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    // Checks if tree will breach build height
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    // Checks if tree will be built on air
    let below_pos = pos + Pos::new(0, -1, 0);
    if !self.place_above.contains(world.get(below_pos)) || world.get(pos) != block![air] {
      return Err(UndoError);
    }

    world.set(pos + Pos::new(0, 0, 0), self.trunk);
    world.set(pos + Pos::new(0, 1, 0), self.trunk);

    let mut options = vec![(0.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)];

    let index_select = rng.rand_exclusive(0, options.len() as i32) as usize;

    let sway = options.remove(index_select);
    // Small sway
    let sway_x = rng.rand_inclusive(1.0, 1.3) * sway.0;
    let sway_z = rng.rand_inclusive(1.0, 1.3) * sway.1;
    self.low_trunk_build(world, pos + Pos::new(0, 2, 0), rng, sway_x, sway_z);

    // Far sway
    let sway2 = rng.choose(&options);
    let sway_x = rng.rand_inclusive(2.3, 2.8) * sway2.0;
    let sway_z = rng.rand_inclusive(2.3, 2.8) * sway2.1;
    self.low_trunk_build(world, pos + Pos::new(0, 2, 0), rng, sway_x, sway_z);

    self.leg_build(world, pos + Pos::new(0, 2, 0), rng, 1.0, 1.0);

    self.leg_build(world, pos + Pos::new(0, 2, 0), rng, -1.0, -1.0);

    //println!("{sway2}");
    Ok(())
  }
}

impl skyJungleTree {
  fn low_trunk_build(
    &self,
    world: &mut PartialWorld,
    pos: Pos,
    rng: &mut Rng,
    sway_x: f64,
    sway_z: f64,
  ) {
    let mut trunk_pos = pos;

    // Trunk.
    let mut x = pos.x as f64;
    let mut z = pos.z as f64;

    for i in 0..4 {
      let mut dx = sway_x * rng.rand_inclusive(-0.3, 1.0);
      let mut dz = sway_z * rng.rand_inclusive(-0.3, 1.0);

      while dx.abs() > 1.0 || dz.abs() > 1.0 {
        world.set(trunk_pos + Pos::new((x + dx) as i32, 0, (z + dz) as i32), self.trunk);
        dx /= 2.0;
        dz /= 2.0;
      }
      x += dx;
      z += dz;

      // Connect trunks on the edges, not on corners.
      if x as i32 != trunk_pos.x || z as i32 != trunk_pos.z {
        world.set(trunk_pos, self.trunk);
      }

      // Set these, so that leaves at the end go in the right spot.
      trunk_pos.x = x as i32;
      trunk_pos.z = z as i32;
      world.set(trunk_pos, self.trunk);

      trunk_pos.y += 1;
    }
    world.set(trunk_pos, self.test1);
    self.place_leaves(world, rng, trunk_pos);
  }

  fn leg_build(&self, world: &mut PartialWorld, pos: Pos, rng: &mut Rng, sway_x: f64, sway_z: f64) {
    let mut trunk_pos = pos;

    // Trunk.
    let mut x = pos.x as f64;
    let mut z = pos.z as f64;

    for i in 0..9 {
      let mut dx = sway_x * rng.rand_inclusive(-0.3, 1.0);
      let mut dz = sway_z * rng.rand_inclusive(-0.3, 1.0);

      while dx.abs() > 1.0 || dz.abs() > 1.0 {
        world.set(trunk_pos + Pos::new((x + dx) as i32, 0, (z + dz) as i32), self.trunk);
        dx /= 2.0;
        dz /= 2.0;
      }
      x += dx;
      z += dz;

      // Connect trunks on the edges, not on corners.
      if x as i32 != trunk_pos.x || z as i32 != trunk_pos.z {
        if world.get(trunk_pos + Pos::new(0, 1, 0)) == block![air] {
          world.set(trunk_pos, self.covered_trunk)
        } else {
          world.set(trunk_pos, self.trunk);
          if world.get(trunk_pos + Pos::new(0, -1, 0)) == self.covered_trunk {
            world.set(trunk_pos + Pos::new(0, -1, 0), self.trunk);
          }
        }
      }

      // Set these, so that leaves at the end go in the right spot.
      trunk_pos.x = x as i32;
      trunk_pos.z = z as i32;
      if world.get(trunk_pos + Pos::new(0, 1, 0)) == block![air] {
        world.set(trunk_pos, self.covered_trunk)
      } else {
        world.set(trunk_pos, self.trunk);
        // Checks if trunk block will be over a grass jungle block and fixes it
        if world.get(trunk_pos + Pos::new(0, -1, 0)) == self.covered_trunk {
          world.set(trunk_pos + Pos::new(0, -1, 0), self.trunk);
        }
      }

      // Ends the root when it hits the ground
      if self.ground.contains(world.get(trunk_pos + Pos::new(0, -1, 0))) {
        break;
      }

      trunk_pos.y -= 1;
    }
  }

  fn place_leaves(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let size = 3_i32;
    for x in -size..=size {
      for z in -size..=size {
        let leaf_pos = pos + Pos::new(x, 0, z);
        if (x.abs() + z.abs()) <= (size + (size / 2)) && world.get(leaf_pos) == block![air] {
          world.set(leaf_pos, self.leaves);
        }
      }
    }

    let size = 2_i32;
    for x in -size..=size {
      for z in -size..=size {
        let leaf_pos = pos + Pos::new(x, 1, z);
        if (x.abs() + z.abs()) <= (size + (size / 2)) && world.get(leaf_pos) == block![air] {
          world.set(leaf_pos, self.leaves);
        }
      }
    }

    // Add some hangign vines from the leaves.
    for (dx, dz) in [(2, 2), (-2, 2), (2, -2), (-2, -2)] {
      for i in 0..rng.rand_inclusive(0, 1) {
        self.streamer_placer(world, rng, pos + Pos::new(dx, -1, dz));
      }
    }

    // Add a couple more leaves down the trunk.
    for (dx, dz) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
      for i in 1..rng.rand_inclusive(1, 3) {
        let pos = pos + Pos::new(dx, -i, dz);
        if world.get(pos) == block![air] {
          world.set(pos, self.leaves);
        }
      }
    }
  }

  fn streamer_placer(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    let size = 1_i32;
    for x in -size..=size {
      for z in -size..=size {
        let leaf_pos = pos + Pos::new(x, 0, z);
        if (x.abs() + z.abs()) <= (size + (size / 2)) && world.get(leaf_pos) == self.leaves {
          if world.get(leaf_pos + Pos::new(0, -1, 0)) == block![air]
            && world.get(leaf_pos + Pos::new(0, -2, 0)) == block![air]
            && world.get(leaf_pos + Pos::new(0, -3, 0)) == block![air]
          {
            for i in 1..rng.rand_inclusive(4, 7) {
              let vine_pos = leaf_pos + Pos::new(0, -i, 0);
              if world.get(vine_pos + Pos::new(0, -2, 0)) != block![air]
                && 1 == rng.rand_inclusive(0, 1)
              {
                break;
              } else if world.get(vine_pos + Pos::new(0, -1, 0)) != block![air] {
                break;
              } else {
                world.set(vine_pos, self.hanging_vines)
              }
            }
          }
        }
      }
    }
  }
}
