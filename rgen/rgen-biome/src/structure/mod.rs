use rgen_base::{Chunk, ChunkPos};
use rgen_world::{BlockInfoSupplier, PartialWorld};

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

  pub fn decorate(&self, world: &mut PartialWorld, chunk_pos: ChunkPos) {
    self.village.decorate(world, chunk_pos);
  }
}
