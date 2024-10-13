use core::fmt;
use std::{collections::HashMap, sync::Arc};

use crossbeam_channel::{Receiver, Sender};
use parking_lot::{Mutex, RwLock};
use rgen_base::{Chunk, ChunkPos, Pos, StateId};

mod block;
mod gc;
mod info;

pub use info::{BlockInfoCache, BlockInfoSupplier};

pub struct Context {
  pub seed:   u64,
  pub blocks: BlockInfoCache<Box<dyn BlockInfoSupplier + Send + Sync>>,
  // pub biomes: Biomes,
}

impl Context {
  /*
  pub fn new_test(seed: u64) -> Self {
    Context { seed, blocks: Blocks::test_blocks(), biomes: Biomes::test_blocks() }
  }
  */
  pub fn new_test(_seed: u64) -> Self { todo!() }
}

pub trait Generator {
  fn generate_base(&self, ctx: &Context, chunk: &mut Chunk, pos: ChunkPos);
  fn decorate<T>(&self, ctx: &Context, world: &mut PartialWorld<T>, pos: ChunkPos);
}

pub struct CachedWorld {
  base_chunks: Mutex<HashMap<ChunkPos, PartialChunk>>,

  // FIXME: Need to clean up this map once it gets full. The cleanup needs to be somewhat
  // intelligent, so this is kinda tricky.
  chunks: Mutex<StagedWorldStorage>,

  requester: Requester,
}

pub struct PartialWorld<'a, T> {
  info:    T,
  storage: Box<dyn PartialWorldStorage + 'a>,
}

pub trait PartialWorldStorage {
  fn get(&self, pos: Pos) -> StateId;
  fn set(&mut self, pos: Pos, block: StateId);
}

impl<'a, T> PartialWorld<'a, T> {
  pub fn new(info: T, storage: impl PartialWorldStorage + 'a) -> Self {
    PartialWorld { info, storage: Box::new(storage) }
  }
}

pub struct StagedWorldStorage {
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
  stage: Stage,
  chunk: Chunk,
}

/// The maximum radius of a single decoration, in chunks.
const RADIUS: i32 = 1;

struct Requester {
  tx: Sender<(ChunkPos, Stage)>,
  rx: Receiver<(ChunkPos, Stage)>,

  /// These chunks are the state after the entire `rx` has been processed.
  ///
  /// Any new `request` calls will look at this list, and be skipped if there is
  /// already a request for that chunk.
  chunks: RwLock<HashMap<ChunkPos, Mutex<Stage>>>,
}

impl CachedWorld {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    CachedWorld {
      base_chunks: Mutex::new(HashMap::new()),
      chunks:      Mutex::new(StagedWorldStorage::new()),
      requester:   Requester::new(),
    }
  }

  fn request(&self, pos: ChunkPos, stage: Stage) { self.requester.request(pos, stage); }

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

    // spawn up a GC thread to run every 10 seconds.
    let slf = self.clone();
    std::thread::spawn(move || loop {
      std::thread::sleep(std::time::Duration::from_secs(10));
      slf.gc();
    });
  }

  fn work(&self, ctx: &Context, generator: &(impl Generator + Send + Sync)) {
    let (pos, stage) = self.requester.recv();
    match stage {
      Stage::Base => self.generate_base(ctx, generator, pos),
      Stage::Decorated => self.generate_decorated(ctx, generator, pos),
      Stage::NeighborDecorated => self.generate_neighbor_decorated(ctx, generator, pos),
    };
  }

  pub fn generate<R>(&self, pos: ChunkPos, f: impl FnOnce(&Chunk) -> R) -> R {
    // The minimum radius required to generate a neighbor decorated chunk is `RADIUS
    // + 2`. However, this leads to very low parallelism when generating a region of
    // chunks next to each other. Increasing this by 1 leads to much better real
    // world performance (~15x), where chunks are generated next to each other
    // often. Increasing this any more only has negligible speed improvements.
    for x in -RADIUS * 3..=RADIUS * 3 {
      for z in -RADIUS * 3..=RADIUS * 3 {
        self.request(pos + ChunkPos::new(x, z), Stage::Base);
      }
    }

    for x in -RADIUS..=RADIUS {
      for z in -RADIUS..=RADIUS {
        self.request(pos + ChunkPos::new(x, z), Stage::Decorated);
      }
    }

    let mut i = 0;
    loop {
      // If the GC runs while we're waiting, the chunk might not get generated. This
      // is here to make sure it always gets generated.
      if i % 10 == 0 {
        self.request(pos, Stage::NeighborDecorated);
      }
      i += 1;

      std::thread::sleep(std::time::Duration::from_micros(100));

      let w = self.chunks.lock();
      match w.chunks.get(&pos) {
        Some(chunk) if chunk.stage == Stage::NeighborDecorated => break f(&chunk.chunk),
        _ => continue,
      }
    }
  }

  fn generate_neighbor_decorated(&self, ctx: &Context, generator: &impl Generator, pos: ChunkPos) {
    let mut chunks = self.chunks.lock();
    if chunks.chunks.get(&pos).map(|c| c.stage < Stage::Decorated).unwrap_or(true) {
      drop(chunks);
      self.generate_decorated(ctx, generator, pos);
      self.requester.retry(pos, Stage::NeighborDecorated);
      return;
    }

    let mut valid = true;
    for x in -RADIUS..=RADIUS {
      for z in -RADIUS..=RADIUS {
        let pos = pos + ChunkPos::new(x, z);
        if chunks.chunks.get(&pos).map(|c| c.stage < Stage::Decorated).unwrap_or(true) {
          self.request(pos, Stage::Decorated);
          valid = false;
        }
      }
    }
    if !valid {
      self.requester.retry(pos, Stage::NeighborDecorated);
      return;
    }

    match chunks.chunks.get(&pos).unwrap().stage {
      Stage::Decorated => {
        chunks.chunks.get_mut(&pos).unwrap().stage = Stage::NeighborDecorated;
      }
      Stage::NeighborDecorated => (),
      Stage::Base => unreachable!(),
    }
  }

  fn generate_decorated(&self, ctx: &Context, generator: &impl Generator, pos: ChunkPos) {
    let mut chunks = self.chunks.lock();
    if !chunks.chunks.contains_key(&pos) {
      drop(chunks);
      self.generate_base(ctx, generator, pos);
      self.requester.retry(pos, Stage::Decorated);
      return;
    }

    let mut valid = true;
    for x in -RADIUS..=RADIUS {
      for z in -RADIUS..=RADIUS {
        let pos = pos + ChunkPos::new(x, z);
        if !chunks.chunks.contains_key(&pos) {
          self.request(pos, Stage::Base);
          valid = false;
        }
      }
    }
    if !valid {
      self.requester.retry(pos, Stage::Decorated);
      return;
    }

    match chunks.chunks.get(&pos).unwrap().stage {
      Stage::Decorated | Stage::NeighborDecorated => (),
      Stage::Base => {
        chunks.chunks.get_mut(&pos).unwrap().stage = Stage::Decorated;
        generator.decorate(
          ctx,
          &mut PartialWorld { info: &ctx.blocks, storage: Box::new(&mut *chunks) },
          pos,
        );
      }
    }
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
      w.chunks.insert(pos, StagedChunk { stage: Stage::Base, chunk });
    }
  }
}

impl StagedWorldStorage {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self { StagedWorldStorage { chunks: HashMap::new() } }
}

impl Requester {
  pub fn new() -> Self {
    let (tx, rx) = crossbeam_channel::unbounded();
    Requester { tx, rx, chunks: RwLock::new(HashMap::new()) }
  }

  // TODO: Might need a bit more thinking.
  pub fn retry(&self, pos: ChunkPos, stage: Stage) {
    {
      let mut w = self.chunks.write();
      w.insert(pos, Mutex::new(stage));
    }

    self.tx.send((pos, stage)).unwrap();
  }

  pub fn request(&self, pos: ChunkPos, stage: Stage) {
    // Quick sanity check.
    match self.chunks.read().get(&pos) {
      Some(s) if *s.lock() >= stage => return,
      _ => {}
    }

    // Real check.
    {
      let mut w = self.chunks.write();
      if let Some(s) = w.get_mut(&pos) {
        if *s.get_mut() >= stage {
          return;
        }
      }
      w.insert(pos, Mutex::new(stage));
    }

    self.tx.send((pos, stage)).unwrap();
  }

  pub fn recv(&self) -> (ChunkPos, Stage) {
    match self.rx.recv() {
      Ok((pos, stage)) => (pos, stage),
      Err(_) => panic!("channel disconnected"),
    }
  }
}

impl fmt::Display for StagedWorldStorage {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "PartialWorld {{")?;

    let min_x = self.chunks.keys().map(|p| p.x).min().unwrap_or(0);
    let max_x = self.chunks.keys().map(|p| p.x).max().unwrap_or(0);
    let min_z = self.chunks.keys().map(|p| p.z).min().unwrap_or(0);
    let max_z = self.chunks.keys().map(|p| p.z).max().unwrap_or(0);

    for z in min_z..=max_z {
      write!(f, "\n  ")?;
      for x in min_x..=max_x {
        let pos = ChunkPos::new(x, z);
        let stage = match self.chunks.get(&pos).map(|c| c.stage) {
          Some(Stage::Base) => "B",
          Some(Stage::Decorated) => "D",
          Some(Stage::NeighborDecorated) => "N",
          None => " ",
        };
        write!(f, "{} ", stage)?;
      }
    }
    write!(f, "\n}}")
  }
}
