use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::super::BiomeBuilder;

pub fn woodland_river(g: &mut BiomeBuilder) {
  g.id = biome![birch_forest_hills];
  g.color = "#899781";
  g.set_top_block(block![grass]);

  g.place("Oak tree", PlacerStage::Tree, placer::OakTree::default());

  g.place(
    "oak log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            block![log[variant = "oak"]],
      moss_log:       block![rgen:mossy_stump[variant = "oak"]],
      ground:         block![grass],
      plants:         block![stone].into(),
      avg_per_chunk:  0.5,
      chance_of_moss: 8,
      is_shrooms:     true,
      shroom:         block![rgen:polypore],
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

pub fn woodland(g: &mut BiomeBuilder) {
  g.id = biome![birch_forest_hills];
  g.color = "#899781";
  g.set_top_block(block![grass]);

  g.place("Oak tree", PlacerStage::Tree, placer::OakTree::default());

  g.place(
    "oak log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            block![log[variant = "oak"]],
      moss_log:       block![rgen:mossy_stump[variant = "oak"]],
      ground:         block![grass],
      plants:         block![stone].into(),
      avg_per_chunk:  0.5,
      chance_of_moss: 8,
      is_shrooms:     true,
      shroom:         block![rgen:polypore],
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

#[allow(dead_code)]
pub fn windswept_woodland(g: &mut BiomeBuilder) {
  g.id = biome![birch_forest_hills];
  g.color = "#899781";
  g.set_top_block(block![grass]);

  g.place("Oak tree", PlacerStage::Tree, placer::OakTree::default());

  g.place(
    "oak log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            block![log[variant = "oak"]],
      moss_log:       block![rgen:mossy_stump[variant = "oak"]],
      ground:         block![grass],
      plants:         block![stone].into(),
      avg_per_chunk:  0.5,
      chance_of_moss: 8,
      is_shrooms:     true,
      shroom:         block![rgen:polypore],
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
