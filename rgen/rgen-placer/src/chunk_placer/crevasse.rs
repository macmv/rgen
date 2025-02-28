use rgen_base::{BlockFilter, BlockState, ChunkRelPos, Pos};
use rgen_spline::{Cosine, Spline};

use crate::{
  BiomeCachedChunk, ChunkPlacer,
  noise::{NoiseGenerator, OctavedNoise, OpenSimplexNoise},
};

pub struct Crevasse {
  pub replace:    BlockFilter,
  pub packed_ice: BlockState,

  pub height: i32,

  pub noise: OctavedNoise<OpenSimplexNoise, 3>,
}

impl Crevasse {
  pub fn new() -> Self {
    Crevasse {
      replace:    block![packed_ice].into(),
      height:     10,
      packed_ice: block![packed_ice],
      noise:      OctavedNoise::new(0, 1.0 / 16.0),
    }
  }
}

pub static DEPTH: Spline<&'static [(f64, f64)]> =
  Spline::new(&[(0.00, 1.0), (0.45, 1.0), (0.50, 0.0), (0.55, 1.0), (1.00, 1.0)]);

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
        let depth_value =
          DEPTH.sample::<Cosine>((self.noise.generate(pos.x as f64, pos.z as f64) + 1.0) / 2.0);
        let target_depth = (depth_value * self.height as f64) as i32;

        for y in (0..256).rev() {
          let rel_pos = rel_pos.with_y(y);

          let block = chunk.get(rel_pos);
          if self.replace.contains(block) {
            for i in 0..target_depth {
              chunk.set(rel_pos.with_y(y + i), self.packed_ice);
            }

            break;
          }
        }
      }
    }
  }
}
