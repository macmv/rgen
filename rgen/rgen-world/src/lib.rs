use std::num::NonZeroUsize;

use lru::LruCache;
use rgen_base::{Biomes, Blocks, Chunk, ChunkPos, Pos};

mod block;

pub struct Context {
  pub seed:   u64,
  pub blocks: Blocks,
  pub biomes: Biomes,
}

impl Context {
  pub fn new_test(seed: u64) -> Context {
    Context { seed, blocks: Blocks::test_blocks(), biomes: Biomes::test_blocks() }
  }
}

pub trait Generator {
  // FIXME: This is only used for rgen-viewer, it kinda needs reworking.
  fn height_at(&self, pos: Pos) -> f64;
  fn generate_biomes(&self, chunk_pos: ChunkPos, biomes: &mut [u8; 256]);

  fn generate_base(&self, ctx: &Context, chunk: &mut Chunk, pos: ChunkPos);
  fn decorate(&self, ctx: &Context, world: &mut PartialWorld, pos: ChunkPos);
}

pub struct PartialWorld {
  chunks: LruCache<ChunkPos, StagedChunk>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
  Base,
  Decorated,
  NeighborDecorated,
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
const CACHE_SIZE: usize = 512;

/// The maximum radius of a single decoration, in chunks.
const RADIUS: i32 = 1;

impl PartialWorld {
  pub fn new() -> PartialWorld {
    PartialWorld { chunks: LruCache::new(NonZeroUsize::new(CACHE_SIZE).unwrap()) }
  }

  pub fn generate(&mut self, ctx: &Context, generator: &impl Generator, pos: ChunkPos) -> &Chunk {
    let chunk = self.generate_base(ctx, generator, pos);

    if chunk.stage != Some(Stage::NeighborDecorated) {
      chunk.stage = Some(Stage::NeighborDecorated);

      for x in -RADIUS * 2..=RADIUS * 2 {
        for z in -RADIUS * 2..=RADIUS * 2 {
          self.generate_base(ctx, generator, pos + ChunkPos::new(x, z));
        }
      }

      for x in -RADIUS * 2..=RADIUS * 2 {
        for z in -RADIUS * 2..=RADIUS * 2 {
          self.generate_decorated(ctx, generator, pos + ChunkPos::new(x, z));
        }
      }
    }

    &self.chunks.get(&pos).unwrap().chunk
  }

  fn generate_decorated(
    &mut self,
    ctx: &Context,
    generator: &impl Generator,
    pos: ChunkPos,
  ) -> &mut StagedChunk {
    let chunk = self.generate_base(ctx, generator, pos);

    if chunk.stage == Some(Stage::Base) {
      chunk.stage = Some(Stage::Decorated);
      generator.decorate(ctx, self, pos);
    }

    self.chunks.get_mut(&pos).unwrap()
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
