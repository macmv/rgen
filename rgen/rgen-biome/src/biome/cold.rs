//! This includes the ICE_CAP, TUNDRA, SUB_ARCTIC, COLD_SWAMP, and
//! COOL_TEMPERATE biome categories.

use rgen_placer::placer;

use super::IdContext;
use crate::builder::{BiomeBuilder, PlacerStage};

const LIGHT_BLUE: u8 = 3;
const GRAY: u8 = 7;
const SILVER: u8 = 8;

pub fn ice_spikes(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.top_block = ctx.blocks.wool.with_data(LIGHT_BLUE);

  gen.place(
    "ice_patches",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block,
      place:    ctx.blocks.ice.default_state,
      attempts: 100,
    },
  );
}

pub fn broken_glacier(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.top_block = ctx.blocks.wool.with_data(LIGHT_BLUE);

  gen.place(
    "ice_patches",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block,
      place:    ctx.blocks.ice.default_state,
      attempts: 100,
    },
  );
}

pub fn glacier(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.top_block = ctx.blocks.wool.with_data(LIGHT_BLUE);

  gen.place(
    "ice_patches",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block,
      place:    ctx.blocks.ice.default_state,
      attempts: 100,
    },
  );
}

pub fn rocky_plains(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.top_block = ctx.blocks.wool.with_data(GRAY);

  gen.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block,
      place:    ctx.blocks.cobblestone.default_state,
      attempts: 100,
    },
  );
}

pub fn rockies(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.top_block = ctx.blocks.wool.with_data(GRAY);
}

pub fn frozen_meadow(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.top_block = ctx.blocks.wool.default_state;
}

pub fn frozen_desert(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.top_block = ctx.blocks.wool.default_state;
}

pub fn snowy_plains(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "snow",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  ctx.blocks.stone.default_state,
      place:    ctx.blocks.snow.default_state,
      attempts: 100,
    },
  );
}

pub fn snowy_woodland(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "spruce_tree",
    PlacerStage::Sand,
    placer::BasicTree {
      trunk:  ctx.blocks.log.default_state,
      leaves: ctx.blocks.leaves.default_state,
    },
  );
}

pub fn fir_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "fir_tree",
    PlacerStage::Sand,
    placer::BasicTree {
      trunk:  ctx.blocks.log.default_state,
      leaves: ctx.blocks.leaves.default_state,
    },
  );
}

pub fn spruce_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "spruce_tree",
    PlacerStage::Sand,
    placer::BasicTree {
      trunk:  ctx.blocks.log.default_state,
      leaves: ctx.blocks.leaves.default_state,
    },
  );
}

pub fn snowy_crag(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(SILVER);

  gen.place(
    "snow",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  ctx.blocks.stone.default_state,
      place:    ctx.blocks.snow.default_state,
      attempts: 100,
    },
  );
}
