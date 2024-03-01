//! All the tools to edit blocks in a world.

use crate::PartialWorld;
use rgen_base::{Block, BlockState, Chunk, ChunkPos, Pos};

impl PartialWorld {
  fn chunk(&mut self, pos: ChunkPos) -> Option<&Chunk> { self.chunks.get(&pos).map(|c| &c.chunk) }
  fn chunk_mut(&mut self, chunk_pos: ChunkPos) -> Option<&mut Chunk> {
    self.chunks.get_mut(&chunk_pos).map(|c| &mut c.chunk)
  }

  pub fn get(&mut self, pos: Pos) -> Block { self.get_state(pos).block }

  pub fn get_state(&mut self, pos: Pos) -> BlockState {
    if let Some(chunk) = self.chunk(pos.chunk()) {
      chunk.get_state(pos.chunk_rel())
    } else {
      // TODO: Log a warning when reading outside the world.
      BlockState::AIR
    }
  }

  pub fn set(&mut self, pos: Pos, block: Block) {
    self.set_state(pos, BlockState { block, state: 0 });
  }

  pub fn set_state(&mut self, pos: Pos, block: BlockState) {
    if let Some(chunk) = self.chunk_mut(pos.chunk()) {
      chunk.set_state(pos.chunk_rel(), block);
    } else {
      // TODO: Log a warning when writing outside the world.
    }
  }

  pub fn top_block(&mut self, pos: Pos) -> Pos { self.top_block_excluding(pos, &[]) }

  /// Returns the highest block that is not air and not in the `exclude` list.
  pub fn top_block_excluding(&mut self, pos: Pos, exclude: &[Block]) -> Pos {
    let mut y = 255;
    while y > 0 {
      let block = self.get(pos.with_y(y));
      if block != Block::AIR && !exclude.contains(&block) {
        break;
      }
      y -= 1;
    }
    pos.with_y(y)
  }
}
