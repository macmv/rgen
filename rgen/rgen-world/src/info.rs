//! This module provides info about various blocks.
//!
//! Info is sources from java (implemented in rgen-jni-impl) and cached here.

use std::collections::HashMap;

use rgen_base::{block_kind, Block, BlockId, BlockInfo, BlockState, StateId, StateOrDefault};

#[derive(Default)]
pub struct BlockInfoSupplier {
  pub lookup: HashMap<Block, BlockId>,
  pub info:   HashMap<BlockId, BlockInfo>,
}

impl BlockInfoSupplier {
  pub fn lookup(&self, kind: Block) -> Option<BlockId> {
    // Air is constant, so we don't cache it.
    if kind == block_kind![air] {
      return Some(BlockId::AIR);
    }

    self.lookup.get(&kind).copied()
  }

  pub fn get(&self, id: BlockId) -> BlockInfo {
    if id == BlockId::AIR {
      return BlockInfo {
        name:         "air".to_string(),
        block:        Some(block_kind![air]),
        default_meta: 0,
      };
    }

    self
      .info
      .get(&id)
      .unwrap_or_else(|| {
        panic!("no such block with id {id:?}");
      })
      .clone()
  }

  // FIXME: Return `BlockInfo` instead of `BlockState`.
  pub fn decode(&self, state: StateId) -> BlockState {
    let info = self.get(state.block());
    match info.block {
      Some(block) => BlockState { block, state: StateOrDefault::new(state.meta()) },
      None => BlockState::AIR,
    }
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
