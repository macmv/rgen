#![feature(test)]

extern crate test;

use std::sync::Arc;

use rgen_base::{Biomes, Blocks, Chunk, ChunkPos, Pos};
use rgen_world::{CachedWorld, Context, Generator, PartialWorld};
use test::Bencher;

#[bench]
fn bench_chunk(b: &mut Bencher) {
  let world = Arc::new(CachedWorld::new());
  let context = Arc::new(rgen_world::Context {
    seed:   1 as u64,
    blocks: Blocks::test_blocks(),
    biomes: Biomes::test_blocks(),
  });
  let generator =
    Arc::new(TerrainGenerator::new(&context.blocks, &context.biomes, context.seed as u64));
  world.spawn_threads(&context, &generator);

  let mut chunk_pos = ChunkPos::new(0, 0);

  b.iter(|| {
    chunk_pos.x += 1;
    if chunk_pos.x > 20 {
      chunk_pos.x = 0;
      chunk_pos.z += 1;
    }

    world.generate(&context, generator.as_ref(), chunk_pos, |_| {});
  });
}

pub struct TerrainGenerator {
  pub seed: u64,

  pub biomes: rgen_biome::WorldBiomes,
}

impl Generator for TerrainGenerator {
  fn height_at(&self, _: Pos) -> f64 { 0.0 }

  fn generate_biomes(&self, chunk_pos: ChunkPos, biomes: &mut [u8; 256]) {
    self.biomes.generate_ids(self.seed, chunk_pos, biomes);
  }

  fn generate_base(&self, ctx: &Context, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    self.biomes.generate_base(self.seed, ctx, chunk, chunk_pos);
  }

  fn decorate(&self, ctx: &Context, world: &mut PartialWorld, chunk_pos: ChunkPos) {
    self.biomes.decorate(&ctx.blocks, self.seed, world, chunk_pos);

    world.set(chunk_pos.min_block_pos() + Pos::new(0, 6, 0), ctx.blocks.dirt.block);
  }
}

impl TerrainGenerator {
  pub fn new(blocks: &Blocks, biome_ids: &rgen_base::Biomes, seed: u64) -> TerrainGenerator {
    TerrainGenerator { seed, biomes: rgen_biome::WorldBiomes::new(blocks, biome_ids) }
  }
}
