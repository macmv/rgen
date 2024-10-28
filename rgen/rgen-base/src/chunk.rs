use smallvec::SmallVec;

use crate::{ChunkRelPos, StateId};

// Mirrors a ChunkPrimer in minecraft.
#[derive(Clone)]
pub struct Chunk {
  data: Box<[u16]>,

  surfaces: Box<[[SmallVec<[u8; 2]>; 16]; 16]>,
}

fn pos_in_world(pos: ChunkRelPos) -> bool { pos.y() >= 0 && pos.y() < 256 }

fn pos_to_index(pos: ChunkRelPos) -> usize {
  ((pos.x() as usize) << 12) | ((pos.z() as usize) << 8) | (pos.y() as usize)
}

impl Chunk {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Chunk {
    // vec![0; 65536].into_boxed_slice() does the same thing, but it builds the
    // whole thing on the stack first in debug mode.
    let data = unsafe {
      use std::alloc::{alloc_zeroed, Layout};

      let layout = Layout::array::<u16>(65536).unwrap();
      // Pointer casts from a *mut u8 to a *mut u16. This is safe because the layout
      // is u16 aligned.
      let ptr = alloc_zeroed(layout).cast();
      // Builds a raw slice from the given pointer, which was just allocated and
      // aligned correctly.
      let slice_ptr = core::ptr::slice_from_raw_parts_mut(ptr, 65536);

      Box::from_raw(slice_ptr)
    };

    Chunk { data, surfaces: Box::new([const { [const { SmallVec::new_const() }; 16] }; 16]) }
  }

  pub fn set(&mut self, pos: ChunkRelPos, block: StateId) {
    if pos_in_world(pos) {
      self.data[pos_to_index(pos)] = block.0;
    }
  }

  pub fn get(&self, pos: ChunkRelPos) -> StateId {
    if pos_in_world(pos) {
      StateId(self.data[pos_to_index(pos)])
    } else {
      StateId::AIR
    }
  }

  pub fn data(&self) -> &[u16] { &self.data }

  pub fn add_surface(&mut self, pos: ChunkRelPos) {
    let surfaces = &mut self.surfaces[pos.z() as usize][pos.x() as usize];
    let y = pos.y() as u8;
    let i = surfaces.partition_point(|p| *p > y);
    surfaces.insert(i, y);
  }

  /// Returns the heights of all surfaces at the given column.
  ///
  /// A "surface" is a block that is exposed to enough air directly above for
  /// the top layer to be placed down.
  ///
  /// This list is sorted by highest to lowest, so the first element will be the
  /// highest block.
  pub fn surfaces(&self, column: ChunkRelPos) -> &[u8] {
    &self.surfaces[column.z() as usize][column.x() as usize]
  }
}
