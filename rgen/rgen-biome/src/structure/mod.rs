use rgen_base::{Chunk, ChunkPos};
use rgen_world::BlockInfoSupplier;

mod village;

pub struct StructureGenerator {
  village: village::VillageGenerator,
}

impl StructureGenerator {
  pub fn new(seed: u64) -> Self {
    StructureGenerator { village: village::VillageGenerator::new(seed) }
  }

  pub fn generate(&self, info: &BlockInfoSupplier, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    self.village.generate(info, chunk, chunk_pos);
  }
}
