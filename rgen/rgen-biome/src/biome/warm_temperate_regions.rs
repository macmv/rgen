use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::BiomeBuilder;

#[allow(dead_code)]
pub fn cherry_blossom_grove(g: &mut BiomeBuilder) {
  g.id = biome![birch_forest_hills];
  g.color = "#A3B5A0";
  g.set_top_block(block![grass]);

  g.place("Small Cherry Tree", PlacerStage::Tree, placer::Sakura::new());
  g.place(
    "sprinkling of bamboo",
    PlacerStage::Sand,
    placer::BambooClump {
      attempts:      10,
      avg_per_chunk: 3.0,
      place_above:   g.top_block().into(),
      radius:        1..=4,
      bamboo:        placer::Bamboo {
        place_above:   g.top_block().into(),
        stalk:         block![rgen:bamboo],
        pint_size:     true,
        avg_per_chunk: 0.0,
      },
    },
  );
  g.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    800,
      place_above: [block![grass]].into(),
      place:       block![tallgrass[type = "tall_grass"]],
    },
  );

  g.place(
    "tall grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      g.top_block().into(),
      place_short:      block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[half = "lower", variant = "double_grass"]],
      place_tall_upper: block![double_plant[half = "upper"]],

      radius:        4..=10,
      attempts:      100,
      avg_per_chunk: 3.0,
    },
  );

  g.place(
    "Lilac",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      g.top_block().into(),
      place_short:      block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[half = "lower", variant = "double_grass"]],
      place_tall_upper: block![double_plant[half = "upper"]],

      radius:        4..=10,
      attempts:      40,
      avg_per_chunk: 3.0,
    },
  );
}

#[allow(dead_code)]
pub fn cherry_blossom_wood(g: &mut BiomeBuilder) {
  g.id = biome![birch_forest_hills];
  g.color = "#7C8F6B";
  g.set_top_block(block![grass]);
  g.place("Small Cherry Tree", PlacerStage::Tree, placer::Sakura::new());
}

#[allow(dead_code)]
pub fn lavender_grove(g: &mut BiomeBuilder) {
  g.id = biome![birch_forest_hills];
  g.color = "#899781";
  g.set_top_block(block![grass]);

  g.place(
    "SmallLavenderScatter",
    PlacerStage::Tree,
    placer::LavenderScatter {
      attempts:    900,
      place_above: [block![grass]].into(),
      is_large:    false,
      place:       block![rgen:lavender_plant],
    },
  );
  g.place(
    "LargeLavenderScatter",
    PlacerStage::Tree,
    placer::LavenderScatter {
      attempts:    600,
      place_above: [block![grass]].into(),
      is_large:    true,
      place:       block![rgen:double_tall_lavender_plant],
    },
  );
}

#[allow(dead_code)]
pub fn volcano_growth(g: &mut BiomeBuilder) {
  g.id = biome![plains];
  g.color = "#899781";
  g.set_top_block(block![grass]);
  g.add_layer(block![rgen:basalt], 5, 8);

  g.place("Lava Lake", PlacerStage::Tree, placer::LavaLake::new());
  g.place("Basalt Pillar", PlacerStage::Tree, placer::Pillar::new());
  g.place(
    "basalt_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       g.top_block().into(),
      place:         block![rgen:basalt],
      radius:        2..=4,
      avg_per_chunk: 1.0,
    },
  );
  g.place(
    "sprinkling of bamboo",
    PlacerStage::Sand,
    placer::BambooClump {
      attempts:      10,
      avg_per_chunk: 3.0,
      place_above:   g.top_block().into(),
      radius:        1..=4,
      bamboo:        placer::Bamboo {
        place_above:   g.top_block().into(),
        stalk:         block![rgen:bamboo],
        pint_size:     true,
        avg_per_chunk: 0.0,
      },
    },
  );
  g.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    800,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[type = "tall_grass"]],
    },
  );

  g.place(
    "tall grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      g.top_block().into(),
      place_short:      block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[half = "lower", variant = "double_grass"]],
      place_tall_upper: block![double_plant[half = "upper"]],

      radius:        4..=10,
      attempts:      200,
      avg_per_chunk: 3.0,
    },
  );

  g.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    100,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[type = "fern"]],
    },
  );
}
