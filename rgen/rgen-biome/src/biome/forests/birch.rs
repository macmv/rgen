use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::super::{BiomeBuilder, IdContext};

pub fn birch_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.color = "#768A6A";
  gen.set_top_block(ctx.blocks.grass.default_state);

  gen.place("Basalt Pillar", PlacerStage::Tree, placer::RiverSide::new(ctx.blocks));

  gen.place(
    "birch log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            ctx.blocks.log.with_data(2),
      moss_log:       ctx.blocks.rgen_mossy_stump.with_data(1),
      ground:         ctx.blocks.grass.default_state,
      plants:         ctx.blocks.stone.default_state.into(),
      avg_per_chunk:  1.75,
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
      avg_per_chunk:    5.0,
      is_shrooms:       true,
      chance_of_shroom: 100.0,
      shroom:           ctx.blocks.rgen_polypore.with_data(0),
      ground:           ctx.blocks.grass.default_state,
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

pub fn birch_woodland(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.color = "#768A6A";
  gen.set_top_block(ctx.blocks.grass.default_state);

  gen.place(
    "birch log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            ctx.blocks.log.with_data(2),
      moss_log:       ctx.blocks.rgen_mossy_stump.with_data(1),
      ground:         ctx.blocks.grass.default_state,
      plants:         ctx.blocks.stone.default_state.into(),
      avg_per_chunk:  1.75,
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

pub fn aspen_wood(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.set_top_block(ctx.blocks.grass.default_state);
  gen.color = "#B0C2A5";

  gen.place("Aspen Tree", PlacerStage::Tree, placer::AspenTree::new(ctx.blocks));

  gen.place(
    "birch log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            ctx.blocks.log.with_data(2),
      moss_log:       ctx.blocks.log.with_data(2), //ctx.blocks.rgen_mossy_stump.with_data(1), //
      ground:         ctx.blocks.grass.default_state,
      plants:         ctx.blocks.stone.default_state.into(),
      avg_per_chunk:  1.75,
      chance_of_moss: 015,
      is_shrooms:     false,
      shroom:         ctx.blocks.rgen_polypore.default_state,
    },
  );

  gen.place(
    "bushes",
    PlacerStage::Tree,
    placer::BasicDryBush {
      avg_in_chunk: 13 as f64,
      leaves:       ctx.blocks.rgen_leaves3.default_state,
      place_above:  ctx.blocks.grass.block.into(),
      trunk:        ctx.blocks.log.with_data(2),
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    600,
      place_above: [ctx.blocks.grass.block, ctx.blocks.rgen_mossy_stump.block].into(),
      place:       ctx.blocks.tallgrass.with_data(2),
    },
  );
  gen.place(
    "tall grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block().into(),
      place_short:      ctx.blocks.tallgrass.with_data(1), // Grass
      place_tall_lower: ctx.blocks.double_plant.with_data(2), // Tall grass lower
      place_tall_upper: ctx.blocks.double_plant.with_data(10), // Tall grass upper

      radius:        6..=10,
      attempts:      300,
      avg_per_chunk: 3.0,
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    2300,
      place_above: [ctx.blocks.grass.block].into(),
      place:       ctx.blocks.tallgrass.with_data(1),
    },
  );
}
