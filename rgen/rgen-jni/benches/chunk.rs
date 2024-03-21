#![feature(test)]

extern crate test;

use std::sync::Arc;

use rgen_base::{Biomes, Blocks, Chunk, ChunkPos};
use rgen_biome::WorldBiomes;
use rgen_world::{CachedWorld, Generator, PartialWorld};
use test::Bencher;

#[bench]
fn bench_chunk(b: &mut Bencher) {
  let world = Arc::new(CachedWorld::new());
  let context = Arc::new(rgen_world::Context {
    seed:   1 as u64,
    blocks: Blocks::test_blocks(),
    biomes: Biomes::test_blocks(),
  });
  let generator = Arc::new(WorldBiomes::new(&context.blocks, &context.biomes, context.seed as u64));
  world.spawn_threads(&context, &generator);

  let mut chunk_pos = ChunkPos::new(0, 0);

  b.iter(|| {
    chunk_pos.x += 1;
    if chunk_pos.x > 20 {
      chunk_pos.x = 0;
      chunk_pos.z += 1;
    }

    world.generate(chunk_pos, |_| {});
  });
}

#[bench]
fn bench_base(b: &mut Bencher) {
  let context = Arc::new(rgen_world::Context {
    seed:   1 as u64,
    blocks: Blocks::test_blocks(),
    biomes: Biomes::test_blocks(),
  });
  let generator = Arc::new(WorldBiomes::new(&context.blocks, &context.biomes, context.seed as u64));

  let mut chunk_pos = ChunkPos::new(0, 0);

  b.iter(|| {
    chunk_pos.x += 1;
    if chunk_pos.x > 20 {
      chunk_pos.x = 0;
      chunk_pos.z += 1;
    }

    let mut chunk = Chunk::new();
    generator.generate_base(&context, &mut chunk, chunk_pos);
  });
}
