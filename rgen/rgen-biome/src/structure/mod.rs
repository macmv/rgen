use rgen_base::{Chunk, ChunkPos};

mod village;

pub struct StructureGenerator {
  village: village::VillageGenerator,
}

impl StructureGenerator {
  pub fn new(seed: u64) -> Self {
    StructureGenerator { village: village::VillageGenerator::new(seed) }
  }

  pub fn generate(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    self.village.generate(chunk, chunk_pos);
  }
}
