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

      radius: 5,
    },
  );
}

const WHITE: u8 = 0;
const ORANGE: u8 = 1;
const MAGENTA: u8 = 2;
const LIGHT_BLUE: u8 = 3;
const YELLOW: u8 = 4;
const LIME: u8 = 5;
const PINK: u8 = 6;
const GRAY: u8 = 7;
const SILVER: u8 = 8;
const CYAN: u8 = 9;
const PURPLE: u8 = 10;
const BLUE: u8 = 11;
const BROWN: u8 = 12;
const GREEN: u8 = 13;
const RED: u8 = 14;
const BLACK: u8 = 15;

pub fn rocky_valley(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(GRAY);
}

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

pub fn cool_valley(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(CYAN);
}

pub fn warm_valley(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(YELLOW);
}

pub fn swamp(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(GREEN);
}

pub fn dry_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(ORANGE);
}

pub fn hot_swamp(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(RED);
}

pub fn tropic_swamp(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.with_data(LIME);
}
