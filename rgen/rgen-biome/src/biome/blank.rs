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
