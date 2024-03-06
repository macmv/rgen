use std::collections::HashMap;

use parking_lot::Mutex;
use rayon::prelude::*;
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

pub struct CachedWorld {
  base_chunks: Mutex<HashMap<ChunkPos, PartialChunk>>,

  // FIXME: Need to clean up this map once it gets full. The cleanup needs to be somewhat
  // intelligent, so this is kinda tricky.
  chunks: Mutex<PartialWorld>,
}

pub struct PartialWorld {
  /// A chunk existing in here means its either decorated or about to be
  /// decorated.
  ///
  /// This struct doesn't have any interior mutability, so a chunk existing in
  /// here means its decorated (but its neighbors aren't necessarily).
  chunks: HashMap<ChunkPos, StagedChunk>,
}

enum PartialChunk {
  // This is insertted into the `base_chunks` map while the chunk is being generated. Its a sort
  // of lock that hints to other threads not to generate this chunk.
  Building,
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

impl CachedWorld {
  pub fn new() -> CachedWorld {
    CachedWorld {
      base_chunks: Mutex::new(HashMap::new()),
      chunks:      Mutex::new(PartialWorld::new()),
    }
  }

  pub fn generate<R>(
    &self,
    ctx: &Context,
    generator: &(impl Generator + Send + Sync),
    pos: ChunkPos,
    f: impl FnOnce(&Chunk) -> R,
  ) -> R {
    let needs_generate = {
      let w = self.chunks.lock();
      let chunk = w.chunks.get(&pos);
      chunk.map(|c| c.stage != Some(Stage::NeighborDecorated)).unwrap_or(true)
    };

    if needs_generate {
      let width = RADIUS * 2 * 2 + 1;
      (0..width.pow(2)).into_par_iter().for_each(|i| {
        let x = i % width;
        let z = i / width;

        self.generate_base(ctx, generator, pos + ChunkPos::new(x - RADIUS * 2, z - RADIUS * 2));
      });

      for x in -RADIUS..=RADIUS {
        for z in -RADIUS..=RADIUS {
          self.generate_decorated(ctx, generator, pos + ChunkPos::new(x, z));
        }
      }
    }

    {
      let mut w = self.chunks.lock();
      let chunk = w.chunk(pos).unwrap();
      f(&chunk)
    }
  }

  fn generate_decorated(&self, ctx: &Context, generator: &impl Generator, pos: ChunkPos) {
    let mut w = self.chunks.lock();
    generator.decorate(ctx, &mut w, pos);
  }

  fn generate_base(&self, ctx: &Context, generator: &impl Generator, pos: ChunkPos) {
    // Lock this chunk for building.
    {
      let mut w = self.base_chunks.lock();
      match w.get(&pos) {
        Some(_) => return,
        None => w.insert(pos, PartialChunk::Building),
      };
    }

    let mut chunk = Chunk::new();
    generator.generate_base(ctx, &mut chunk, pos);

    {
      let mut w = self.chunks.lock();
      w.chunks.insert(pos, StagedChunk { stage: Some(Stage::Base), chunk });
    }
  }
}

impl PartialWorld {
  pub fn new() -> Self { PartialWorld { chunks: HashMap::new() } }
}
