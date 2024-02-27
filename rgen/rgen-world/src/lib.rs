use std::num::NonZeroUsize;

use lru::LruCache;
use rgen_base::{Biomes, Blocks, Chunk, ChunkPos};

pub struct Context {
  pub seed:   u64,
  pub blocks: Blocks,
  pub biomes: Biomes,
}

pub trait Generator {
  fn generate_base(&self, ctx: &Context, chunk: &mut Chunk, pos: ChunkPos);
  fn decorate(&self, ctx: &Context, world: &mut PartialWorld, pos: ChunkPos);
}

pub struct PartialWorld {
  chunks: LruCache<ChunkPos, StagedChunk>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
  /// This means the chunk has the base stone, water for oceans, and top layers
  /// filled in.
  Base,

  /// This means the chunk has been decorated with trees, ores, etc.
  Decorated,
}

struct StagedChunk {
  /// No stage means the chunk is empty.
  stage: Option<Stage>,
  chunk: Chunk,
}

impl Default for StagedChunk {
  fn default() -> Self { StagedChunk { stage: None, chunk: Chunk::new() } }
}

/// The size in chunks of the partial world cache.
const CACHE_SIZE: usize = 128;

/// The maximum radius of a single decoration, in chunks.
const RADIUS: i32 = 1;

impl PartialWorld {
  pub fn new() -> PartialWorld {
    PartialWorld { chunks: LruCache::new(NonZeroUsize::new(CACHE_SIZE).unwrap()) }
  }

  pub fn generate(&mut self, ctx: &Context, generator: &impl Generator, pos: ChunkPos) -> &Chunk {
    let chunk = self.generate_base(ctx, generator, pos);

    if chunk.stage == Some(Stage::Base) {
      chunk.stage = Some(Stage::Decorated);

      for x in -RADIUS..=RADIUS {
        for z in -RADIUS..=RADIUS {
          self.generate_base(ctx, generator, pos + ChunkPos::new(x, z));
        }
      }

      generator.decorate(ctx, self, pos);
    }

    // Re-borrow the chunk, as the previous borrow got released on the `decorate`
    // call.
    &self.chunks.get(&pos).unwrap().chunk
  }

  fn generate_base(
    &mut self,
    ctx: &Context,
    generator: &impl Generator,
    pos: ChunkPos,
  ) -> &mut StagedChunk {
    let chunk = self.chunks.get_or_insert_mut(pos, StagedChunk::default);

    if chunk.stage.is_none() {
      chunk.stage = Some(Stage::Base);
      generator.generate_base(ctx, &mut chunk.chunk, pos);
    }

    chunk
  }
}
