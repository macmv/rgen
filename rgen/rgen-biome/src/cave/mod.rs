mod cheese;
mod noodle;

use cheese::CheeseCarver;
use noodle::NoodleCarver;
use rgen_base::{Chunk, ChunkPos};

use crate::{biome::IdContext, WorldBiomes};

pub struct CaveCarver {
  noodle: NoodleCarver,
  cheese: CheeseCarver,
}

impl CaveCarver {
  pub fn new(ctx: &IdContext, seed: u64) -> Self {
    CaveCarver { noodle: NoodleCarver::new(ctx, seed), cheese: CheeseCarver::new(ctx, seed) }
  }

  pub fn carve(&self, world: &WorldBiomes, chunk: &mut Chunk, pos: ChunkPos) {
    self.noodle.carve(chunk, pos);
    self.cheese.carve(world, chunk, pos);
  }
}
