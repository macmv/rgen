use crate::{Block, BlockState, ChunkRelPos};

// Mirrors a ChunkPrimer in minecraft.
#[derive(Clone)]
#[allow(clippy::new_without_default)]
pub struct Chunk {
  data: Box<[u16]>,
}

fn pos_in_world(pos: ChunkRelPos) -> bool { pos.y() >= 0 && pos.y() < 256 }

fn pos_to_index(pos: ChunkRelPos) -> usize {
  ((pos.x() as usize) << 12) | ((pos.z() as usize) << 8) | (pos.y() as usize)
}

impl Chunk {
  pub fn new() -> Chunk {
    // vec![0; 65536].into_boxed_slice() does the same thing, but it builds the
    // whole thing on the stack first in debug mode.
    unsafe {
      use std::alloc::{alloc_zeroed, Layout};

      let layout = Layout::array::<u16>(65536).unwrap();
      // Pointer casts from a *mut u8 to a *mut u16. This is safe because the layout
      // is u16 aligned.
      let ptr = alloc_zeroed(layout).cast();
      // Builds a raw slice from the given pointer, which was just allocated and
      // aligned correctly.
      let slice_ptr = core::ptr::slice_from_raw_parts_mut(ptr, 65536);
      Chunk { data: Box::from_raw(slice_ptr) }
    }
  }

  pub fn set(&mut self, pos: ChunkRelPos, block: impl Into<BlockState>) {
    if pos_in_world(pos) {
      self.data[pos_to_index(pos)] = block.into().raw_id();
    }
  }

  pub fn get(&self, pos: ChunkRelPos) -> Block { self.get_state(pos).block }
  pub fn get_state(&self, pos: ChunkRelPos) -> BlockState {
    if pos_in_world(pos) {
      BlockState::from_raw_id(self.data[pos_to_index(pos)])
    } else {
      BlockState::AIR
    }
  }

  pub fn data(&self) -> &[u16] { &self.data }
}
