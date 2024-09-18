use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::super::{BiomeBuilder, IdContext};

pub fn woodland_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.color = "#899781";
  gen.set_top_block(ctx.blocks.grass.default_state);

  gen.place("Oak tree", PlacerStage::Tree, placer::OakTree::new(ctx.blocks));

  gen.place(
    "oak log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            ctx.blocks.log.with_data(0),
      moss_log:       ctx.blocks.rgen_mossy_stump.with_data(0),
      ground:         ctx.blocks.grass.default_state,
      plants:         ctx.blocks.stone.default_state.into(),
      avg_per_chunk:  0.5,
      chance_of_moss: 8,
      is_shrooms:     true,
      shroom:         ctx.blocks.rgen_polypore.default_state,
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
}

pub fn woodland(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.color = "#899781";
  gen.set_top_block(ctx.blocks.grass.default_state);

  gen.place("Oak tree", PlacerStage::Tree, placer::OakTree::new(ctx.blocks));

  gen.place(
    "oak log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            ctx.blocks.log.with_data(0),
      moss_log:       ctx.blocks.rgen_mossy_stump.with_data(0),
      ground:         ctx.blocks.grass.default_state,
      plants:         ctx.blocks.stone.default_state.into(),
      avg_per_chunk:  0.5,
      chance_of_moss: 8,
      is_shrooms:     true,
      shroom:         ctx.blocks.rgen_polypore.default_state,
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
}
