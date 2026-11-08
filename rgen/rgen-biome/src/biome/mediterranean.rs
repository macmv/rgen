use rgen_base::{biome, block, BlockFilter};
use rgen_placer::placer;

use crate::builder::PlacerStage;
use super::super::BiomeBuilder;

pub fn redwood_grove(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#718B62";
  g.set_top_block(block![grass]);
  g.place("Redwood grove trees", PlacerStage::Tree, placer::OakTree::default());
}

pub fn riparian_woodland(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#779C83";
  g.set_top_block(block![grass]);
  g.set_underwater_block(block![clay]);
  g.place("Riparian woodland trees", PlacerStage::Tree, placer::OakTree::default());
}

pub fn chaparral_flats(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#A9A06B";
  g.set_top_block(block![grass]);
  g.place(
    "Chaparral shrubs",
    PlacerStage::Tree,
    placer::BasicDryBush {
      avg_per_chunk: 5.0,
      leaves: block![leaves[variant = "oak"]],
      place_above: block![grass].into(),
      trunk: block![log[variant = "oak"]],
    },
  );
}

pub fn coastal_dunes(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#D8C783";
  g.set_top_block(block![sand]);
  g.set_underwater_block(block![sand]);
}

/// Blocks considered Salt Marsh plants, shared by vegetation placers.
pub fn salt_marsh_plants() -> BlockFilter {
  [
    block![tallgrass],
    block![double_plant],
    block![red_flower],
    block![yellow_flower],
    block![rgen:dried_grass],
    block![rgen:double_tall_dried_grass],
  ]
  .into()
}

pub fn salt_marsh(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#A99A67";
  g.set_top_block(block![grass]);
  g.set_underwater_block(block![sand]);

  g.place(
    "Tall marsh bushes",
    PlacerStage::Tree,
    placer::TallBush {
      ground: [block![grass], block![dirt], block![gravel], block![rgen:mud]].into(),
      plantable: [block![grass], block![dirt], block![gravel]].into(),
      plants: salt_marsh_plants(),
      log: block![log[axis = "y", variant = "oak"]],
      leaves: block![leaves[variant = "oak", check_decay = false, decayable = true]],
      avg_per_chunk: 4.25,
    },
  );

  g.place(
    "Fallen oak logs",
    PlacerStage::Tree,
    placer::LongLog {
      avg_per_chunk: 0.6,
      ground: block![grass].into(),
      log: block![log[variant = "oak"]],
    },
  );
  g.place(
    "Mud patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: block![grass].into(),
      place: block![rgen:mud],
      radius: 2..=5,
      avg_per_chunk: 1.2,
    },
  );
  g.place(
    "Gravel patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: block![grass].into(),
      place: block![gravel],
      radius: 2..=5,
      avg_per_chunk: 1.5,
    },
  );
  g.place(
    "Rare sand patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: block![grass].into(),
      place: block![sand],
      radius: 1..=3,
      avg_per_chunk: 0.08,
    },
  );

  g.place(
    "Brown dried grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above: block![grass].into(),
      place_short: block![rgen:dried_grass[variant = "brown_1"]],
      place_tall_lower: block![rgen:double_tall_dried_grass[half = "lower", variant = "brown_1"]],
      place_tall_upper: block![rgen:double_tall_dried_grass[half = "upper", variant = "brown_1"]],
      radius: 4..=9,
      attempts: 45,
      avg_per_chunk: 1.0,
    },
  );
  g.place(
    "Yellow dried grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above: block![grass].into(),
      place_short: block![rgen:dried_grass[variant = "yellow_1"]],
      place_tall_lower: block![rgen:double_tall_dried_grass[half = "lower", variant = "yellow_1"]],
      place_tall_upper: block![rgen:double_tall_dried_grass[half = "upper", variant = "yellow_1"]],
      radius: 4..=9,
      attempts: 45,
      avg_per_chunk: 0.8,
    },
  );
  g.place(
    "Green dried grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above: block![grass].into(),
      place_short: block![rgen:dried_grass[variant = "green_1"]],
      place_tall_lower: block![rgen:double_tall_dried_grass[half = "lower", variant = "green_1"]],
      place_tall_upper: block![rgen:double_tall_dried_grass[half = "upper", variant = "green_1"]],
      radius: 4..=9,
      attempts: 45,
      avg_per_chunk: 0.5,
    },
  );

  g.place(
    "Salt marsh grasses",
    PlacerStage::Tree,
    placer::Scatter {
      avg_per_chunk: 1.0,
      attempts: 320,
      place_above: [block![grass], block![dirt]].into(),
      place: block![tallgrass[type = "tall_grass"]],
    },
  );
}

pub fn open_plains(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#A9B477";
  g.set_top_block(block![grass]);
  g.place(
    "Open plains grass",
    PlacerStage::Tree,
    placer::Scatter {
      avg_per_chunk: 1.0,
      attempts: 180,
      place_above: [block![grass]].into(),
      place: block![tallgrass[type = "tall_grass"]],
    },
  );
}
