use std::{
  collections::{HashMap, HashSet},
  sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
  },
};

use parking_lot::Mutex;
use rgen_base::ChunkPos;

use crate::{
  render::RenderBuffer,
  terrain::TerrainGenerator,
  view::WorldViewer,
  world::{World, WorldReadLock},
};

/// This struct stores the rendering state as of the last frame. This is updated
/// quickly by the main thread at the end of each frame. During a frame, this
/// information is used by the generation and rendering threads to generate the
/// correct chunks needed.
pub struct RenderQueue {
  state: Mutex<RenderState>,

  generating: Mutex<Vec<ChunkPos>>,
  rendering:  Mutex<Vec<ChunkPos>>,

  // Queue age is a bit interesting. The function to find the next rendering chunk is O(N), where N
  // is the number of chunks at the head of the queue that cannot be rendered yet, because their
  // neighbors haven't been generated.
  //
  // So, when the list of chunks to render is small, its easiest to just search through it on each
  // thread. However, once the list gets longer, it starts becoming too slow.
  //
  // This is where the queue age comes in. This number is incremented every frame, and if it gets
  // too large, the above queues are re-generated, to reset the number of chunks that need to be
  // searched every time a render thread goes to pull a new chunk.
  queue_age: AtomicU8,
}

#[derive(Clone, PartialEq, Eq)]
pub struct RenderState {
  pub min_chunk: ChunkPos,
  pub max_chunk: ChunkPos,
  pub center:    ChunkPos,
  pub radius:    i32,
}

impl RenderQueue {
  pub fn update(
    &self,
    generated_chunks: &WorldReadLock,
    rendered_chunks: &HashMap<ChunkPos, RenderBuffer>,
    updater: impl FnOnce(&mut RenderState),
  ) {
    let mut state = self.state.lock();
    let old_state = state.clone();
    updater(&mut state);

    if *state != old_state || self.queue_age.fetch_add(1, Ordering::SeqCst) > 20 {
      self.queue_age.store(0, Ordering::SeqCst);
      self.regenerate_queue(&state, generated_chunks, &rendered_chunks);
    }
  }

  fn regenerate_queue(
    &self,
    state: &RenderState,
    generated_chunks: &WorldReadLock,
    rendered_chunks: &HashMap<ChunkPos, RenderBuffer>,
  ) {
    let mut generating = self.generating.lock();
    let mut rendering = self.rendering.lock();

    generating.clear();
    rendering.clear();

    for i in 0..state.radius {
      let min_circle = state.center - ChunkPos::new(i, i);
      let max_circle = state.center + ChunkPos::new(i, i);

      for x in min_circle.x..=max_circle.x {
        for z in min_circle.z..=max_circle.z {
          let chunk_pos = ChunkPos::new(x, z);

          if chunk_pos.x < state.min_chunk.x
            || chunk_pos.x > state.max_chunk.x
            || chunk_pos.z < state.min_chunk.z
            || chunk_pos.z > state.max_chunk.z
          {
            continue;
          }

          if !generated_chunks.has_chunk(chunk_pos) {
            generating.push(chunk_pos);
          }
          if !rendered_chunks.contains_key(&chunk_pos) {
            rendering.push(chunk_pos);
          }
        }
      }
    }

    generating.reverse();
    rendering.reverse();
  }

  /// Pulls off the next chunk to be generated.
  pub fn pop_generate(&self) -> Option<ChunkPos> { self.generating.lock().pop() }

  /// Pulls off the next chunk to be rendered.
  pub fn pop_render<'a>(&self, world: WorldReadLock<'a>) -> Option<(ChunkPos, WorldReadLock<'a>)> {
    let mut rendering = self.rendering.lock();

    for i in (0..rendering.len()).into_iter().rev() {
      let chunk_pos = rendering[i];
      if world.has_chunk(chunk_pos + ChunkPos::new(1, 1))
        && world.has_chunk(chunk_pos + ChunkPos::new(1, 0))
        && world.has_chunk(chunk_pos + ChunkPos::new(1, -1))
        && world.has_chunk(chunk_pos + ChunkPos::new(0, 1))
        && world.has_chunk(chunk_pos)
        && world.has_chunk(chunk_pos + ChunkPos::new(0, -1))
        && world.has_chunk(chunk_pos + ChunkPos::new(-1, 1))
        && world.has_chunk(chunk_pos + ChunkPos::new(-1, 0))
        && world.has_chunk(chunk_pos + ChunkPos::new(-1, -1))
      {
        rendering.remove(i);
        return Some((chunk_pos, world));
      }
    }

    None
  }
}

// Spawn up 16 threads to generate chunks.
const POOL_SIZE: usize = 16;

impl RenderQueue {
  pub fn new() -> RenderQueue {
    let state = RenderState {
      min_chunk: ChunkPos::new(0, 0),
      max_chunk: ChunkPos::new(0, 0),
      center:    ChunkPos::new(0, 0),
      radius:    0,
    };

    RenderQueue {
      state: Mutex::new(state),

      generating: Mutex::new(vec![]),
      rendering:  Mutex::new(vec![]),
      queue_age:  AtomicU8::new(0),
    }
  }

  pub fn spawn_generation_threads(self: &Arc<Self>, world: &Arc<World<TerrainGenerator>>) {
    for _ in 0..POOL_SIZE {
      let slf = self.clone();
      let world = world.clone();

      std::thread::spawn(move || loop {
        if let Some(chunk_pos) = slf.pop_generate() {
          world.build_chunk(chunk_pos);
        } else {
          // If there's nothing to do, it means the screen is full. So wait around for a
          // while, as it usually means nothing is happening, so we don't want to spin a
          // bunch.
          std::thread::sleep(std::time::Duration::from_millis(100));
        }
      });
    }
  }

  pub fn spawn_render_threads(
    self: &Arc<Self>,
    world: &Arc<World<TerrainGenerator>>,
    view: &Arc<WorldViewer>,
  ) {
    for _ in 0..POOL_SIZE {
      let slf = self.clone();
      let world = world.clone();
      let view = view.clone();

      std::thread::spawn(move || loop {
        if let Some((chunk_pos, read_lock)) = slf.pop_render(world.read()) {
          view.render_chunk(&world.context, &read_lock, chunk_pos);
        } else {
          // If there's nothing to do, it means the screen is full. So wait around for a
          // while, as it usually means nothing is happening, so we don't want to spin a
          // bunch.
          std::thread::sleep(std::time::Duration::from_millis(100));
        }
      });
    }
  }
}
