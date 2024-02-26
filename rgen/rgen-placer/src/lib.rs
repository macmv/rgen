mod world;

use rgen_base::Pos;
pub use world::World;

/// A Placer places a set of blocks at a position in the world.
///
/// Placers are chunk-agnostic, and they will be called multiple times for a
/// single feature, so that a placer may build accross chunks easily.
pub trait Placer {
  /// The maximum radius, in blocks in the X-Z axis, that this placer will
  /// place. This is a square around the position passed to `place`.
  fn radius(&self) -> u8;

  /// Places the blocks in the world at the given position.
  fn place(&self, world: &mut World, pos: Pos);
}
