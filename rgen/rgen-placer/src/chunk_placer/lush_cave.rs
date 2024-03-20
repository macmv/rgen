use rgen_base::{BlockState, Blocks, Pos};

use crate::{ChunkPlacer, Random};

pub struct LushCaveMoss {
  pub moss: BlockState,
}

impl LushCaveMoss {
  pub fn new(blocks: &Blocks) -> Self {
    LushCaveMoss { moss: blocks.rgen_mossy_carpet.default_state }
  }
}

impl ChunkPlacer for LushCaveMoss {
  fn place(
    &self,
    chunk: &mut rgen_base::Chunk,
    rng: &mut crate::Rng,
    chunk_pos: rgen_base::ChunkPos,
  ) {
    for x in 0..16 {
      for z in 0..16 {
        for y in (0..256).rev() {
          let pos = chunk_pos.min_block_pos() + Pos::new(x, y, z);

          let below = chunk.get((pos - Pos::new(0, 1, 0)).chunk_rel());
          let block = chunk.get(pos.chunk_rel());
          if below != rgen_base::Block::AIR && block == rgen_base::Block::AIR {
            if rng.rand_exclusive(0, 10) == 0 {
              chunk.set_state(pos.chunk_rel(), self.moss);
            }
          }
        }
      }
    }
  }
}
