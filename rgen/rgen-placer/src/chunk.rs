// This is a new chunk type, which stores cached biome information. This
// information is thrown away as soon as the chunk is generated. The IDs stored
// in this chunk are temporary, and unique to each chunk. So biome id 0 in one
// chunk is not the same as biome id 0 in another chunk.
//
// Each biome cached chunk can only store one of 16 different biomes per block,
// as there will not be more than 16 biomes in one chunk. This means that only 4
// bits of extra information are needed for each block.

use rgen_base::{BlockInfo, BlockState, Chunk, ChunkRelPos};
use rgen_world::BlockInfoSupplier;

pub struct BiomeCachedChunk<'a> {
  info:      &'a BlockInfoSupplier,
  pub chunk: &'a mut Chunk,

  // The "active" biome. This chunk will be passed to various chunk placers, which will check if a
  // given position is "active". This is the active ID that gets checked against that block.
  active: TemporaryBiome,
  biomes: Box<[[BiomeColumn; 16]; 16]>,
}

#[derive(Clone, Copy)]
pub struct BiomeColumn {
  pub surface: TemporaryBiome,
  pub cave:    TemporaryBiome,

  pub min_height: i32,
  // max_height: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TemporaryBiome(pub u8);

impl TemporaryBiome {
  pub fn incr(&mut self) { self.0 += 1; }
}

impl BiomeColumn {
  pub const ZERO: Self = BiomeColumn {
    surface:    TemporaryBiome(0),
    cave:       TemporaryBiome(0),
    min_height: 0,
    // max_height: 0,
  };
}

impl<'a> BiomeCachedChunk<'a> {
  pub fn new(supplier: &'a BlockInfoSupplier, chunk: &'a mut Chunk) -> Self {
    BiomeCachedChunk {
      info: supplier,
      chunk,
      active: TemporaryBiome(0),
      biomes: Box::new([[BiomeColumn::ZERO; 16]; 16]),
    }
  }

  /// Sets the active biome ID. Note that placers should not call this!
  pub fn set_active(&mut self, active: TemporaryBiome) { self.active = active; }

  pub fn is_active(&self, pos: ChunkRelPos) -> bool {
    let column = self.biomes[pos.z() as usize][pos.x() as usize];
    if pos.y() < column.min_height {
      self.active == column.cave
    } else {
      self.active == column.surface
    }
  }

  /// Sets the biome at the given position.
  ///
  /// This will not overwrite the previous biome, so it should not be called by
  /// any placers.
  pub fn set_column(&mut self, pos: ChunkRelPos, column: BiomeColumn) {
    self.biomes[pos.z() as usize][pos.x() as usize] = column;
  }
}

// Block impls
impl<'a> BiomeCachedChunk<'a> {
  pub fn get(&self, pos: ChunkRelPos) -> BlockInfo { self.info.decode(self.chunk.get(pos)) }
  pub fn set(&mut self, pos: ChunkRelPos, state: impl Into<BlockState>) {
    self.chunk.set(pos, self.info.encode(state.into()))
  }
}
