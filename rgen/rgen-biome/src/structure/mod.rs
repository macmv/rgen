use rgen_base::{Chunk, ChunkPos};

use crate::biome::IdContext;

mod village;

pub struct StructureGenerator {
  village: village::VillageGenerator,
}

impl StructureGenerator {
  pub fn new(ctx: &IdContext, seed: u64) -> Self {
    StructureGenerator { village: village::VillageGenerator::new(ctx, seed) }
  }

  pub fn generate(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    self.village.generate(chunk, chunk_pos);
  }
}
