use std::ops::BitOr;

use smallvec::SmallVec;

use crate::{BlockInfo, BlockKind, BlockState, StateOrProps};

/// A block filter is a filter for matching against blocks.
///
/// It can match individual block states (like oak stairs facing north), or
/// entire blocks (like any log block), or a combination of those (like any
/// bottom slabs or any logs).
///
/// This filter can also be set to many every block, using the `Any` variant.
///
/// The main way to use a `BlockFilter` is to check if it contains a block
/// state, using the [`contains`](BlockFilter::contains) function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockFilter {
  /// Matches any block.
  All,

  /// Matches the specific block state, or all states of the state is
  /// `StateOrDefault::DEFAULT`.
  Block(SmallVec<[BlockState; 2]>),
}

impl From<BlockKind> for BlockFilter {
  fn from(value: BlockKind) -> Self {
    BlockFilter::Block(SmallVec::from_buf_and_len(
      [
        BlockState { block: value, state: StateOrProps::Default },
        BlockState { block: BlockKind::Air, state: StateOrProps::Default },
      ],
      1,
    ))
  }
}
impl From<BlockState> for BlockFilter {
  fn from(value: BlockState) -> Self {
    BlockFilter::Block(SmallVec::from_buf_and_len([value, BlockState::AIR], 1))
  }
}

impl<const N: usize> From<[BlockState; N]> for BlockFilter {
  fn from(value: [BlockState; N]) -> Self { BlockFilter::Block(SmallVec::from_slice(&value)) }
}
impl From<&[BlockState]> for BlockFilter {
  fn from(value: &[BlockState]) -> Self { BlockFilter::Block(SmallVec::from_slice(value)) }
}

impl BitOr for BlockFilter {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (BlockFilter::All, _) => BlockFilter::All,
      (_, BlockFilter::All) => BlockFilter::All,

      (BlockFilter::Block(mut a), BlockFilter::Block(b)) => {
        for block in b {
          if a
            .iter()
            .any(|s| s.block == block.block && (s.state.is_default() || s.state == block.state))
          {
            continue;
          }
          if block.state.is_default() {
            a.retain(|s| s.block != block.block);
          }
          a.push(block);
        }
        BlockFilter::Block(a)
      }
    }
  }
}

impl BlockFilter {
  /// Checks if a block filter contains the given block state.
  ///
  /// ```
  /// # use rgen_base::{BlockKind, BlockData, BlockFilter, BlockState, StateOrProps, BlockInfo, StateId};
  /// let grass_data = BlockData {
  ///   name:         String::new(),
  ///   block:        Some(BlockKind::Grass),
  ///   default_meta: 0,
  /// };
  /// let stone_data = BlockData {
  ///   name:         String::new(),
  ///   block:        Some(BlockKind::Stone),
  ///   default_meta: 0,
  /// };
  /// let air_data = BlockData {
  ///   name:         String::new(),
  ///   block:        Some(BlockKind::Air),
  ///   default_meta: 0,
  /// };
  /// let default_grass = BlockInfo::new(&grass_data, StateId(32 | 0));
  /// let snowy_grass = BlockInfo::new(&grass_data, StateId(32 | 1));
  /// let stone = BlockInfo::new(&stone_data, StateId(16 | 0));
  /// let air = BlockInfo::new(&air_data, StateId(0));
  ///
  /// let default_grass_state = BlockState { block: BlockKind::Grass, state: StateOrProps::Default };
  /// let snowy_grass_state = BlockState { block: BlockKind::Grass, state: StateOrProps::meta(1) };
  /// let air_state = BlockState { block: BlockKind::Air, state: StateOrProps::Default };
  ///
  /// let filter: BlockFilter = [default_grass_state, air_state].into();
  ///
  /// assert!(filter.contains(default_grass));
  /// assert!(filter.contains(snowy_grass));
  /// assert!(!filter.contains(stone));
  /// assert!(filter.contains(air));
  ///
  /// assert!(!filter.contains(BlockKind::Stone));
  /// assert!(filter.contains(BlockKind::Air));
  ///
  /// let any_filter = BlockFilter::All;
  ///
  /// assert!(any_filter.contains(default_grass));
  /// assert!(any_filter.contains(snowy_grass));
  /// assert!(any_filter.contains(stone));
  /// assert!(any_filter.contains(air));
  ///
  /// assert!(any_filter.contains(BlockKind::Stone));
  /// assert!(any_filter.contains(BlockKind::Air));
  ///
  /// let snowy_filter: BlockFilter = [snowy_grass_state].into();
  /// assert!(!snowy_filter.contains(default_grass));
  /// assert!(snowy_filter.contains(snowy_grass));
  /// ```
  pub fn contains(&self, state: impl BlockFilterable + Copy) -> bool {
    match self {
      BlockFilter::All => true,
      BlockFilter::Block(b) => b.iter().any(|s| state.compare_state(s)),
    }
  }
}

pub trait BlockFilterable {
  fn block_kind(&self) -> BlockKind;
  fn compare_state(&self, other: &BlockState) -> bool;
}

impl BlockFilterable for BlockState {
  fn block_kind(&self) -> BlockKind { self.block }
  fn compare_state(&self, other: &BlockState) -> bool { self == other }
}
impl BlockFilterable for BlockInfo<'_> {
  fn block_kind(&self) -> BlockKind { self.block_kind() }
  fn compare_state(&self, other: &BlockState) -> bool { self == other }
}
impl BlockFilterable for BlockKind {
  fn block_kind(&self) -> BlockKind { *self }
  fn compare_state(&self, other: &BlockState) -> bool { other.block == *self }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{BlockData, StateId, StateOrProps};

  // NB: Other crates will write `block![]` instead of this function.
  fn block(b: BlockKind, state: u8) -> BlockState {
    BlockState { block: b, state: StateOrProps::meta(state) }
  }

  fn block_info(data: &BlockData, state: u8) -> BlockInfo {
    BlockInfo { data, state: StateId(state.into()) }
  }

  #[test]
  fn block_set_or() {
    let a = BlockFilter::from(BlockKind::Air);
    let b = BlockFilter::from(BlockKind::Stone);

    assert_eq!(
      a | b,
      BlockFilter::Block(SmallVec::from_slice(&[
        BlockState { block: BlockKind::Air, state: StateOrProps::Default },
        BlockState { block: BlockKind::Stone, state: StateOrProps::Default },
      ]))
    );

    let a = BlockFilter::from(block(BlockKind::Air, 0));
    let b = BlockFilter::from(block(BlockKind::Air, 1));

    assert_eq!(
      a | b,
      BlockFilter::Block(SmallVec::from_slice(&[
        BlockState { block: BlockKind::Air, state: StateOrProps::meta(0) },
        BlockState { block: BlockKind::Air, state: StateOrProps::meta(1) },
      ]))
    );

    let a = BlockFilter::from(block(BlockKind::Air, 0));
    let b = BlockFilter::from(block(BlockKind::Air, 1));
    let c = BlockFilter::from(BlockKind::Air);

    assert_eq!(
      a | b | c,
      BlockFilter::Block(SmallVec::from_slice(&[BlockState {
        block: BlockKind::Air,
        state: StateOrProps::Default,
      },]))
    );

    let a = BlockFilter::from(block(BlockKind::Air, 0));
    let b = BlockFilter::from(BlockKind::Air);
    let c = BlockFilter::from(block(BlockKind::Air, 1));

    assert_eq!(
      a | b | c,
      BlockFilter::Block(SmallVec::from_slice(&[BlockState {
        block: BlockKind::Air,
        state: StateOrProps::Default,
      },]))
    );

    let a = BlockFilter::from(block(BlockKind::Air, 0));
    let b = BlockFilter::from(block(BlockKind::Air, 0));

    assert_eq!(
      a | b,
      BlockFilter::Block(SmallVec::from_slice(&[BlockState {
        block: BlockKind::Air,
        state: StateOrProps::meta(0),
      },]))
    );
  }

  #[test]
  fn block_set_contains() {
    let a = BlockFilter::from(BlockKind::Air);
    let b = BlockFilter::from(BlockKind::Stone);

    let air_data = BlockData {
      name:         String::new(),
      block:        Some(BlockKind::Air),
      default_meta: 0,
    };
    let stone_data = BlockData {
      name:         String::new(),
      block:        Some(BlockKind::Stone),
      default_meta: 0,
    };

    assert!(a.contains(block_info(&air_data, 0)));
    assert!(!a.contains(block_info(&stone_data, 0)));
    assert!(b.contains(block_info(&stone_data, 0)));
    assert!(!b.contains(block_info(&air_data, 0)));

    let a = BlockFilter::from(block(BlockKind::Air, 0));
    let b = BlockFilter::from(block(BlockKind::Air, 1));

    assert!(a.contains(block_info(&air_data, 0)));
    assert!(b.contains(block_info(&air_data, 1)));
    assert!(!a.contains(block_info(&air_data, 1)));
    assert!(!b.contains(block_info(&air_data, 0)));
    assert!(!a.contains(block_info(&stone_data, 0)));
    assert!(!b.contains(block_info(&stone_data, 0)));

    let a = BlockFilter::All;

    assert!(a.contains(block_info(&air_data, 0)));
    assert!(a.contains(block_info(&air_data, 1)));
    assert!(a.contains(block_info(&stone_data, 0)));
  }
}
