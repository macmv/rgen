use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::super::BiomeBuilder;

pub fn birch_river(gen: &mut BiomeBuilder) {
  gen.id = biome![birch_forest_hills];
  gen.color = "#768A6A";
  gen.set_top_block(block![grass]);

  gen.place("Basalt Pillar", PlacerStage::Tree, placer::RiverSide::new());

  gen.place(
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
  gen.place(
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

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[type = "fern"]],
    },
  );
  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[type = "tall_grass"]],
    },
  );
}

pub fn birch_woodland(gen: &mut BiomeBuilder) {
  gen.id = biome![birch_forest_hills];
  gen.color = "#768A6A";
  gen.set_top_block(block![grass]);

  gen.place(
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
  gen.place(
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

  gen.place(
    "mossy carpet",
    PlacerStage::Sand2,
    placer::Spread {
      place:         block![rgen:mossy_carpet],
      replace:       block![grass].into(),
      radius:        4..=5,
      avg_per_chunk: 1.0,
    },
  );
  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[type = "fern"]],
    },
  );
  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[type = "tall_grass"]],
    },
  );

  gen.place(
    "forget me not",
    PlacerStage::Sand2,
    placer::Spread {
      place:         block![rgen:flower[type = "forget_me_not"]],
      replace:       block![grass].into(),
      radius:        1..=3,
      avg_per_chunk: 0.6,
    },
  );
}

pub fn aspen_wood(gen: &mut BiomeBuilder) {
  gen.id = biome![birch_forest_hills];
  gen.set_top_block(block![grass]);
  gen.color = "#B0C2A5";

  gen.place("Aspen Tree", PlacerStage::Tree, placer::AspenTree::new());

  gen.place(
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

  gen.place(
    "bushes",
    PlacerStage::Tree,
    placer::BasicDryBush {
      avg_in_chunk: 13_f64,
      leaves:       block![rgen:leaves3],
      place_above:  block![grass].into(),
      trunk:        block![log[variant = "birch"]],
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    600,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[type = "fern"]],
    },
  );
  gen.place(
    "tall grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block().into(),
      place_short:      block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[half = "lower", variant = "double_grass"]],
      place_tall_upper: block![double_plant[half = "upper"]],

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
      place_above: [block![grass]].into(),
      place:       block![tallgrass[type = "tall_grass"]],
    },
  );
}
