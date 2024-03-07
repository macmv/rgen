use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{BiomeBuilder, IdContext};

pub fn blank(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  // White wool
  gen.top_block = ctx.blocks.wool.default_state;
  gen.sub_layer = ctx.blocks.wool.with_data(PURPLE);

  gen.place(
    "grass",
    PlacerStage::Sand,
    placer::Splotch {
      replace: ctx.blocks.wool.default_state.into(),
      place:   ctx.blocks.grass.default_state,

      radius: 3..=8,
    },
  );
}

// Wool colors:
// const WHITE: u8 = 0;
// const ORANGE: u8 = 1;
// const MAGENTA: u8 = 2;
// const LIGHT_BLUE: u8 = 3;
// const YELLOW: u8 = 4;
// const LIME: u8 = 5;
// const PINK: u8 = 6;
// const GRAY: u8 = 7;
// const SILVER: u8 = 8;
// const CYAN: u8 = 9;
// const PURPLE: u8 = 10;
// const BLUE: u8 = 11;
// const BROWN: u8 = 12;
// const GREEN: u8 = 13;
// const RED: u8 = 14;
// const BLACK: u8 = 15;

const LIGHT_BLUE: u8 = 3;
const SILVER: u8 = 8;
const PURPLE: u8 = 10;
const BLUE: u8 = 11;
const BROWN: u8 = 12;
const GREEN: u8 = 13;

pub fn crag(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(SILVER);

  gen.place(
    "grass",
    PlacerStage::Sand,
    placer::Splatter {
      replace: ctx.blocks.stone.default_state,
      place:   ctx.blocks.grass.default_state,

      attempts: 50,
    },
  );
  gen.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splatter {
      replace: ctx.blocks.stone.default_state,
      place:   ctx.blocks.gravel.default_state,

      attempts: 100,
    },
  );
  gen.place(
    "cobble",
    PlacerStage::Sand,
    placer::Splatter {
      replace: ctx.blocks.stone.default_state,
      place:   ctx.blocks.mossy_cobblestone.default_state,

      attempts: 100,
    },
  );
}

pub fn bog(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(BROWN);
}
pub fn cold_bog(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(BLUE);
}
pub fn fall_bog(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(LIGHT_BLUE);
}
pub fn conifer_swamp(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(GREEN);
}

pub fn birch_woodland(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "basic birch tree",
    PlacerStage::Tree,
    placer::BasicBirch {
      trunk:            ctx.blocks.log.with_data(2),
      leaves:           ctx.blocks.leaves.with_data(2),
      avg_per_chunk:    12.0,
      is_shrooms:       true,
      chance_of_shroom: 100.0,
      shroom:           ctx.blocks.rgen_polypore.with_data(0),
      ground:           ctx.blocks.grass.default_state,
    },
  );
  gen.place(
    "birch log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            ctx.blocks.log.with_data(2),
      moss_log:       ctx.blocks.rgen_mossy_stump.with_data(1),
      plants:         ctx.blocks.stone.default_state.into(),
      avg_per_chunk:  8.0,
      chance_of_moss: 50,
    },
  );
  gen.place(
    "mossy carpet",
    PlacerStage::Sand2,
    placer::Spread {
      place:   ctx.blocks.rgen_mossy_carpet.default_state,
      replace: ctx.blocks.grass.default_state.into(),
      radius:  4..=5,
    },
  )
}
