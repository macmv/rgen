use std::collections::HashMap;

use crossbeam_channel::{Receiver, Sender};
use parking_lot::{RwLock, RwLockReadGuard};
use rgen_base::{Biome, ChunkPos, ChunkRelPos, Pos};
use rgen_world::{Context, Generator};

use crate::terrain::TerrainGenerator;

pub struct World<G> {
  pub context:   Context,
  pub generator: G,

  chunks: RwLock<HashMap<ChunkPos, BiomeChunk>>,

  pub completed_tx: Sender<(ChunkPos, BiomeChunk)>,
  pub completed_rx: Receiver<(ChunkPos, BiomeChunk)>,
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
    let (ctx, crx) = crossbeam_channel::bounded(64);

    World {
      context,
      generator,
      chunks: RwLock::new(HashMap::new()),

      completed_tx: ctx,
      completed_rx: crx,
    }
  }

  pub fn recv_chunks(&self) {
    let mut w = self.chunks.write();
    for (pos, chunk) in self.completed_rx.try_iter() {
      w.insert(pos, chunk);
    }
  }

  pub fn read(&self) -> WorldReadLock { WorldReadLock { chunks: self.chunks.read() } }
}

pub struct WorldReadLock<'a> {
  chunks: RwLockReadGuard<'a, HashMap<ChunkPos, BiomeChunk>>,
}

impl WorldReadLock<'_> {
  pub fn has_chunk(&self, chunk_pos: ChunkPos) -> bool { self.chunks.contains_key(&chunk_pos) }

  #[track_caller]
  pub fn column_at(&self, pos: Pos) -> Column {
    let chunk_pos = pos.chunk();
    self.chunks.get(&chunk_pos).map(|c| c.column_at(pos.chunk_rel())).unwrap_or_default()
  }

  #[track_caller]
  pub fn height_at(&self, pos: Pos) -> f64 { self.column_at(pos).height }
}

impl World<TerrainGenerator> {
  pub fn build_chunk(&self, chunk_pos: ChunkPos) {
    let mut columns = [Column::EMPTY; 256];

    for rel_x in 0..16 {
      for rel_z in 0..16 {
        let pos = chunk_pos.min_block_pos() + Pos::new(rel_x, 0, rel_z);
        let biome = self.generator.biomes.choose_biome(self.generator.seed, pos).id;
        let i = (rel_x * 16 + rel_z) as usize;

        let height = self.generator.height_at(pos);

        columns[i] = Column { height, biome };
      }
    }

    self.completed_tx.send((chunk_pos, BiomeChunk { columns })).unwrap();
  }
}

impl BiomeChunk {
  pub fn column_at(&self, rel_pos: ChunkRelPos) -> Column {
    let i = (rel_pos.x() * 16 + rel_pos.z()) as usize;
    self.columns[i]
  }
}
