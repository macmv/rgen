use std::{
  collections::HashMap,
  sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
  },
};

use parking_lot::Mutex;

use crate::{
  region::RegionPos,
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

  generating: Mutex<Vec<RegionPos>>,
  rendering:  Mutex<Vec<RegionPos>>,

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
  pub min_chunk: RegionPos,
  pub max_chunk: RegionPos,
  pub center:    RegionPos,
  pub radius:    i32,
}

impl RenderQueue {
  pub fn update(
    &self,
    generated_chunks: &WorldReadLock,
    rendered_chunks: &HashMap<RegionPos, RenderBuffer>,
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
    rendered_chunks: &HashMap<RegionPos, RenderBuffer>,
  ) {
    let mut generating = self.generating.lock();
    let mut rendering = self.rendering.lock();

    generating.clear();
    rendering.clear();

    for i in 0..state.radius {
      let min_circle = state.center - RegionPos::new(i, i);
      let max_circle = state.center + RegionPos::new(i, i);

      let x_iter = (min_circle.x..=max_circle.x)
        .into_iter()
        .flat_map(|x| [min_circle.z, max_circle.z].into_iter().map(move |z| RegionPos::new(x, z)));
      let z_iter = (min_circle.z + 1..max_circle.z)
        .into_iter()
        .flat_map(|z| [min_circle.x, max_circle.x].into_iter().map(move |x| RegionPos::new(x, z)));

      for region_pos in x_iter.chain(z_iter) {
        if region_pos.x < state.min_chunk.x
          || region_pos.x > state.max_chunk.x
          || region_pos.z < state.min_chunk.z
          || region_pos.z > state.max_chunk.z
        {
          continue;
        }

        if !generated_chunks.has_chunk(region_pos) {
          generating.push(region_pos);
        }
        if !rendered_chunks.contains_key(&region_pos) {
          rendering.push(region_pos);
        }
      }
    }

    generating.reverse();
    rendering.reverse();
  }

  /// Pulls off the next chunk to be generated.
  pub fn pop_generate(&self) -> Option<RegionPos> { self.generating.lock().pop() }

  /// Pulls off the next chunk to be rendered.
  pub fn pop_render<'a>(&self, world: WorldReadLock<'a>) -> Option<(RegionPos, WorldReadLock<'a>)> {
    let mut rendering = self.rendering.lock();

    for i in (0..rendering.len()).into_iter().rev() {
      let region_pos = rendering[i];
      if world.has_chunk(region_pos + RegionPos::new(1, 1))
        && world.has_chunk(region_pos + RegionPos::new(1, 0))
        && world.has_chunk(region_pos + RegionPos::new(1, -1))
        && world.has_chunk(region_pos + RegionPos::new(0, 1))
        && world.has_chunk(region_pos)
        && world.has_chunk(region_pos + RegionPos::new(0, -1))
        && world.has_chunk(region_pos + RegionPos::new(-1, 1))
        && world.has_chunk(region_pos + RegionPos::new(-1, 0))
        && world.has_chunk(region_pos + RegionPos::new(-1, -1))
      {
        rendering.remove(i);
        return Some((region_pos, world));
      }
    }

    None
  }
}

const POOL_SIZE: usize = 8;

impl RenderQueue {
  pub fn new() -> RenderQueue {
    let state = RenderState {
      min_chunk: RegionPos::new(0, 0),
      max_chunk: RegionPos::new(0, 0),
      center:    RegionPos::new(0, 0),
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
        if let Some(region_pos) = slf.pop_generate() {
          world.build_chunk(region_pos);
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
        if let Some((region_pos, read_lock)) = slf.pop_render(world.read()) {
          view.render_chunk(&world.context, &read_lock, region_pos);
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
