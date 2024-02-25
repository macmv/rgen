use crate::{
  chunk::Chunk,
  ctx::Blocks,
  noise::{octaved::OctavedNoise, perlin::PerlinNoise, NoiseGenerator},
  pos::ChunkRelPos,
};

mod climate;

pub struct Generator {
  seed: u64,

  height_map: OctavedNoise<PerlinNoise>,
}

impl Generator {
  pub fn new(seed: u64) -> Generator {
    Generator {
      seed,
      height_map: OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },
    }
  }

  pub fn generate(&self, chunk_x: i32, chunk_z: i32, blocks: &Blocks, chunk: &mut Chunk) {
    for rel_x in 0..16_u8 {
      for rel_z in 0..16_u8 {
        let x: i32 = chunk_x * 16 + i32::from(rel_x);
        let z: i32 = chunk_z * 16 + i32::from(rel_z);

        let height =
          ((self.height_map.generate(x as f64, z as f64, self.seed) + 1.0) * 64.0) as i32;

        for y in 0..height as u8 {
          chunk.set(ChunkRelPos::new(rel_x, y, rel_z), blocks.stone);
        }
      }
    }

    chunk.set(ChunkRelPos::new(0, 6, 0), blocks.dirt);
  }
}

#[cfg(test)]
mod tests {
  use crate::ctx::Block;

  use super::*;

  fn blocks() -> Blocks { Blocks { stone: Block::from_raw_id(1), dirt: Block::from_raw_id(2) } }

  #[test]
  fn test_generator() {
    let mut chunk = Chunk::new();
    let blocks = blocks();
    let generator = Generator::new(1);

    generator.generate(0, 0, &blocks, &mut chunk);
  }
}
