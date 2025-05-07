use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::super::BiomeBuilder;

pub fn birch_river(g: &mut BiomeBuilder) {
  g.id = biome![birch_forest_hills];
  g.color = "#768A6A";
  g.set_top_block(block![grass]);

  g.place("Basalt Pillar", PlacerStage::Tree, placer::RiverSide::new());

  g.place(
    "birch log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            block![log[variant = "birch"]],
      moss_log:       block![rgen:mossy_stump[variant = "birch"]],
      ground:         block![grass],
      plants:         block![stone].into(),
      avg_per_chunk:  1.75,
      chance_of_moss: 5,
      is_shrooms:     true,
      shroom:         block![rgen:polypore],
    },
  );
  g.place(
    "basic birch tree",
    PlacerStage::Tree,
    placer::BasicBirch {
      trunk:            block![log[variant = "birch"]],
      leaves:           block![leaves[variant = "birch"]],
      avg_per_chunk:    5.0,
      is_shrooms:       true,
      chance_of_shroom: 100.0,
      shroom:           block![rgen:polypore[type = "one"]],
      ground:           block![grass],
    },
  );

  g.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      avg_per_chunk: 1.0,
      attempts:      160,
      place_above:   [block![grass], block![rgen:mossy_stump]].into(),
      place:         block![tallgrass[type = "fern"]],
    },
  );
  g.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      avg_per_chunk: 1.0,
      attempts:      160,
      place_above:   [block![grass], block![rgen:mossy_stump]].into(),
      place:         block![tallgrass[type = "tall_grass"]],
    },
  );
}

pub fn birch_woodland(g: &mut BiomeBuilder) {
  g.id = biome![birch_forest_hills];
  g.color = "#768A6A";
  g.set_top_block(block![grass]);

  g.place(
    "birch log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            block![log[variant = "birch"]],
      moss_log:       block![rgen:mossy_stump[variant = "birch"]],
      ground:         block![grass],
      plants:         block![stone].into(),
      avg_per_chunk:  1.75,
      chance_of_moss: 5,
      is_shrooms:     true,
      shroom:         block![rgen:polypore],
    },
  );
  g.place(
    "basic birch tree",
    PlacerStage::Tree,
    placer::BasicBirch {
      trunk:            block![log[variant = "birch"]],
      leaves:           block![leaves[variant = "birch"]],
      avg_per_chunk:    12.0,
      is_shrooms:       true,
      chance_of_shroom: 100.0,
      shroom:           block![rgen:polypore[type = "one"]],
      ground:           block![grass],
    },
  );

  g.place(
    "mossy carpet",
    PlacerStage::Sand2,
    placer::Spread {
      place:         block![rgen:mossy_carpet],
      replace:       block![grass].into(),
      radius:        4..=5,
      avg_per_chunk: 1.0,
    },
  );
  g.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      avg_per_chunk: 1.0,
      attempts:      160,
      place_above:   [block![grass], block![rgen:mossy_stump]].into(),
      place:         block![tallgrass[type = "fern"]],
    },
  );
  g.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      avg_per_chunk: 1.0,
      attempts:      160,
      place_above:   [block![grass], block![rgen:mossy_stump]].into(),
      place:         block![tallgrass[type = "tall_grass"]],
    },
  );

  g.place(
    "forget me not",
    PlacerStage::Sand2,
    placer::Spread {
      place:         block![rgen:flower[type = "forgetmenot"]],
      replace:       block![grass].into(),
      radius:        1..=3,
      avg_per_chunk: 0.6,
    },
  );
}

pub fn aspen_wood(g: &mut BiomeBuilder) {
  g.id = biome![birch_forest_hills];
  g.set_top_block(block![grass]);
  g.color = "#B0C2A5";

  g.place("Aspen Tree", PlacerStage::Tree, placer::AspenTree::new());

  g.place(
    "birch log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            block![log[variant = "birch"]],
      // block![rgen_mossy_stump[variant = "birch"]],
      moss_log:       block![log[variant = "birch"]],
      ground:         block![grass],
      plants:         block![stone].into(),
      avg_per_chunk:  1.75,
      chance_of_moss: 15,
      is_shrooms:     false,
      shroom:         block![rgen:polypore],
    },
  );

  g.place(
    "bushes",
    PlacerStage::Tree,
    placer::BasicDryBush {
      avg_per_chunk: 13_f64,
      leaves:        block![rgen:leaves3],
      place_above:   block![grass].into(),
      trunk:         block![log[variant = "birch"]],
    },
  );

  g.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      avg_per_chunk: 1.0,
      attempts:      600,
      place_above:   [block![grass], block![rgen:mossy_stump]].into(),
      place:         block![tallgrass[type = "fern"]],
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

      radius:        6..=10,
      attempts:      300,
      avg_per_chunk: 3.0,
    },
  );

  g.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      avg_per_chunk: 1.0,
      attempts:      2300,
      place_above:   [block![grass]].into(),
      place:         block![tallgrass[type = "tall_grass"]],
    },
  );
}
