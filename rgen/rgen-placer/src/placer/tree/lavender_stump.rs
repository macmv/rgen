use rgen_base::{BlockFilter, BlockState, Pos};
use rgen_world::{PartialWorld, UndoError};

use crate::{Placer, Result, Rng};

pub struct LavenderStump {
  pub place_above: BlockFilter,
  pub log: BlockState,
  pub avg_per_chunk: f64,
}

impl Placer for LavenderStump {
  fn radius(&self) -> u8 { 1 }
  fn avg_per_chunk(&self) -> f64 { self.avg_per_chunk }

  fn place(&self, world: &mut PartialWorld, _rng: &mut Rng, pos: Pos) -> Result {
    if pos.y < 1 || pos.y + 1 >= 255
      || !self.place_above.contains(world.get(pos + Pos::new(0,-1,0)))
      || world.get(pos) != block![air]
      || world.get(pos + Pos::new(0,1,0)) != block![air]
    {
      return Err(UndoError);
    }
    world.set(pos, self.log);
    world.set(pos + Pos::new(0,1,0), self.log);
    Ok(())
  }
}
