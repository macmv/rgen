use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{BiomeBuilder, IdContext};

pub fn cherry_blossom_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
  println!("hey neil, hey neil, how you doing?");
  gen.id = ctx.biomes.birch_forest;
  gen.color = "#A3B5A0";
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place("Small Cherry Tree", PlacerStage::Tree, placer::Sakura::new(ctx.blocks));
  gen.place(
    "sprinkling of bamboo",
    PlacerStage::Sand,
    placer::BambooClump {
      attempts:      10,
      avg_per_chunk: 3.0,
      place_above:   gen.top_block.into(),
      radius:        1..=4,
      bamboo:        placer::Bamboo {
        place_above:  gen.top_block.into(),
        stalk:        ctx.blocks.rgen_bamboo.default_state,
        pint_size:    true,
        avg_in_chunk: 0.0,
      },
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    800,
      place_above: [ctx.blocks.grass.block].into(),
      place:       ctx.blocks.tallgrass.with_data(1),
    },
  );

  gen.place(
    "tall grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block.into(),
      place_short:      ctx.blocks.tallgrass.with_data(1), // Grass
      place_tall_lower: ctx.blocks.double_plant.with_data(2), // Tall grass lower
      place_tall_upper: ctx.blocks.double_plant.with_data(10), // Tall grass upper

      radius:   4..=10,
      attempts: 100,
    },
  );

  gen.place(
    "Lilac",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block.into(),
      place_short:      ctx.blocks.tallgrass.with_data(1), // Grass
      place_tall_lower: ctx.blocks.double_plant.with_data(1), // lilac bottom
      place_tall_upper: ctx.blocks.double_plant.with_data(10), // double plant top

      radius:   4..=10,
      attempts: 40,
    },
  );
}

pub fn cherry_blossom_wood(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.color = "#7C8F6B";
  gen.top_block = ctx.blocks.grass.default_state;
  gen.place("Small Cherry Tree", PlacerStage::Tree, placer::Sakura::new(ctx.blocks));
}

pub fn birch_woodland(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.color = "#768A6A";
  gen.top_block = ctx.blocks.grass.default_state;

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

//Woodland, Lavendar grove, Field, AspenWood, Volcano growth
