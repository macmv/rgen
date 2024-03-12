mod cheese;
mod noodle;

use cheese::CheeseCarver;
use noodle::NoodleCarver;
use rgen_base::{Chunk, ChunkPos};

use crate::biome::IdContext;

pub struct CaveCarver {
  noodle: NoodleCarver,
  cheese: CheeseCarver,
}

impl CaveCarver {
  pub fn new(ctx: &IdContext) -> Self {
    CaveCarver { noodle: NoodleCarver::new(ctx), cheese: CheeseCarver::new(ctx) }
  }

  pub fn carve(&self, seed: u64, chunk: &mut Chunk, pos: ChunkPos) {
    self.noodle.carve(seed, chunk, pos);
    self.cheese.carve(seed, chunk, pos);
  }
}
