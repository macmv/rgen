use rgen_base::{BlockFilter, BlockState, Blocks, ChunkRelPos, Pos};
use rgen_spline::{Cosine, Spline};

use crate::{
  noise::{NoiseGenerator, OctavedNoise, OpenSimplexNoise},
  BiomeCachedChunk, ChunkPlacer,
};

pub struct Crevasse {
  pub replace: BlockFilter,

  pub depth: i32,

  pub ice: BlockState,

  pub noise: OctavedNoise<OpenSimplexNoise, 4>,
}

impl Crevasse {
  pub fn new(blocks: &Blocks) -> Self {
    Crevasse {
      replace: blocks.stone.block.into(),
      depth:   10,
      ice:     blocks.ice.default_state,
      noise:   OctavedNoise::new(0, 1.0 / 32.0),
    }
  }
}

pub static DEPTH: Spline<&'static [(f64, f64)]> =
  Spline::new(&[(-1.0, 0.0), (-0.1, 0.0), (0.0, 1.0), (0.1, 0.0), (1.0, 0.0)]);

impl ChunkPlacer for Crevasse {
  fn place(
    &self,
    chunk: &mut BiomeCachedChunk,
    _rng: &mut crate::Rng,
    chunk_pos: rgen_base::ChunkPos,
  ) {
    for x in 0..16 {
      for z in 0..16 {
        let rel_pos = ChunkRelPos::new(x, 255, z);
        if !chunk.is_active(rel_pos) {
          continue;
        }

        let pos = Pos::new(x as i32, 255, z as i32) + chunk_pos.min_block_pos();
        let depth_value = DEPTH.sample::<Cosine>(self.noise.generate(pos.x as f64, pos.z as f64));
        let target_depth = (depth_value * self.depth as f64) as i32;

        let mut depth = 0;
        for y in (0..256).rev() {
          let rel_pos = rel_pos.with_y(y);

          let block = chunk.chunk.get_state(rel_pos);
          if depth != 0 || self.replace.contains(block) {
            depth += 1;
          }

          if depth < target_depth {
            chunk.chunk.set(rel_pos, BlockState::AIR);
          }
          if depth >= target_depth {
            break;
          }
        }
      }
    }
  }
}
