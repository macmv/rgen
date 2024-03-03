use std::ops::BitOr;

use smallvec::SmallVec;

use crate::{Block, BlockState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockSet {
  /// Matches any block.
  All,

  /// Matches any of the given sets.
  Any(Vec<BlockSet>),

  /// Matches any state of the given block.
  Block(SmallVec<[Block; 4]>),

  /// Matches the specific block state.
  BlockState(SmallVec<[BlockState; 2]>),
}

impl From<Block> for BlockSet {
  fn from(value: Block) -> Self {
    BlockSet::Block(SmallVec::from_buf_and_len([value, Block::AIR, Block::AIR, Block::AIR], 1))
  }
}
impl From<BlockState> for BlockSet {
  fn from(value: BlockState) -> Self {
    BlockSet::BlockState(SmallVec::from_buf_and_len([value, BlockState::AIR], 1))
  }
}

impl<const N: usize> From<[Block; N]> for BlockSet {
  fn from(value: [Block; N]) -> Self { BlockSet::Block(SmallVec::from_slice(&value)) }
}
impl From<&[Block]> for BlockSet {
  fn from(value: &[Block]) -> Self { BlockSet::Block(SmallVec::from_slice(value)) }
}

impl<const N: usize> From<[BlockState; N]> for BlockSet {
  fn from(value: [BlockState; N]) -> Self { BlockSet::BlockState(SmallVec::from_slice(&value)) }
}
impl From<&[BlockState]> for BlockSet {
  fn from(value: &[BlockState]) -> Self { BlockSet::BlockState(SmallVec::from_slice(value)) }
}

impl BitOr for BlockSet {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (BlockSet::All, _) => BlockSet::All,
      (_, BlockSet::All) => BlockSet::All,

      (BlockSet::Block(mut a), BlockSet::Block(b)) => {
        a.extend(b);
        BlockSet::Block(a)
      }
      (BlockSet::BlockState(mut a), BlockSet::BlockState(b)) => {
        a.extend(b);
        BlockSet::BlockState(a)
      }

      (BlockSet::Block(b), BlockSet::BlockState(s))
        if b.len() == 1 && s.iter().all(|s| s.block == b[0]) =>
      {
        BlockSet::Block(b)
      }
      (BlockSet::BlockState(s), BlockSet::Block(b))
        if b.len() == 1 && s.iter().all(|s| s.block == b[0]) =>
      {
        BlockSet::Block(b)
      }

      (mut a, BlockSet::Any(b)) => {
        for b in b {
          a = a | b;
        }
        a
      }

      (BlockSet::Any(a), mut b) => {
        for a in a {
          b = b | a;
        }
        b
      }

      (a, b) => BlockSet::Any(vec![a, b]),
    }
  }
}

impl BlockSet {
  pub fn contains(&self, state: BlockState) -> bool {
    match self {
      BlockSet::All => true,
      BlockSet::Any(b) => b.iter().any(|b| b.contains(state)),
      BlockSet::Block(b) => b.iter().any(|b| *b == state.block),
      BlockSet::BlockState(b) => b.iter().any(|s| *s == state),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn block_set_or() {
    let a = BlockSet::from(Block(0));
    let b = BlockSet::from(Block(1));

    assert_eq!(a | b, BlockSet::Block(SmallVec::from_slice(&[Block(0), Block(1)])));

    let a = BlockSet::from(BlockState { block: Block(0), state: 0 });
    let b = BlockSet::from(BlockState { block: Block(0), state: 1 });

    assert_eq!(
      a | b,
      BlockSet::BlockState(SmallVec::from_slice(&[
        BlockState { block: Block(0), state: 0 }.into(),
        BlockState { block: Block(0), state: 1 }.into(),
      ]))
    );

    let a = BlockSet::from(BlockState { block: Block(0), state: 0 });
    let b = BlockSet::from(BlockState { block: Block(0), state: 1 });
    let c = BlockSet::from(Block(0));

    assert_eq!(a | b | c, BlockSet::Block(SmallVec::from_slice(&[Block(0)])));
  }

  #[test]
  fn block_set_contains() {
    let a = BlockSet::from(Block(0));
    let b = BlockSet::from(Block(1));

    assert!(a.contains(BlockState { block: Block(0), state: 0 }));
    assert!(!a.contains(BlockState { block: Block(1), state: 0 }));
    assert!(b.contains(BlockState { block: Block(1), state: 0 }));
    assert!(!b.contains(BlockState { block: Block(0), state: 0 }));

    let a = BlockSet::from(BlockState { block: Block(0), state: 0 });
    let b = BlockSet::from(BlockState { block: Block(0), state: 1 });

    assert!(a.contains(BlockState { block: Block(0), state: 0 }));
    assert!(b.contains(BlockState { block: Block(0), state: 1 }));
    assert!(!a.contains(BlockState { block: Block(0), state: 1 }));
    assert!(!b.contains(BlockState { block: Block(0), state: 0 }));
    assert!(!a.contains(BlockState { block: Block(1), state: 0 }));
    assert!(!b.contains(BlockState { block: Block(1), state: 0 }));

    let a = BlockSet::All;

    assert!(a.contains(BlockState { block: Block(0), state: 0 }));
    assert!(a.contains(BlockState { block: Block(0), state: 1 }));
    assert!(a.contains(BlockState { block: Block(1), state: 0 }));
  }
}
