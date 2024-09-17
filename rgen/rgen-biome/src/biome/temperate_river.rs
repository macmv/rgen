use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{BiomeBuilder, IdContext};

pub fn cherry_blossom_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  println!("hey neil, hey neil, how you doing?");
  gen.id = ctx.biomes.birch_forest;
  gen.color = "#A3B5A0";
  gen.set_top_block(ctx.blocks.grass.default_state);

  gen.place("Small Cherry Tree", PlacerStage::Tree, placer::Sakura::new(ctx.blocks));
  gen.place(
    "sprinkling of bamboo",
    PlacerStage::Sand,
    placer::BambooClump {
      attempts:      10,
      avg_per_chunk: 3.0,
      place_above:   gen.top_block().into(),
      radius:        1..=4,
      bamboo:        placer::Bamboo {
        place_above:  gen.top_block().into(),
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
      place_above:      gen.top_block().into(),
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
      place_above:      gen.top_block().into(),
      place_short:      ctx.blocks.tallgrass.with_data(1), // Grass
      place_tall_lower: ctx.blocks.double_plant.with_data(1), // lilac bottom
      place_tall_upper: ctx.blocks.double_plant.with_data(10), // double plant top

      radius:   4..=10,
      attempts: 40,
    },
  );
}

pub fn birch_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.color = "#768A6A";
  gen.set_top_block(ctx.blocks.grass.default_state);

  //gen.place("Basalt Pillar", PlacerStage::Tree,
  // placer::Pillar::new(ctx.blocks));

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

  gen.place(
    "forget me not",
    PlacerStage::Sand2,
    placer::Spread {
      place:         ctx.blocks.rgen_flower.with_data(0),
      replace:       ctx.blocks.grass.default_state.into(),
      radius:        1..=3,
      avg_per_chunk: 0.3,
    },
  );
}

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
pub fn lavender_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.color = "#899781";
  gen.set_top_block(ctx.blocks.grass.default_state);

  gen.place(
    "SmallLavenderScatter",
    PlacerStage::Tree,
    placer::LavenderScatter {
      attempts:    900,
      place_above: [ctx.blocks.grass.block].into(),
      is_large:    false,
      place:       ctx.blocks.rgen_lavender.default_state,
    },
  );
  gen.place(
    "LargeLavenderScatter",
    PlacerStage::Tree,
    placer::LavenderScatter {
      attempts:    600,
      place_above: [ctx.blocks.grass.block].into(),
      is_large:    true,
      place:       ctx.blocks.rgen_tall_lavender.default_state,
    },
  );
}
pub fn volcano_river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#899781";
  gen.set_top_block(ctx.blocks.grass.default_state);
  gen.add_layer(ctx.blocks.rgen_basalt.default_state, 5, 8);

  gen.place("Basalt Pillar", PlacerStage::Tree, placer::Pillar::new(ctx.blocks));
  gen.place(
    "basalt_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.rgen_basalt.with_data(0),
      radius:        2..=4,
      avg_per_chunk: 1.0,
    },
  );
  gen.place(
    "sprinkling of bamboo",
    PlacerStage::Sand,
    placer::BambooClump {
      attempts:      1,
      avg_per_chunk: 1.0,
      place_above:   gen.top_block().into(),
      radius:        1..=2,
      bamboo:        placer::Bamboo {
        place_above:  gen.top_block().into(),
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
      place_above: [ctx.blocks.grass.block, ctx.blocks.rgen_mossy_stump.block].into(),
      place:       ctx.blocks.tallgrass.with_data(1),
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

      radius:   4..=10,
      attempts: 200,
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    80,
      place_above: [ctx.blocks.grass.block, ctx.blocks.rgen_mossy_stump.block].into(),
      place:       ctx.blocks.tallgrass.with_data(2),
    },
  );
}
//Field, Volcano growth
