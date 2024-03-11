use std::collections::HashMap;

use crossbeam_channel::{Receiver, Sender};
use parking_lot::{RwLock, RwLockReadGuard};
use rgen_base::{Biome, Pos};
use rgen_world::{Context, Generator};

use crate::{
  region::{RegionPos, REGION_SIZE},
  terrain::TerrainGenerator,
};

pub struct World<G> {
  pub context:   Context,
  pub generator: G,

  chunks: RwLock<HashMap<RegionPos, BiomeChunk>>,

  pub completed_tx: Sender<(RegionPos, BiomeChunk)>,
  pub completed_rx: Receiver<(RegionPos, BiomeChunk)>,
}

pub struct BiomeChunk {
  columns: [[Column; REGION_SIZE as usize]; REGION_SIZE as usize],
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
  chunks: RwLockReadGuard<'a, HashMap<RegionPos, BiomeChunk>>,
}

impl WorldReadLock<'_> {
  pub fn has_chunk(&self, region_pos: RegionPos) -> bool { self.chunks.contains_key(&region_pos) }

  #[track_caller]
  pub fn column_at(&self, pos: Pos) -> Column {
    let region_pos = RegionPos::from_pos(pos);
    self.chunks.get(&region_pos).map(|c| c.column_at(pos)).unwrap_or_default()
  }

  #[track_caller]
  pub fn height_at(&self, pos: Pos) -> f64 { self.column_at(pos).height }
}

impl World<TerrainGenerator> {
  pub fn build_chunk(&self, region_pos: RegionPos) {
    let mut columns = [[Column::EMPTY; REGION_SIZE as usize]; REGION_SIZE as usize];

    for rel_x in 0..REGION_SIZE {
      for rel_z in 0..REGION_SIZE {
        let pos = region_pos.min_block_pos() + Pos::new(rel_x, 0, rel_z);
        let biome = self.generator.biomes.choose_biome(self.generator.seed, pos).id;

        let height = self.generator.height_at(pos);

        columns[rel_x as usize][rel_z as usize] = Column { height, biome };
      }
    }

    self.completed_tx.send((region_pos, BiomeChunk { columns })).unwrap();
  }
}

impl BiomeChunk {
  pub fn column_at(&self, pos: Pos) -> Column {
    let x = (pos.x % REGION_SIZE + REGION_SIZE) % REGION_SIZE;
    let z = (pos.z % REGION_SIZE + REGION_SIZE) % REGION_SIZE;
    self.columns[x as usize][z as usize]
  }
}
