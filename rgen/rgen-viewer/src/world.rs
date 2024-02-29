use std::collections::HashMap;

use rgen_base::{Biome, ChunkPos, ChunkRelPos, Pos};
use rgen_world::{Context, Generator};

pub struct World<G> {
  pub context:   Context,
  pub generator: G,

  chunks: HashMap<ChunkPos, BiomeChunk>,
}

pub struct BiomeChunk {
  columns: [Column; 256],
}

#[derive(Clone, Copy)]
pub struct Column {
  /// The height of this column, in blocks.
  pub height: f64,

  /// The biome at this column.
  pub biome: Biome,
}

impl Column {
  const EMPTY: Column = Column { height: 0.0, biome: Biome::VOID };
}

impl Default for Column {
  fn default() -> Column { Column::EMPTY }
}

impl<G> World<G> {
  pub fn new(context: Context, generator: G) -> World<G> {
    World { context, generator, chunks: HashMap::new() }
  }

  pub fn has_chunk(&self, chunk_pos: ChunkPos) -> bool { self.chunks.contains_key(&chunk_pos) }

  #[track_caller]
  pub fn column_at(&self, pos: Pos) -> Column {
    let chunk_pos = pos.chunk();
    self.chunks.get(&chunk_pos).map(|c| c.column_at(pos.chunk_rel())).unwrap_or_default()
  }

  #[track_caller]
  pub fn height_at(&self, pos: Pos) -> f64 { self.column_at(pos).height }
}

impl<G: Generator> World<G> {
  pub fn generate_chunk(&mut self, chunk_pos: ChunkPos) {
    if !self.chunks.contains_key(&chunk_pos) {
      let mut columns = [Column::EMPTY; 256];

      let mut biomes = [0; 256];
      self.generator.generate_biomes(chunk_pos, &mut biomes);

      for rel_x in 0..16 {
        for rel_z in 0..16 {
          let pos = chunk_pos.min_block_pos() + Pos::new(rel_x, 0, rel_z);
          let i = (rel_x * 16 + rel_z) as usize;

          let biome_id = biomes[i];
          let biome = Biome::from_raw_id(biome_id.into());

          let height = self.generator.height_at(pos);

          columns[i] = Column { height, biome };
        }
      }

      self.chunks.insert(chunk_pos, BiomeChunk { columns });
    }
  }
}

impl BiomeChunk {
  pub fn column_at(&self, rel_pos: ChunkRelPos) -> Column {
    let i = (rel_pos.x() * 16 + rel_pos.z()) as usize;
    self.columns[i]
  }
}
