//! All the tools to edit blocks in a world.

use crate::{PartialWorld, PartialWorldStorage, StagedWorldStorage};
use rgen_base::{BlockKind, BlockState, Chunk, ChunkPos, Pos, StateId};
use rgen_llama::Structure;

impl StagedWorldStorage {
  pub(crate) fn chunk(&self, pos: ChunkPos) -> Option<&Chunk> {
    self.chunks.get(&pos).map(|c| &c.chunk)
  }

  pub(crate) fn chunk_mut(&mut self, chunk_pos: ChunkPos) -> Option<&mut Chunk> {
    self.chunks.get_mut(&chunk_pos).map(|c| &mut c.chunk)
  }
}

impl PartialWorldStorage for &mut StagedWorldStorage {
  fn get(&self, pos: Pos) -> StateId {
    if let Some(chunk) = self.chunk(pos.chunk()) {
      chunk.get(pos.chunk_rel())
    } else {
      // TODO: Log a warning when reading outside the world.
      StateId::AIR
    }
  }

  fn set(&mut self, pos: Pos, block: StateId) {
    if let Some(chunk) = self.chunk_mut(pos.chunk()) {
      chunk.set(pos.chunk_rel(), block);
    } else {
      // TODO: Log a warning when writing outside the world.
    }
  }
}

impl PartialWorld<'_> {
  // FIXME: Return a `BlockInfo`, so that we can compare against default state and
  // such.
  pub fn get(&self, pos: Pos) -> BlockState { self.info.decode(self.storage.get(pos)) }

  pub fn set(&mut self, pos: Pos, state: impl Into<BlockState>) {
    self.storage.set(pos, self.info.encode(state.into()));
  }

  // TODO: allow for an array of blocks to not be overridden
  pub fn place_structure(&mut self, pos: Pos, structure: &Structure) {
    for y in 0..structure.height() {
      for z in 0..structure.depth() {
        for x in 0..structure.width() {
          let rel_pos = Pos::new(x as i32, y as i32, z as i32);
          let block = structure.get(rel_pos);
          if block != BlockState::AIR {
            self.set(pos + rel_pos, block);
          }
        }
      }
    }
  }

  pub fn top_block(&mut self, pos: Pos) -> Pos { self.top_block_excluding(pos, &[]) }

  /// Returns the highest block that is not air and not in the `exclude` list.
  pub fn top_block_excluding(&mut self, pos: Pos, exclude: &[BlockKind]) -> Pos {
    let mut y = 255;
    while y > 0 {
      let block = self.get(pos.with_y(y)).block;
      if block != BlockKind::Air && !exclude.contains(&block) {
        break;
      }
      y -= 1;
    }
    pos.with_y(y)
  }
}
