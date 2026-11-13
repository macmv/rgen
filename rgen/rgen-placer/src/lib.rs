#![allow(clippy::new_without_default)]

mod chunk;
pub mod chunk_placer;
pub mod grid;
pub mod icing_placer;
pub mod noise;
pub mod placer;
mod rng;

pub use chunk::*;
use rgen_base::{ChunkPos, Pos};
use rgen_world::{PartialWorld, UndoError};
pub use rng::{Random, Rng};

pub type Result = std::result::Result<(), UndoError>;

#[macro_use]
extern crate rgen_base;

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

/// An IcingPlacer places the last layer of decoration on a chunk.
///
/// Icing placers run once a chunk is final, which is when all 8 of its
/// neighbors have finished being decorated. That means every tree is already in
/// place (including ones that grew in from a neighboring chunk), so an icing
/// placer can see exactly what is above and around it, and never gets
/// overwritten by something that runs later.
///
/// In exchange, an icing placer can only edit the chunk it is placing in, and
/// it must not rely on anything a neighboring chunk's icing might have done.
///
/// This is deliberately a separate type from `ChunkPlacer`. A `ChunkPlacer`
/// runs before any tree exists, so it can't take trees into account, and an
/// `IcingPlacer` that forgot to would be a bug. Registering a placer in the
/// wrong pass should not compile.
pub trait IcingPlacer: Send + Sync {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos);
}
