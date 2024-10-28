use rgen_base::{BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Random, Result, Rng};

pub struct Monument {
  pub material:       BlockState,
  pub fancy_material: BlockState,
  pub reward:         BlockState,
}

impl Placer for Monument {
  fn radius(&self) -> u8 { 2 }

  fn avg_per_chunk(&self) -> f64 { 0.2 }

  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result {
    let height = rng.rand_inclusive(4, 9);

    if pos.y + height + 2 >= 255 || pos.y <= 1 {
      return Err(UndoError);
    }

    //sets base
    for rel_x in -1..=1_i32 {
      for rel_z in -1..=1_i32 {
        world.set(Pos::new(pos.x + rel_x, pos.y - 1, pos.z + rel_z), self.material);
      }
    }

    //structure
    for rel_y in 0..height {
      let mut air_count = 0;
      for rel_x in -1..=1_i32 {
        for rel_z in -1..=1_i32 {
          if rel_x == 0 && rel_z == 0 {
            world.set(pos + Pos::new(rel_x, rel_y, rel_z), self.material);
          } else {
            let chance = rng.rand_inclusive(0, 100);
            if chance < 80 {
              world.set(pos + Pos::new(rel_x, rel_y, rel_z), self.material);
            } else if chance < 90 {
              world.set(pos + Pos::new(rel_x, rel_y, rel_z), self.fancy_material);
            } else if chance < 100 {
              if air_count > 2 {
                world.set(pos + Pos::new(rel_x, rel_y, rel_z), self.fancy_material);
              }
              air_count += 1;
            }
          }
        }
      }
    }

    world.set(pos, self.reward);

    Ok(())
  }
}
