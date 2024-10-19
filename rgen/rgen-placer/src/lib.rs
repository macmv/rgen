#![allow(clippy::new_without_default)]

mod chunk;
pub mod chunk_placer;
pub mod grid;
pub mod noise;
pub mod placer;
mod rng;

pub use chunk::*;
use rgen_base::{ChunkPos, Pos};
use rgen_world::{PartialWorld, UndoError};
pub use rng::{Random, Rng};

pub type Result = std::result::Result<(), UndoError>;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

/// A Placer places a set of blocks at a position in the world.
///
/// Placers are chunk-agnostic, and they will be called multiple times for a
/// single feature, so that a placer may build accross chunks easily.
pub trait Placer: Send + Sync {
  /// The maximum radius, in blocks in the X-Z axis, that this placer will
  /// place. This is a square around the position passed to `place`.
  fn radius(&self) -> u8;

  /// The amount of times, on average, that this placer should be run for each
  /// chunk.
  fn avg_per_chunk(&self) -> f64 { 1.0 }

  /// Places the blocks in the world at the given position.
  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos) -> Result;
}

/// A ChunkPlacer places a set of decorations on a single chunk.
///
/// This is less flexible than a `Placer`, because it can only access a single
/// chunk, but it ends up being faster, as it will be run in parallel.
pub trait ChunkPlacer: Send + Sync {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos);
}
