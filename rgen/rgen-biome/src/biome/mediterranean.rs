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
    block![rgen:glasswort],
    block![rgen:salt_marsh_grass],
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
      avg_per_chunk: 0.5,
    },
  );

  g.place(
    "Glasswort",
    PlacerStage::Tree,
    placer::Glasswort {
      avg_per_chunk: 1.0,
      attempts: 320,
    },
  );

  g.place(
    "Reeds",
    PlacerStage::Tree,
    placer::Reeds {
      avg_per_chunk: 6.0,
      attempts: 64,
      must_be_one_block_deep: true,
    },
  );

  // Keep the clump placer, spreading each color's density across every short
  // variant. The tall block has variants _1 and _2, so _3 clumps reuse _1.
  for (name, short, tall, avg_per_chunk) in [
    ("Brown dried grass 1", block![rgen:dried_grass[variant = "brown_1"]], "brown_1", 2.0 / 3.0),
    ("Brown dried grass 2", block![rgen:dried_grass[variant = "brown_2"]], "brown_2", 2.0 / 3.0),
    ("Brown dried grass 3", block![rgen:dried_grass[variant = "brown_3"]], "brown_1", 2.0 / 3.0),
    ("Yellow dried grass 1", block![rgen:dried_grass[variant = "yellow_1"]], "yellow_1", 1.6 / 3.0),
    ("Yellow dried grass 2", block![rgen:dried_grass[variant = "yellow_2"]], "yellow_2", 1.6 / 3.0),
    ("Yellow dried grass 3", block![rgen:dried_grass[variant = "yellow_3"]], "yellow_1", 1.6 / 3.0),
    ("Green dried grass 1", block![rgen:dried_grass[variant = "green_1"]], "green_1", 1.0 / 3.0),
    ("Green dried grass 2", block![rgen:dried_grass[variant = "green_2"]], "green_2", 1.0 / 3.0),
    ("Green dried grass 3", block![rgen:dried_grass[variant = "green_3"]], "green_1", 1.0 / 3.0),
  ] {
    g.place(
      name,
      PlacerStage::Tree,
      placer::GrassClumps {
        place_above: [block![sand], block![grass]].into(),
        place_short: short,
        place_tall_lower: block![rgen:double_tall_dried_grass[half = "lower", variant = tall]],
        place_tall_upper: block![rgen:double_tall_dried_grass[half = "upper", variant = tall]],
        radius: 3..=5,
        attempts: 45,
        avg_per_chunk,
      },
    );
  }

  g.place(
    "Salt marsh grass clumps",
    PlacerStage::Tree,
    placer::Clumps {
      place_above: [block![sand], block![grass]].into(),
      place: block![rgen:salt_marsh_grass],
      radius: 2..=5,
      attempts: 45,
      avg_per_chunk: 2.0,
    },
  );

  g.place(
    "Salt marsh grasses",
    PlacerStage::Tree,
    placer::Scatter {
      avg_per_chunk: 1.0,
      attempts: 320,
      place_above: [block![grass]].into(),
      place: block![tallgrass[type = "tall_grass"]],
    },
  );
  g.place(
    "Double tall grass",
    PlacerStage::Tree,
    placer::DoubleScatter {
      avg_per_chunk: 0.2,
      attempts: 320,
      place_above: [block![grass]].into(),
      // Vanilla metadata 2 is lower double-grass; 10 is upper.
      place_lower: block![double_plant[2]],
      place_upper: block![double_plant[10]],
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
