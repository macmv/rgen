mod checkerboard;
mod crevasse;
mod lush_cave;
mod snow_snow;
mod snow_stone;

pub use checkerboard::CheckerboardSurface;
pub use crevasse::*;
pub use lush_cave::LushCaveMoss;
pub use snow_snow::SnowOnSnowSurface;
pub use snow_stone::SnowOnStoneSurface;

use rgen_base::{Block, BlockState, Blocks, ChunkRelPos, Pos};

use crate::{BiomeCachedChunk, ChunkPlacer, Random};

pub struct GlowVine {
  pub stone:     Block,
  pub glow_vine: Block,
}

impl GlowVine {
  pub fn new(blocks: &Blocks) -> Self {
    GlowVine { stone: blocks.stone.block, glow_vine: blocks.rgen_glow_vine.block }
  }
}

impl ChunkPlacer for GlowVine {
  fn place(
    &self,
    chunk: &mut BiomeCachedChunk,
    rng: &mut crate::Rng,
    chunk_pos: rgen_base::ChunkPos,
  ) {
    for x in 0..16 {
      for z in 0..16 {
        // Only generate these low down.
        for y in (0..40).rev() {
          let rel_pos = ChunkRelPos::new(x, y, z);
          if !chunk.is_active(rel_pos) {
            continue;
          }

          // I don't want to bother with chunk borders.
          if x == 0 || x == 15 || z == 0 || z == 15 {
            continue;
          }

          let pos = chunk_pos.min_block_pos() + Pos::new(x as i32, y, z as i32);

          let block = chunk.chunk.get(pos.chunk_rel());
          if block == rgen_base::Block::AIR && rng.rand_exclusive(0, 24) == 0 {
            let north = chunk.chunk.get((pos + Pos::new(0, 0, -1)).chunk_rel()) == self.stone;
            let south = chunk.chunk.get((pos + Pos::new(0, 0, 1)).chunk_rel()) == self.stone;
            let east = chunk.chunk.get((pos + Pos::new(-1, 0, 0)).chunk_rel()) == self.stone;
            let west = chunk.chunk.get((pos + Pos::new(1, 0, 0)).chunk_rel()) == self.stone;

            if north || south || east || west {
              chunk.chunk.set(
                pos.chunk_rel(),
                BlockState {
                  block: self.glow_vine,
                  state: south as u8 | (east as u8) << 1 | (north as u8) << 2 | (west as u8) << 3,
                },
              );
            }
          }
        }
      }
    }
  }
}
