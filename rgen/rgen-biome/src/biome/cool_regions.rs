use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{BiomeBuilder, IdContext};

const LIGHT_BLUE: u8 = 3;
const SILVER: u8 = 8;
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
