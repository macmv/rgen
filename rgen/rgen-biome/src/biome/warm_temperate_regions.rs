use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{BiomeBuilder, IdContext};

pub fn cherry_blossom_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
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

      radius:        4..=10,
      attempts:      100,
      avg_per_chunk: 3.0,
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

      radius:        4..=10,
      attempts:      40,
      avg_per_chunk: 3.0,
    },
  );
}

pub fn cherry_blossom_wood(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.birch_forest;
  gen.color = "#7C8F6B";
  gen.set_top_block(ctx.blocks.grass.default_state);
  gen.place("Small Cherry Tree", PlacerStage::Tree, placer::Sakura::new(ctx.blocks));
}

pub fn lavender_grove(ctx: &IdContext, gen: &mut BiomeBuilder) {
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
pub fn volcano_growth(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#899781";
  gen.set_top_block(ctx.blocks.grass.default_state);
  gen.add_layer(ctx.blocks.rgen_basalt.default_state, 5, 8);

  gen.place("Lava Lake", PlacerStage::Tree, placer::LavaLake::new(ctx.blocks));
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

      radius:        4..=10,
      attempts:      200,
      avg_per_chunk: 3.0,
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    100,
      place_above: [ctx.blocks.grass.block, ctx.blocks.rgen_mossy_stump.block].into(),
      place:       ctx.blocks.tallgrass.with_data(2),
    },
  );
}
