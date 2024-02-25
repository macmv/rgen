use crate::pos::ChunkRelPos;

// Mirrors a ChunkPrimer in minecraft.
pub struct Chunk {
  data: Box<[u16]>,
}

fn pos_to_index(pos: ChunkRelPos) -> usize {
  ((pos.z() as usize) << 12) | ((pos.x() as usize) << 8) | (pos.y() as usize)
}

impl Chunk {
  pub fn new() -> Chunk { Chunk { data: vec![0; 65536].into_boxed_slice() } }

  pub fn set(&mut self, pos: ChunkRelPos, block_id: u16, block_data: u8) {
    assert!(block_id < 4096);
    assert!(block_data < 16);
    self.data[pos_to_index(pos)] = block_id << 4 | (block_data as u16);
  }

  pub fn data(&self) -> &[u16] { &self.data }
}
