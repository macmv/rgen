use std::{collections::HashMap, sync::Arc};

use parking_lot::Mutex;

use crate::{region::RegionPos, render::RenderBuffer, view::WorldViewer, world::World};

/// This struct stores the rendering state as of the last frame. This is updated
/// quickly by the main thread at the end of each frame. During a frame, this
/// information is used by the generation and rendering threads to generate the
/// correct chunks needed.
pub struct RenderQueue {
  state: Mutex<RenderState>,

  rendering: Mutex<Vec<RegionPos>>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct RenderState {
  pub min_chunk: RegionPos,
  pub max_chunk: RegionPos,
  pub center:    RegionPos,
  pub radius:    i32,
}

impl RenderQueue {
  /// Calls `updater`. If `updater` returns true, or mutates the given state,
  /// the render queue is regenerated.
  pub fn update(
    &self,
    rendered_chunks: &HashMap<RegionPos, RenderBuffer>,
    updater: impl FnOnce(&mut RenderState) -> bool,
  ) {
    let mut state = self.state.lock();
    let old_state = state.clone();

    if updater(&mut state) || *state != old_state {
      self.regenerate_queue(&state, rendered_chunks);
    }
  }

  fn regenerate_queue(
    &self,
    state: &RenderState,
    rendered_chunks: &HashMap<RegionPos, RenderBuffer>,
  ) {
    let mut rendering = self.rendering.lock();

    rendering.clear();

    for i in 0..state.radius {
      let min_circle = state.center - RegionPos::new(i, i);
      let max_circle = state.center + RegionPos::new(i, i);

      let x_iter = (min_circle.x..=max_circle.x)
        .flat_map(|x| [min_circle.z, max_circle.z].into_iter().map(move |z| RegionPos::new(x, z)));
      let z_iter = (min_circle.z + 1..max_circle.z)
        .flat_map(|z| [min_circle.x, max_circle.x].into_iter().map(move |x| RegionPos::new(x, z)));

      for region_pos in x_iter.chain(z_iter) {
        if region_pos.x < state.min_chunk.x
          || region_pos.x > state.max_chunk.x
          || region_pos.z < state.min_chunk.z
          || region_pos.z > state.max_chunk.z
        {
          continue;
        }

        if !rendered_chunks.contains_key(&region_pos) {
          rendering.push(region_pos);
        }
      }
    }

    rendering.reverse();
  }

  /// Pulls off the next chunk to be rendered.
  pub fn pop_render(&self) -> Option<RegionPos> { self.rendering.lock().pop() }
}

const POOL_SIZE: usize = 32;

impl RenderQueue {
  pub fn new() -> RenderQueue {
    let state = RenderState {
      min_chunk: RegionPos::new(0, 0),
      max_chunk: RegionPos::new(0, 0),
      center:    RegionPos::new(0, 0),
      radius:    0,
    };

    RenderQueue { state: Mutex::new(state), rendering: Mutex::new(vec![]) }
  }

  pub fn spawn_render_threads(self: &Arc<Self>, world: &Arc<World>, view: &Arc<WorldViewer>) {
    for _ in 0..POOL_SIZE {
      let slf = self.clone();
      let world = world.clone();
      let view = view.clone();

      std::thread::spawn(move || loop {
        if let Some(region_pos) = slf.pop_render() {
          view.render_chunk(&world, region_pos);
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
