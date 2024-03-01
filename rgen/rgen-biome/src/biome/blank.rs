use rgen_base::BlockState;
use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{BiomeBuilder, IdContext};

pub fn blank(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.stone;

  gen.place(
    "grass",
    PlacerStage::Sand,
    placer::Splatter {
      replace: BlockState { block: ctx.blocks.stone, state: 0 },
      place:   BlockState { block: ctx.blocks.grass, state: 0 },

      attempts: 50,
    },
  );
  gen.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splatter {
      replace: BlockState { block: ctx.blocks.stone, state: 0 },
      place:   BlockState { block: ctx.blocks.gravel, state: 0 },

      attempts: 100,
    },
  );
  gen.place(
    "cobble",
    PlacerStage::Sand,
    placer::Splatter {
      replace: BlockState { block: ctx.blocks.stone, state: 0 },
      place:   BlockState { block: ctx.blocks.mossy_cobblestone, state: 0 },

      attempts: 100,
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
  gen.top_block = ctx.blocks.wool;
  gen.top_block_data = GRAY;
}

pub fn bog(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool;
  gen.top_block_data = BROWN;
}

pub fn cool_valley(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool;
  gen.top_block_data = CYAN;
}

pub fn warm_valley(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool;
  gen.top_block_data = YELLOW;
}

pub fn swamp(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool;
  gen.top_block_data = GREEN;
}

pub fn dry_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool;
  gen.top_block_data = ORANGE;
}

pub fn hot_swamp(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool;
  gen.top_block_data = RED;
}

pub fn tropic_swamp(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool;
  gen.top_block_data = LIME;
}
