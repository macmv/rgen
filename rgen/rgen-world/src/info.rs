//! This module provides info about various blocks.
//!
//! Info is sources from java (implemented in rgen-jni-impl) and cached here.

use std::collections::HashMap;

use rgen_base::{block_kind, BlockData, BlockId, BlockInfo, BlockKind, BlockState, StateId};

#[derive(Default)]
pub struct BlockInfoSupplier {
  pub lookup: HashMap<BlockKind, BlockId>,
  pub info:   HashMap<BlockId, BlockData>,
}

impl BlockInfoSupplier {
  pub fn lookup(&self, kind: BlockKind) -> Option<BlockId> {
    // Air is constant, so we don't cache it.
    if kind == block_kind![air] {
      return Some(BlockId::AIR);
    }

    self.lookup.get(&kind).copied()
  }

  pub fn get(&self, id: BlockId) -> &BlockData {
    self.info.get(&id).unwrap_or_else(|| {
      panic!("no such block with id {id:?}");
    })
  }

  pub fn decode(&self, state: StateId) -> BlockInfo {
    BlockInfo::new(self.get(state.block()), state)
  }

  pub fn encode(&self, state: BlockState) -> StateId {
    let id = self.lookup(state.block).unwrap();
    let meta = match state.state.state() {
      Some(meta) => meta,
      None => self.get(id).default_meta,
    };
    StateId::new(id, meta)
  }
}
