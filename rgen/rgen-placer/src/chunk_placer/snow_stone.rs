use rgen_base::{BlockFilter, BlockState, ChunkRelPos};

use crate::{
  BiomeCachedChunk, ChunkPlacer, Random,
  noise::{NoiseGenerator, OpenSimplexNoise, SeededNoise},
};

pub struct SnowOnStoneSurface {
  pub a:           BlockState,
  pub place_above: BlockFilter,
  pub min_snow:    i32,
  pub add_snow:    f64,
  pub noise:       OpenSimplexNoise,
}

impl SnowOnStoneSurface {
  pub fn new(seed: u64) -> Self {
    SnowOnStoneSurface {
      noise:       OpenSimplexNoise::new(seed),
      a:           block![snow_layer],
      place_above: block![stone].into(),
      min_snow:    1,
      add_snow:    2.0,
    }
  }
}

impl ChunkPlacer for SnowOnStoneSurface {
  fn place(
    &self,
    chunk: &mut BiomeCachedChunk,
    rng: &mut crate::Rng,
    _chunk_pos: rgen_base::ChunkPos,
  ) {
    for x in 0..16 {
      for z in 0..16 {
        let pos = ChunkRelPos::new(x, 255, z);
        if !chunk.is_active(pos) {
          continue;
        }

        for y in (0..256).rev() {
          let pos = pos.with_y(y);

          let block = chunk.get(pos);
          if self.place_above.contains(block) {
            let snow_addition = ((self.noise.generate(x as f64 / 4.0, z as f64 / 4.0) * 0.5 + 0.5)
              * self.add_snow) as i32;
            let snow = self
              .a
              .with_data((rng.rand_inclusive(self.min_snow, self.min_snow + snow_addition)) as u8);
            chunk.set(pos.with_y(y + 1), snow);
            break;
          }
        }
      }
    }
  }
}
