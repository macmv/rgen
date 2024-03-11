use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{BiomeBuilder, IdContext};

pub fn birch_woodland(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "birch log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            ctx.blocks.log.with_data(2),
      moss_log:       ctx.blocks.rgen_mossy_stump.with_data(1),
      ground:         ctx.blocks.grass.default_state,
      plants:         ctx.blocks.stone.default_state.into(),
      avg_per_chunk:  8.0,
      chance_of_moss: 5,
      is_shrooms:     true,
      shroom:         ctx.blocks.rgen_polypore.default_state,
    },
  );
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
    "mossy carpet",
    PlacerStage::Sand2,
    placer::Spread {
      place:         ctx.blocks.rgen_mossy_carpet.default_state,
      replace:       ctx.blocks.grass.default_state.into(),
      radius:        4..=5,
      avg_per_chunk: 1.0,
    },
  );
  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [ctx.blocks.grass.block, ctx.blocks.rgen_mossy_stump.block].into(),
      place:       ctx.blocks.tallgrass.with_data(2),
    },
  );
  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [ctx.blocks.grass.block, ctx.blocks.rgen_mossy_stump.block].into(),
      place:       ctx.blocks.tallgrass.with_data(1),
    },
  );

  gen.place(
    "forget me not",
    PlacerStage::Sand2,
    placer::Spread {
      place:         ctx.blocks.rgen_flower.with_data(0),
      replace:       ctx.blocks.grass.default_state.into(),
      radius:        1..=3,
      avg_per_chunk: 0.6,
    },
  );
}
