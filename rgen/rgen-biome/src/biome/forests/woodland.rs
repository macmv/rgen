use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::super::BiomeBuilder;

pub fn woodland_river(gen: &mut BiomeBuilder) {
  gen.id = biome![birch_forest_hills];
  gen.color = "#899781";
  gen.set_top_block(block![grass]);

  gen.place("Oak tree", PlacerStage::Tree, placer::OakTree::new());

  gen.place(
    "oak log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            block![log[0]],
      moss_log:       block![rgen:mossy_stump[0]],
      ground:         block![grass],
      plants:         block![stone].into(),
      avg_per_chunk:  0.5,
      chance_of_moss: 8,
      is_shrooms:     true,
      shroom:         block![rgen:polypore],
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[2]],
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[1]],
    },
  );
}

pub fn woodland(gen: &mut BiomeBuilder) {
  gen.id = biome![birch_forest_hills];
  gen.color = "#899781";
  gen.set_top_block(block![grass]);

  gen.place("Oak tree", PlacerStage::Tree, placer::OakTree::new());

  gen.place(
    "oak log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            block![log[0]],
      moss_log:       block![rgen:mossy_stump[0]],
      ground:         block![grass],
      plants:         block![stone].into(),
      avg_per_chunk:  0.5,
      chance_of_moss: 8,
      is_shrooms:     true,
      shroom:         block![rgen:polypore],
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[2]],
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[1]],
    },
  );
}

#[allow(dead_code)]
pub fn windswept_woodland(gen: &mut BiomeBuilder) {
  gen.id = biome![birch_forest_hills];
  gen.color = "#899781";
  gen.set_top_block(block![grass]);

  gen.place("Oak tree", PlacerStage::Tree, placer::OakTree::new());

  gen.place(
    "oak log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            block![log[0]],
      moss_log:       block![rgen:mossy_stump[0]],
      ground:         block![grass],
      plants:         block![stone].into(),
      avg_per_chunk:  0.5,
      chance_of_moss: 8,
      is_shrooms:     true,
      shroom:         block![rgen:polypore],
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[2]],
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    160,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[1]],
    },
  );
}
