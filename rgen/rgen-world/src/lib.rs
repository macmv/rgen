use std::{collections::HashMap, sync::Arc};

use crossbeam_channel::{Receiver, RecvError, Sender, TryRecvError};
use parking_lot::{Mutex, RwLock};
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

  requests_tx: Sender<(ChunkPos, Stage)>,
  requests_rx: Receiver<(ChunkPos, Stage)>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    let (tx, rx) = crossbeam_channel::unbounded();

    CachedWorld {
      base_chunks: Mutex::new(HashMap::new()),
      chunks:      Mutex::new(PartialWorld::new()),
      requests_tx: tx,
      requests_rx: rx,
    }
  }

  fn request(&self, pos: ChunkPos, stage: Stage) {
    match stage {
      Stage::Base => {
        if self.base_chunks.lock().contains_key(&pos) {
          return;
        }
      }
      _ => {}
    }

    self.requests_tx.send((pos, stage)).unwrap();
  }

  pub fn spawn_threads(
    self: &Arc<Self>,
    ctx: &Arc<Context>,
    generator: &Arc<impl Generator + Send + Sync + 'static>,
  ) {
    for _ in 0..32 {
      let slf = self.clone();
      let ctx = ctx.clone();
      let generator = generator.clone();

      std::thread::spawn(move || loop {
        slf.work(&ctx, generator.as_ref());
      });
    }
  }

  fn work(&self, ctx: &Context, generator: &(impl Generator + Send + Sync)) {
    match self.requests_rx.recv() {
      Ok((pos, stage)) => {
        match stage {
          Stage::Base => self.generate_base(ctx, generator, pos),
          Stage::Decorated => self.generate_decorated(ctx, generator, pos),
          _ => {}
        };
      }
      Err(_) => panic!(),
    }
  }

  pub fn generate<R>(
    &self,
    _ctx: &Context,
    _generator: &(impl Generator + Send + Sync),
    pos: ChunkPos,
    f: impl FnOnce(&Chunk) -> R,
  ) -> R {
    for x in -RADIUS * 2..=RADIUS * 2 {
      for z in -RADIUS * 2..=RADIUS * 2 {
        self.request(pos + ChunkPos::new(x, z), Stage::Base);
      }
    }

    for x in -RADIUS..=RADIUS {
      for z in -RADIUS..=RADIUS {
        self.request(pos + ChunkPos::new(x, z), Stage::Decorated);
      }
    }

    self.request(pos, Stage::NeighborDecorated);

    loop {
      std::thread::sleep(std::time::Duration::from_millis(10));
      let w = self.chunks.lock();
      match w.chunks.get(&pos) {
        Some(chunk) if chunk.stage == Some(Stage::Decorated) => break,
        _ => continue,
      }
    }

    {
      let mut w = self.chunks.lock();
      let chunk = w.chunk(pos).unwrap();
      f(&chunk)
    }
  }

  fn generate_decorated(&self, ctx: &Context, generator: &impl Generator, pos: ChunkPos) {
    let mut chunks = self.chunks.lock();
    let mut valid = true;
    for x in -RADIUS..=RADIUS {
      for z in -RADIUS..=RADIUS {
        if !chunks.chunks.contains_key(&(pos + ChunkPos::new(x, z))) {
          self.request(pos + ChunkPos::new(x, z), Stage::Base);
          valid = false;
        }
      }
    }
    if !valid {
      self.request(pos, Stage::Decorated);
      return;
    }

    let stage = chunks.chunks.get(&pos).map(|c| c.stage).unwrap().unwrap();
    if stage >= Stage::Decorated {
      return;
    }
    chunks.chunks.get_mut(&pos).unwrap().stage = Some(Stage::Decorated);
    generator.decorate(ctx, &mut chunks, pos);
  }

  fn generate_base(&self, ctx: &Context, generator: &impl Generator, pos: ChunkPos) {
    // Lock this chunk for building. Until this point, everything has been
    // optimistic. This is the first real lock. If this does not return, this thread
    // is the sole generator of this chunk.
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
