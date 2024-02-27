pub mod grid;
pub mod noise;
pub mod placer;
mod rng;

use rgen_base::Pos;
use rgen_world::PartialWorld;
pub use rng::{Random, Rng};

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
  fn amount_per_chunk(&self) -> u32 { 1 }

  /// Places the blocks in the world at the given position.
  fn place(&self, world: &mut PartialWorld, rng: &mut Rng, pos: Pos);
}
