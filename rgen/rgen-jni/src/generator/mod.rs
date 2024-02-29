use crate::ChunkContext;
use rgen_base::{Blocks, Chunk, ChunkPos, ChunkRelPos, Pos};
use rgen_placer::noise::{NoiseGenerator, OctavedNoise, PerlinNoise};
use rgen_world::{Context, Generator, PartialWorld};

pub struct TerrainGenerator {
  seed: u64,

  height_map: OctavedNoise<PerlinNoise>,

  biomes: rgen_biome::Biomes,
}

impl Generator for TerrainGenerator {
  fn height_at(&self, _: Pos) -> f64 { 0.0 }

  fn generate_biomes(&self, chunk_pos: ChunkPos, biomes: &mut [u8; 256]) {
    self.biomes.generate_ids(self.seed, chunk_pos, biomes);
  }

  fn generate_base(&self, ctx: &Context, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    for rel_x in 0..16_u8 {
      for rel_z in 0..16_u8 {
        let pos = chunk_pos.min_block_pos() + Pos::new(rel_x.into(), 0, rel_z.into());

        let height =
          ((self.height_map.generate(pos.x as f64, pos.z as f64, self.seed) + 1.0) * 64.0) as i32;

        for y in 0..height as u8 {
          chunk.set(ChunkRelPos::new(rel_x, y, rel_z), ctx.blocks.stone);
        }
      }
    }

    self.biomes.generate_top_layer(&ctx.blocks, self.seed, chunk, chunk_pos);
  }

  fn decorate(&self, ctx: &Context, world: &mut PartialWorld, chunk_pos: ChunkPos) {
    self.biomes.decorate(&ctx.blocks, self.seed, world, chunk_pos);

    world.set(chunk_pos.min_block_pos() + Pos::new(0, 6, 0), ctx.blocks.dirt);
  }
}

impl TerrainGenerator {
  pub fn new(blocks: &Blocks, biome_ids: &rgen_base::Biomes, seed: u64) -> TerrainGenerator {
    TerrainGenerator {
      seed,
      height_map: OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },

      biomes: rgen_biome::Biomes::new(blocks, biome_ids),
    }
  }

  pub fn generate(&self, ctx: &ChunkContext, chunk: &mut Chunk) {
    for rel_x in 0..16_u8 {
      for rel_z in 0..16_u8 {
        let pos = ctx.chunk_pos.min_block_pos() + Pos::new(rel_x.into(), 0, rel_z.into());

        let height =
          ((self.height_map.generate(pos.x as f64, pos.z as f64, self.seed) + 1.0) * 64.0) as i32;

        for y in 0..height as u8 {
          chunk.set(ChunkRelPos::new(rel_x, y, rel_z), ctx.blocks.stone);
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use rgen_base::{Biomes, Blocks, ChunkPos};

  use super::*;

  #[test]
  fn test_generator() {
    let mut chunk = Chunk::new();
    let blocks = Blocks::test_blocks();
    let biomes = Biomes::test_blocks();
    let generator = TerrainGenerator::new(&blocks, &biomes, 1);

    let ctx = ChunkContext { chunk_pos: ChunkPos::new(0, 0), blocks: &blocks };

    generator.generate(&ctx, &mut chunk);
  }
}
