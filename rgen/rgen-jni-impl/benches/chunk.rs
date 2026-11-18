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

/// Fixed valid-world coordinates and block checksums for before/after changes.
/// Run with `cargo +nightly test --release -p rgen-jni-impl --bench chunk
/// compare_fixed_workloads -- --nocapture`. Each invocation starts cold.
#[test]
fn compare_fixed_workloads() {
  let seed = std::env::var("RGEN_BENCH_SEED").ok()
    .and_then(|s| s.parse().ok()).unwrap_or(1);
  let context = rgen_world::Context::new_test(seed);
  let generator = WorldBiomes::new(&context.blocks, context.seed);
  for distant in [true, false] {
    if !distant {
      for x in 100..116 {
        generator.generate_base(&context, &mut Chunk::new(), ChunkPos::new(x,100));
      }
    }
    let count = if distant { 32 } else { 128 };
    let mut total = std::time::Duration::ZERO;
    let mut checksum = 0xcbf29ce484222325_u64;
    for i in 0..count {
      let pos = if distant {
        ChunkPos::new((i*2654435761_u64 % 8192) as i32-4096,
          ((i*2246822519_u64+1013) % 8192) as i32-4096)
      } else { ChunkPos::new(100+(i%16) as i32,101+(i/16) as i32) };
      let mut chunk = Chunk::new();
      let start = std::time::Instant::now();
      generator.generate_base(&context, &mut chunk, pos);
      total += start.elapsed();
      // Outside the timing: every block must match after optimizations.
      for &block in chunk.data() {
        checksum = (checksum ^ block as u64).wrapping_mul(0x100000001b3);
      }
    }
    println!("FIXED seed={seed} workload={} count={count} mean_ms={:.4} checksum={checksum:016x}",
      if distant { "distant" } else { "neighboring" },total.as_secs_f64()*1000.0/count as f64);
  }
}
