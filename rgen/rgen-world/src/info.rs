//! This module provides info about various blocks.
//!
//! Info is sources from java (implemented in rgen-jni-impl) and cached here.

use std::{collections::HashMap, fmt::Debug, hash::Hash};

use rgen_base::{Biome, BiomeId, BlockData, BlockId, BlockInfo, BlockKind, BlockState, StateId};

pub struct InfoSupplier<K, I, D> {
  pub lookup: HashMap<K, I>,
  pub info:   HashMap<I, D>,
}

impl<K, I, D> Default for InfoSupplier<K, I, D> {
  fn default() -> Self { Self { lookup: HashMap::new(), info: HashMap::new() } }
}

pub trait InfoKey {
  const AIR: Self;
}
pub trait InfoId: Copy {
  const AIR: Self;
}

impl InfoKey for BlockKind {
  const AIR: BlockKind = BlockKind::Air;
}
impl InfoId for BlockId {
  const AIR: BlockId = BlockId::AIR;
}

impl InfoKey for Biome {
  const AIR: Biome = Biome::Void;
}
impl InfoId for BiomeId {
  const AIR: BiomeId = BiomeId::VOID;
}

impl<K: InfoKey + Hash + Eq, I: InfoId, D> InfoSupplier<K, I, D> {
  pub fn lookup(&self, kind: K) -> Option<I> {
    // Air is constant, so we don't cache it.
    if kind == K::AIR {
      return Some(I::AIR);
    }

    self.lookup.get(&kind).copied()
  }
}

impl<K, I: Hash + Eq + Debug, D> InfoSupplier<K, I, D> {
  pub fn get(&self, id: I) -> &D {
    self.info.get(&id).unwrap_or_else(|| {
      panic!("no such block with id {id:?}");
    })
  }
}

pub type BlockInfoSupplier = InfoSupplier<BlockKind, BlockId, BlockData>;
pub type BiomeInfoSupplier = InfoSupplier<Biome, BiomeId, ()>;

impl BlockInfoSupplier {
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
