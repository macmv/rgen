//! This module provides info about various blocks.
//!
//! Info is sources from java (implemented in rgen-jni-impl) and cached here.

use std::{collections::HashMap, num::NonZero};

use parking_lot::RwLock;
use rgen_base::{block_kind, Block, BlockId, BlockInfo, BlockState, StateId, StateOrDefault};

pub trait BlockInfoSupplier {
  fn lookup(&self, kind: Block) -> Option<BlockId>;
  fn get(&self, id: BlockId) -> BlockInfo;

  // FIXME: Return `BlockInfo` instead of `BlockState`.
  fn decode(&self, state: StateId) -> BlockState {
    let info = self.get(state.block());
    BlockState { block: info.block, state: StateOrDefault::new(state.meta()) }
  }
  fn encode(&self, state: BlockState) -> StateId {
    let id = self.lookup(state.block).unwrap();
    let meta = match state.state.state() {
      Some(meta) => meta,
      None => self.get(id).default_meta,
    };
    StateId::new(id, meta)
  }
}

impl<T: BlockInfoSupplier> BlockInfoSupplier for &T {
  fn lookup(&self, kind: Block) -> Option<BlockId> { T::lookup(*self, kind) }
  fn get(&self, id: BlockId) -> BlockInfo { T::get(*self, id) }
}

impl BlockInfoSupplier for Box<dyn BlockInfoSupplier + Send + Sync> {
  fn lookup(&self, kind: Block) -> Option<BlockId> { BlockInfoSupplier::lookup(&**self, kind) }
  fn get(&self, id: BlockId) -> BlockInfo { BlockInfoSupplier::get(&**self, id) }
}

#[derive(Default)]
pub struct BlockInfoCache<T> {
  lookup: RwLock<HashMap<Block, Option<NonZero<u16>>>>,
  info:   RwLock<HashMap<BlockId, BlockInfo>>,

  supplier: T,
}

impl<T: Default> BlockInfoCache<T> {
  pub fn new() -> Self { BlockInfoCache::default() }
}

impl<T: BlockInfoSupplier> BlockInfoSupplier for BlockInfoCache<T> {
  fn lookup(&self, kind: Block) -> Option<BlockId> {
    // Air is constant, so we don't cache it. This lets us cache an
    // `Option<NonZero<u16>>`, which is the same size as a `u16`.
    if kind == block_kind![air] {
      return Some(BlockId::AIR);
    }

    let read = self.lookup.read();
    match read.get(&kind) {
      Some(id) => id.map(|id| BlockId(id.get())), // Fast path.
      None => {
        drop(read); // Unlock the read lock.
        match self.supplier.lookup(kind) {
          // This should never happen. TODO: Log an error!
          Some(BlockId::AIR) => return Some(BlockId::AIR),
          id => {
            let mut lookup = self.lookup.write();
            lookup.insert(kind, id.map(|id| NonZero::new(id.0).unwrap()));
            id
          }
        }
      }
    }
  }

  fn get(&self, id: BlockId) -> BlockInfo {
    let read = self.info.read();
    match read.get(&id) {
      Some(info) => info.clone(),
      None => {
        drop(read); // Unlock the read lock.
        let res = self.supplier.get(id);
        let mut lookup = self.info.write();
        lookup.insert(id, res.clone());
        res
      }
    }
  }
}
