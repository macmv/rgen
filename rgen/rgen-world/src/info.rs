//! This module provides info about various blocks.
//!
//! Info is sources from java (implemented in rgen-jni-impl) and cached here.

use std::{collections::HashMap, fmt::Debug, hash::Hash};

use rgen_base::{
  Biome, BiomeId, BlockData, BlockId, BlockInfo, BlockKind, BlockState, PropMap, PropType,
  PropValue, StateId, StateOrProps,
};

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
    let meta = match state.state {
      StateOrProps::Default => self.get(id).default_meta,
      StateOrProps::Meta(meta) => meta,
      StateOrProps::Props(props) => {
        let data = self.get(id);

        data
          .prop_values
          .iter()
          .enumerate()
          .find_map(|(i, d)| if *d == props { Some(i) } else { None })
          .unwrap_or_else(|| {
            prop_error(data, props);

            panic!("block {} does not have a state with the properties {props:?}", data.name)
          }) as u8
      }
    };
    StateId::new(id, meta)
  }
}

fn prop_error(data: &BlockData, props: PropMap) {
  for (k, v) in props.entries() {
    match (data.prop_types.get(k).copied(), v) {
      (None, _) => panic!("block {} does not have prop {k}, but {k} = {v} was passed", data.name),
      (Some(PropType::Bool), PropValue::Bool(_)) => {}
      (Some(PropType::Bool), _) => {
        panic!("block {} has a boolean property {k}, but {k} = {v} was passed", data.name)
      }
      (Some(PropType::Int(min, max)), PropValue::Int(v)) if v >= min && v < max => {}
      (Some(PropType::Int(min, max)), _) => {
        panic!(
          "block {} has an integer property {k} in the range {min}..{max}, but {k} = {v} was passed",
          data.name
        )
      }
      (Some(PropType::Enum(variants)), PropValue::Enum(v)) if variants.contains(&v) => {}
      (Some(PropType::Enum(variants)), _) => {
        panic!(
          "block {} has an enum property {k} with the variants {variants:?}, but {k} = {v} was passed",
          data.name
        )
      }
    }
  }
}
