//! This includes the ICE_CAP, TUNDRA, SUB_ARCTIC, COLD_SWAMP, and
//! COOL_TEMPERATE biome categories.

use rgen_placer::placer;

use super::{color, IdContext};
use crate::builder::{BiomeBuilder, PlacerStage};

pub fn ice_spikes(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#E3F5FC";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::LIGHT_BLUE));

  gen.place(
    "ice_patches",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block(),
      place:    ctx.blocks.ice.default_state,
      attempts: 100,
    },
  );
}

pub fn broken_glacier(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#82C5E1";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::LIGHT_BLUE));

  gen.place(
    "ice_patches",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block(),
      place:    ctx.blocks.ice.default_state,
      attempts: 100,
    },
  );
}

pub fn glacier(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#6FAFCE";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::LIGHT_BLUE));

  gen.place(
    "ice_patches",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block(),
      place:    ctx.blocks.ice.default_state,
      attempts: 100,
    },
  );
}

pub fn rocky_plains(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.color = "#B2DBEF";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::GRAY));

  gen.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splatter {
      replace:  gen.top_block(),
      place:    ctx.blocks.cobblestone.default_state,
      attempts: 100,
    },
  );
}

pub fn rockies(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.color = "#4E9BB7";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::GRAY));
}

pub fn frozen_meadow(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.color = "#748A99";
  gen.set_top_block(ctx.blocks.concrete.default_state);
}

pub fn frozen_desert(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.color = "#B4BDC8";
  gen.set_top_block(ctx.blocks.concrete.default_state);
}

pub fn snowy_plains(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.cold_taiga;
  gen.color = "#8F9BAB";
  gen.set_top_block(ctx.blocks.grass.default_state);

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
  gen.color = "#6F7D8B";
  gen.set_top_block(ctx.blocks.grass.default_state);

  gen.place(
    "spruce_tree",
    PlacerStage::Sand,
    placer::BasicTree {
      avg_in_chunk: 16.0,
      place_above:  gen.top_block().into(),
      trunk:        ctx.blocks.log.default_state,
      leaves:       ctx.blocks.leaves.default_state,
    },
  );
}

pub fn fir_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#9DA8B7";
  gen.set_top_block(ctx.blocks.grass.default_state);

  gen.place(
    "fir_tree",
    PlacerStage::Sand,
    placer::BasicTree {
      avg_in_chunk: 16.0,
      place_above:  gen.top_block().into(),
      trunk:        ctx.blocks.log.default_state,
      leaves:       ctx.blocks.leaves.default_state,
    },
  );
}

pub fn spruce_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#BCC4D1";
  gen.set_top_block(ctx.blocks.grass.default_state);

  gen.place(
    "spruce_tree",
    PlacerStage::Sand,
    placer::BasicTree {
      avg_in_chunk: 16.0,
      place_above:  gen.top_block().into(),
      trunk:        ctx.blocks.log.default_state,
      leaves:       ctx.blocks.leaves.default_state,
    },
  );
}

pub fn snowy_crag(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#8997A7";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::SILVER));

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
