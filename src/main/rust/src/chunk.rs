use crate::{gen::Block, pos::ChunkRelPos};

// Mirrors a ChunkPrimer in minecraft.
pub struct Chunk {
  data: Box<[u16]>,
}

fn pos_to_index(pos: ChunkRelPos) -> usize {
  ((pos.z() as usize) << 12) | ((pos.x() as usize) << 8) | (pos.y() as usize)
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

  pub fn set(&mut self, pos: ChunkRelPos, block: Block) { self.set_data(pos, block, 0); }
  pub fn set_data(&mut self, pos: ChunkRelPos, block: Block, block_data: u8) {
    assert!(block_data < 16);
    self.data[pos_to_index(pos)] = block.raw_id() | (block_data as u16);
  }

  pub fn data(&self) -> &[u16] { &self.data }
}
