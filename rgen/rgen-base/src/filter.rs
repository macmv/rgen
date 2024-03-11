use std::ops::BitOr;

use smallvec::SmallVec;

use crate::{Block, BlockState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockFilter {
  /// Matches any block.
  All,

  /// Matches any of the given sets.
  Any(Vec<BlockFilter>),

  /// Matches any state of the given block.
  Block(SmallVec<[Block; 4]>),

  /// Matches the specific block state.
  BlockState(SmallVec<[BlockState; 2]>),
}

impl From<Block> for BlockFilter {
  fn from(value: Block) -> Self {
    BlockFilter::Block(SmallVec::from_buf_and_len([value, Block::AIR, Block::AIR, Block::AIR], 1))
  }
}
impl From<BlockState> for BlockFilter {
  fn from(value: BlockState) -> Self {
    BlockFilter::BlockState(SmallVec::from_buf_and_len([value, BlockState::AIR], 1))
  }
}

impl<const N: usize> From<[Block; N]> for BlockFilter {
  fn from(value: [Block; N]) -> Self { BlockFilter::Block(SmallVec::from_slice(&value)) }
}
impl From<&[Block]> for BlockFilter {
  fn from(value: &[Block]) -> Self { BlockFilter::Block(SmallVec::from_slice(value)) }
}

impl<const N: usize> From<[BlockState; N]> for BlockFilter {
  fn from(value: [BlockState; N]) -> Self { BlockFilter::BlockState(SmallVec::from_slice(&value)) }
}
impl From<&[BlockState]> for BlockFilter {
  fn from(value: &[BlockState]) -> Self { BlockFilter::BlockState(SmallVec::from_slice(value)) }
}

impl BitOr for BlockFilter {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (BlockFilter::All, _) => BlockFilter::All,
      (_, BlockFilter::All) => BlockFilter::All,

      (BlockFilter::Block(mut a), BlockFilter::Block(b)) => {
        a.extend(b);
        BlockFilter::Block(a)
      }
      (BlockFilter::BlockState(mut a), BlockFilter::BlockState(b)) => {
        a.extend(b);
        BlockFilter::BlockState(a)
      }

      (BlockFilter::Block(b), BlockFilter::BlockState(s))
        if b.len() == 1 && s.iter().all(|s| s.block == b[0]) =>
      {
        BlockFilter::Block(b)
      }
      (BlockFilter::BlockState(s), BlockFilter::Block(b))
        if b.len() == 1 && s.iter().all(|s| s.block == b[0]) =>
      {
        BlockFilter::Block(b)
      }

      (mut a, BlockFilter::Any(b)) => {
        for b in b {
          a = a | b;
        }
        a
      }

      (BlockFilter::Any(a), mut b) => {
        for a in a {
          b = b | a;
        }
        b
      }

      (a, b) => BlockFilter::Any(vec![a, b]),
    }
  }
}

impl BlockFilter {
  pub fn contains(&self, state: BlockState) -> bool {
    match self {
      BlockFilter::All => true,
      BlockFilter::Any(b) => b.iter().any(|b| b.contains(state)),
      BlockFilter::Block(b) => b.iter().any(|b| *b == state.block),
      BlockFilter::BlockState(b) => b.iter().any(|s| *s == state),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn block_set_or() {
    let a = BlockFilter::from(Block(0));
    let b = BlockFilter::from(Block(1));

    assert_eq!(a | b, BlockFilter::Block(SmallVec::from_slice(&[Block(0), Block(1)])));

    let a = BlockFilter::from(BlockState { block: Block(0), state: 0 });
    let b = BlockFilter::from(BlockState { block: Block(0), state: 1 });

    assert_eq!(
      a | b,
      BlockFilter::BlockState(SmallVec::from_slice(&[
        BlockState { block: Block(0), state: 0 }.into(),
        BlockState { block: Block(0), state: 1 }.into(),
      ]))
    );

    let a = BlockFilter::from(BlockState { block: Block(0), state: 0 });
    let b = BlockFilter::from(BlockState { block: Block(0), state: 1 });
    let c = BlockFilter::from(Block(0));

    assert_eq!(a | b | c, BlockFilter::Block(SmallVec::from_slice(&[Block(0)])));
  }

  #[test]
  fn block_set_contains() {
    let a = BlockFilter::from(Block(0));
    let b = BlockFilter::from(Block(1));

    assert!(a.contains(BlockState { block: Block(0), state: 0 }));
    assert!(!a.contains(BlockState { block: Block(1), state: 0 }));
    assert!(b.contains(BlockState { block: Block(1), state: 0 }));
    assert!(!b.contains(BlockState { block: Block(0), state: 0 }));

    let a = BlockFilter::from(BlockState { block: Block(0), state: 0 });
    let b = BlockFilter::from(BlockState { block: Block(0), state: 1 });

    assert!(a.contains(BlockState { block: Block(0), state: 0 }));
    assert!(b.contains(BlockState { block: Block(0), state: 1 }));
    assert!(!a.contains(BlockState { block: Block(0), state: 1 }));
    assert!(!b.contains(BlockState { block: Block(0), state: 0 }));
    assert!(!a.contains(BlockState { block: Block(1), state: 0 }));
    assert!(!b.contains(BlockState { block: Block(1), state: 0 }));

    let a = BlockFilter::All;

    assert!(a.contains(BlockState { block: Block(0), state: 0 }));
    assert!(a.contains(BlockState { block: Block(0), state: 1 }));
    assert!(a.contains(BlockState { block: Block(1), state: 0 }));
  }
}
