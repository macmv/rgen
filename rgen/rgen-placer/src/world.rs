use rgen_base::{Block, Chunk, ChunkPos, Pos};

/// A World is a proxy around a specific chunk, that lets you "place" a block in
/// the world, but will end up only setting blocks in that one chunk.
pub struct World<'a> {
  chunk_pos: ChunkPos,
  chunk:     &'a mut Chunk,
}

impl<'a> World<'a> {
  pub fn set(&mut self, pos: Pos, block: Block) {
    if pos.in_chunk(self.chunk_pos) {
      self.chunk.set(pos.chunk_rel(), block);
    }
  }
}
