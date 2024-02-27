use std::collections::HashMap;

use rgen_base::{Chunk, ChunkPos};
use rgen_world::{Context, Generator, PartialWorld};

pub struct World<G> {
  pub context:   Context,
  pub generator: G,

  partial: PartialWorld,
  chunks:  HashMap<ChunkPos, Chunk>,
}

impl<G> World<G> {
  pub fn new(context: Context, generator: G) -> World<G> {
    World { context, generator, partial: PartialWorld::new(), chunks: HashMap::new() }
  }
}

impl<G: Generator> World<G> {
  pub fn generate_chunk(&mut self, pos: ChunkPos) {
    if !self.chunks.contains_key(&pos) {
      let chunk = self.partial.generate(&self.context, &self.generator, pos);
      self.chunks.insert(pos, chunk.clone());
    }
  }
}
