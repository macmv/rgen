//! Generates a rectangle of chunks on the calling thread.
//!
//! `CachedWorld` is for the game: it generates chunks on its own threads, as
//! they are asked for, and never lets go of them. This is for tools, which want
//! a whole area at once, and to throw it away afterwards.

use std::collections::HashMap;

use rgen_base::{Chunk, ChunkPos};

use crate::{Context, Generator, PartialWorld, RADIUS, Stage, StagedChunk, StagedWorldStorage};

/// A rectangle of chunks, which are all finished: they are decorated, and so
/// are all of their neighbors.
pub struct Region {
  chunks: HashMap<ChunkPos, Chunk>,
}

impl Region {
  /// Generates every chunk from `min` to `max`, inclusive.
  ///
  /// This also generates the chunks around the rectangle, because decorations
  /// reach into their neighbors: the chunks in the rectangle need their
  /// neighbors decorated, and those need their own neighbors' base terrain. So
  /// the base terrain is generated `RADIUS * 2` chunks out, and decorations
  /// `RADIUS` chunks out. Those extra chunks are thrown away.
  pub fn generate(
    ctx: &Context,
    generator: &impl Generator,
    min: ChunkPos,
    max: ChunkPos,
  ) -> Region {
    let mut storage = StagedWorldStorage::new();

    for pos in rect(min, max, RADIUS * 2) {
      let mut chunk = Chunk::new();
      generator.generate_base(ctx, &mut chunk, pos);
      storage.chunks.insert(pos, StagedChunk { stage: Stage::Base, chunk });
    }

    for pos in rect(min, max, RADIUS) {
      generator.decorate(&mut PartialWorld::new(&ctx.blocks, &mut storage), pos);
      storage.chunks.get_mut(&pos).unwrap().stage = Stage::Decorated;
    }

    let chunks =
      rect(min, max, 0).map(|pos| (pos, storage.chunks.remove(&pos).unwrap().chunk)).collect();
    Region { chunks }
  }

  pub fn chunk(&self, pos: ChunkPos) -> Option<&Chunk> { self.chunks.get(&pos) }

  pub fn chunks(&self) -> impl Iterator<Item = (ChunkPos, &Chunk)> {
    self.chunks.iter().map(|(pos, chunk)| (*pos, chunk))
  }
}

/// Every chunk from `min` to `max`, inclusive, and `margin` chunks past that.
fn rect(min: ChunkPos, max: ChunkPos, margin: i32) -> impl Iterator<Item = ChunkPos> {
  (min.z - margin..=max.z + margin)
    .flat_map(move |z| (min.x - margin..=max.x + margin).map(move |x| ChunkPos::new(x, z)))
}
