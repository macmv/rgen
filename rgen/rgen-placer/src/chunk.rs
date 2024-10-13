// This is a new chunk type, which stores cached biome information. This
// information is thrown away as soon as the chunk is generated. The IDs stored
// in this chunk are temporary, and unique to each chunk. So biome id 0 in one
// chunk is not the same as biome id 0 in another chunk.
//
// Each biome cached chunk can only store one of 16 different biomes per block,
// as there will not be more than 16 biomes in one chunk. This means that only 4
// bits of extra information are needed for each block.

use rgen_base::{BlockState, Chunk, ChunkRelPos};
use rgen_world::BlockInfoSupplier;

pub struct BiomeCachedChunk<'a> {
  info:      &'a dyn BlockInfoSupplier,
  pub chunk: &'a mut Chunk,

  // The "active" biome. This chunk will be passed to various chunk placers, which will check if a
  // given position is "active". This is the active ID that gets checked against that block.
  active: TemporaryBiome,
  biomes: Box<[[[u8; 8]; 16]; 256]>,
}

#[derive(Clone, Copy)]
pub struct TemporaryBiome(pub u8);

impl TemporaryBiome {
  pub fn incr(&mut self) { self.0 += 1; }
}

impl<'a> BiomeCachedChunk<'a> {
  pub fn new(supplier: &'a dyn BlockInfoSupplier, chunk: &'a mut Chunk) -> Self {
    BiomeCachedChunk {
      info: supplier,
      chunk,
      active: TemporaryBiome(0),
      biomes: Box::new([[[0; 8]; 16]; 256]),
    }
  }

  /// Sets the active biome ID. Note that placers should not call this!
  pub fn set_active(&mut self, active: TemporaryBiome) { self.active = active; }

  pub fn is_active(&self, pos: ChunkRelPos) -> bool {
    let tuple = self.biomes[pos.y() as usize][pos.z() as usize][(pos.x() / 2) as usize];
    if pos.x() % 2 == 0 {
      tuple & 0x0F == self.active.0
    } else {
      tuple >> 4 == self.active.0
    }
  }

  /// Sets the biome at the given position.
  ///
  /// This will not overwrite the previous biome, so it should not be called by
  /// any placers.
  pub fn set_biome(&mut self, pos: ChunkRelPos, biome: TemporaryBiome) {
    self.biomes[pos.y() as usize][pos.z() as usize][(pos.x() / 2) as usize] |=
      if pos.x() % 2 == 0 { biome.0 } else { biome.0 << 4 };
  }
}

// Block impls
impl<'a> BiomeCachedChunk<'a> {
  pub fn get(&self, pos: ChunkRelPos) -> BlockState { self.info.decode(self.chunk.get(pos)) }
  pub fn set(&mut self, pos: ChunkRelPos, state: impl Into<BlockState>) {
    self.chunk.set(pos, self.info.encode(state.into()))
  }
}
