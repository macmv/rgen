use rgen_base::{Block, BlockFilter, BlockState, Blocks, Pos};
use rgen_world::PartialWorld;

use crate::{Placer, Random, Rng};

pub struct LavaLake {
  pub ground:       BlockFilter,
  pub material:     BlockState,
  pub avg_in_chunk: f64,
  pub fluid:        BlockState,
}

impl LavaLake {
  pub fn new(blocks: &Blocks) -> Self {
    LavaLake {
      ground:       [blocks.stone.block, blocks.dirt.block, blocks.grass.block].into(),
      material:     blocks.rgen_basalt.with_data(0),
      avg_in_chunk: 0.1,
      fluid:        blocks.lava.default_state,
    }
  }
}

impl Placer for LavaLake {
  fn radius(&self) -> u8 { 16 }

  fn avg_per_chunk(&self) -> f64 { self.avg_in_chunk }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) {
    if pos.y + 20 >= 255 || pos.y <= 1 {
      return;
    }

    self.build_base(rng, pos + Pos::new(0, -1, 0), world);
  }
}

impl LavaLake {
  fn build_base(&self, rng: &mut Rng, pos: Pos, world: &mut PartialWorld) {
    let poolsize: [i32; 4] = [4, 5, 6, 7];
    let x_shift = rng.rand_inclusive(8, 12) as f64 / 10.0;
    let z_shift = rng.rand_inclusive(8, 12) as f64 / 10.0;

    for rel_x in -8..=8_i32 {
      for rel_z in -8..=8_i32 {
        let mut noise_x = rel_x as f64;
        let mut noise_z = rel_z as f64;

        noise_x *= x_shift;
        noise_z *= z_shift;

        let distance_from_center = noise_x.powi(2) + noise_z.powi(2);

        if distance_from_center <= poolsize[2].pow(2) as f64
          && distance_from_center >= poolsize[1].pow(2) as f64
        {
          world.set(pos + Pos::new(rel_x, 0, rel_z), self.material);
          self.feature_add(rng, pos, world, rel_x, rel_z);
        }

        noise_x += rng.rand_inclusive(-1, 1) as f64;
        noise_z += rng.rand_inclusive(-1, 1) as f64;

        let distance_from_center = noise_x.powi(2) + noise_z.powi(2);

        if distance_from_center <= poolsize[3].pow(2) as f64
          && distance_from_center >= poolsize[0].pow(2) as f64
        {
          world.set(pos + Pos::new(rel_x, 0, rel_z), self.material);
          self.feature_add(rng, pos, world, rel_x, rel_z);
        }

        if distance_from_center <= poolsize[2].pow(2) as f64 {
          let pos_below = pos + Pos::new(rel_x, -1, rel_z);
          let pos_rel = pos + Pos::new(rel_x, 0, rel_z);

          if self.ground.contains(world.get(pos_rel))
            || (world.get(pos_rel).block == Block::AIR || world.get(pos_rel).block == Block::WATER)
          {
            world.set(pos_rel, self.fluid);
            world.set(pos_below, self.material)
          }
        }
      }
    }
  }
  fn flat_plate(&self, _rng: &mut Rng, pos: Pos, world: &mut PartialWorld) {
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        for rel_y in 0..-2_i32 {
          let rel_pos = pos + Pos::new(rel_x, rel_y, rel_z);
          if world.get(rel_pos).block == Block::AIR || world.get(rel_pos).block == Block::WATER {
            world.set(rel_pos, self.material);
          }
        }
      }
    }
  }

  fn feature_add(&self, rng: &mut Rng, pos: Pos, world: &mut PartialWorld, rel_x: i32, rel_z: i32) {
    //adds the flatplates if there is ground missing below
    //world.set(pos + Pos::new(rel_x, -1, rel_z), self.fluid);
    if world.get(pos + Pos::new(rel_x, -1, rel_z)).block == Block::AIR
      || world.get(pos + Pos::new(rel_x, -1, rel_z)).block == Block::WATER
    {
      self.flat_plate(rng, pos + Pos::new(0, -1, 0), world);
    }
    //adds little spike pillars
    if rng.rand_inclusive(0, 25) == 0 {
      world.set(pos + Pos::new(rel_x, 1, rel_z), self.material);
      if rng.rand_inclusive(0, 4) == 0 {
        world.set(pos + Pos::new(rel_x, 2, rel_z), self.material);
      }
    }
  }
}
