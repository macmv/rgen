use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{BiomeBuilder, IdContext};

pub fn blank(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  // White wool
  gen.top_block = ctx.blocks.grass.default_state;
  gen.sub_layer = ctx.blocks.dirt.default_state;

  gen.place(
    "grass clumps",
    PlacerStage::Sand,
    placer::Clumps {
      place_above: ctx.blocks.grass.default_state.into(),
      place:       ctx.blocks.tallgrass.with_data(1),

      attempts:      20,
      avg_per_chunk: 3.0,

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
const PURPLE: u8 = 10;
const BLUE: u8 = 11;
const BROWN: u8 = 12;
const GREEN: u8 = 13;
