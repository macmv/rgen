mod cheese;
mod noodle;

use cheese::CheeseCarver;
use noodle::NoodleCarver;
use rgen_base::{Chunk, ChunkPos};
use rgen_world::BlockInfoSupplier;

use crate::WorldBiomes;

pub struct CaveCarver {
  noodle: NoodleCarver,
  cheese: CheeseCarver,
}

impl CaveCarver {
  pub fn new(info: &BlockInfoSupplier, seed: u64) -> Self {
    CaveCarver { noodle: NoodleCarver::new(info, seed), cheese: CheeseCarver::new(info, seed) }
  }

  pub fn carve(&self, world: &WorldBiomes, chunk: &mut Chunk, pos: ChunkPos) {
    self.noodle.carve(chunk, pos);
    self.cheese.carve(world, chunk, pos);
  }
}
