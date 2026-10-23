#![cfg(feature = "inspector")]

use rgen_base::{ChunkPos, ChunkRelPos, StateId, block};
use rgen_biome::{
  WorldBiomes,
  toggles::{self, Toggle},
};
use rgen_world::{Context, Region};

fn count(region: &Region, state: StateId) -> usize {
  let mut n = 0;
  for (_, chunk) in region.chunks() {
    for x in 0..16 {
      for z in 0..16 {
        for y in 0..256 {
          n += (chunk.get(ChunkRelPos::new(x, y, z)) == state) as usize;
        }
      }
    }
  }
  n
}

// This is one test, because the toggles are global.
#[test]
fn toggles_turn_parts_of_the_generator_off() {
  let ctx = Context::new_test(99);
  let world = WorldBiomes::new(&ctx.blocks, ctx.seed);
  let (min, max) = (ChunkPos::new(10, 10), ChunkPos::new(11, 11));
  let coal = ctx.blocks.encode(block![coal_ore]);
  let stone = ctx.blocks.encode(block![stone]);

  let region = Region::generate(&ctx, &world, min, max);
  assert!(count(&region, coal) > 0, "there should be coal to turn off");

  toggles::set(Toggle::Ores, false);
  let region = Region::generate(&ctx, &world, min, max);
  assert_eq!(count(&region, coal), 0);
  toggles::set(Toggle::Ores, true);

  // A toggle's parts go off with it.
  toggles::set(Toggle::CheeseCaves, false);
  assert!(!toggles::enabled(Toggle::CheeseWaterGuard));
  toggles::set(Toggle::CheeseCaves, true);

  // With no caves, there is more stone.
  let with_caves = count(&Region::generate(&ctx, &world, min, max), stone);
  toggles::set(Toggle::Caves, false);
  let without_caves = count(&Region::generate(&ctx, &world, min, max), stone);
  toggles::set(Toggle::Caves, true);
  assert!(without_caves >= with_caves);

  // Every decoration can be named, and turned off.
  let names = world.decoration_names();
  assert!(!names.is_empty());
  toggles::set_placer(names[0], false);
  assert!(!toggles::placer_enabled(names[0]));
  toggles::set_placer(names[0], true);
}
