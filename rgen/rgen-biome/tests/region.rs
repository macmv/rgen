#![cfg(feature = "inspector")]

use rgen_base::{ChunkPos, ChunkRelPos, StateId};
use rgen_biome::{Splines, WorldBiomes, set_splines};
use rgen_world::{Context, Region};

/// The y of the highest block in the column at the corner of `pos`.
fn top(region: &Region, pos: ChunkPos) -> i32 {
  let chunk = region.chunk(pos).unwrap();
  (0..256).rev().find(|&y| chunk.get(ChunkRelPos::new(0, y, 0)) != StateId::AIR).unwrap_or(0)
}

// This is one test, because the splines are global.
#[test]
fn generates_a_region_and_follows_the_splines() {
  let ctx = Context::new_test(1234);
  let world = WorldBiomes::new(&ctx.blocks, ctx.seed);
  let pos = ChunkPos::new(40, -25);

  set_splines(Splines::game());
  let region = Region::generate(&ctx, &world, pos, pos);
  assert_eq!(region.chunks().count(), 1);
  let before = top(&region, pos);
  assert!(before > 0);

  // Raise the whole world by 40 blocks, by changing what continentalness maps to.
  let mut splines = Splines::game();
  for key in &mut splines.continentalness.storage {
    key.1 += 40.0;
  }
  set_splines(splines);
  let region = Region::generate(&ctx, &world, pos, pos);
  assert_ne!(top(&region, pos), before);

  set_splines(Splines::game());
}
