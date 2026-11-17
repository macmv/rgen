#![feature(test)]

extern crate test;

use std::sync::Arc;

use rgen_base::{Chunk, ChunkPos};
use rgen_biome::WorldBiomes;
use rgen_world::{CachedWorld, Generator};
use test::Bencher;

#[bench]
fn bench_sequential(b: &mut Bencher) {
  let world = Arc::new(CachedWorld::new());
  let context = Arc::new(rgen_world::Context::new_test(1));
  let generator = Arc::new(WorldBiomes::new(&context.blocks, context.seed));
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
  let context = Arc::new(rgen_world::Context::new_test(1));
  let generator = Arc::new(WorldBiomes::new(&context.blocks, context.seed));

  let mut chunk_pos = ChunkPos::new(0, 0);

  // The first couple chunks are the slowest, so warm up a bit before calling
  // `b.iter()`, so that it won't run way too many samples.
  for _ in 0..5 * 20 {
    chunk_pos.x += 1;
    if chunk_pos.x > 20 {
      chunk_pos.x = 0;
      chunk_pos.z += 1;
    }

    let mut chunk = Chunk::new();
    generator.generate_base(&context, &mut chunk, chunk_pos);
  }

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

#[bench]
fn bench_base_spread(b: &mut Bencher) {
  let context = Arc::new(rgen_world::Context::new_test(1));
  let generator = Arc::new(WorldBiomes::new(&context.blocks, context.seed));

  // Spread samples across a large seeded area instead of walking neighboring
  // chunks. This shows whether generation stays quick when noise/cache locality
  // from sequential exploration is absent.
  let positions: Vec<_> = (0..256_u64)
    .map(|i| {
      let x = i.wrapping_mul(0x9E37_79B9).rotate_left(17) as i32;
      let z = i.wrapping_mul(0x85EB_CA6B).rotate_left(31) as i32;
      ChunkPos::new(x, z)
    })
    .collect();
  let mut index = 0;

  // Warm up the generator and allocator without warming the timed positions.
  for pos in positions.iter().take(5) {
    let mut chunk = Chunk::new();
    generator.generate_base(&context, &mut chunk, *pos);
  }

  b.iter(|| {
    let pos = positions[index];
    index = (index + 1) % positions.len();
    let mut chunk = Chunk::new();
    generator.generate_base(&context, &mut chunk, pos);
    test::black_box(chunk);
  });
}
