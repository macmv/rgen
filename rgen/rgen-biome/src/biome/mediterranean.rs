use rgen_base::{biome, block, BlockFilter};
use rgen_placer::placer;

use crate::builder::PlacerStage;
use super::super::BiomeBuilder;

pub fn lebanon_cedar_woodland(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#718B62";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);
  add_cedar_woodland_standard(g, 2.0, true, (45, 50, 5), 12.0);
}

pub fn lebanon_cedar_woodland_river(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#718B62";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);
  g.set_underwater_block(block![gravel]);
  g.add_underwater_layer(block![gravel], 1, 2);
  add_cedar_woodland_standard(g, 4.0, true, (0, 50, 50), 5.0);
  add_cedar_tall_bushes(g, 4.0);

  for (name, material, avg_per_chunk) in [
    ("Cedar river ground stone", block![stone], 1.5),
    ("Cedar river ground andesite", block![stone[variant = "andesite"]], 1.5),
    ("Cedar river ground gravel", block![gravel], 2.0),
    ("Cedar river ground cobblestone", block![cobblestone], 1.5),
  ] {
    g.place(
      name,
      PlacerStage::Sand,
      placer::Splotch {
        replace: [
          block![grass],
          block![dirt],
          block![dirt[variant = "coarse_dirt"]],
          block![dirt[variant = "podzol"]],
        ]
        .into(),
        place: material,
        radius: 1..=2,
        avg_per_chunk,
      },
    );
  }

  g.place(
    "Cedar river mixed stone and gravel patches",
    PlacerStage::Sand,
    placer::MixedSplotch {
      replace: [
        block![grass],
        block![dirt],
        block![dirt[variant = "coarse_dirt"]],
        block![dirt[variant = "podzol"]],
      ]
      .into(),
      place: vec![block![gravel], block![mossy_cobblestone], block![cobblestone], block![stone[variant = "andesite"]]],
      radius: 2..=3,
      avg_per_chunk: 1.5,
    },
  );

  for material in [block![clay], block![stone[variant = "andesite"]], block![stone], block![cobblestone]] {
    g.place(
      "Cedar river bed patches",
      PlacerStage::Sand,
      placer::WaterbedPatches {
        replace: [block![gravel], block![sand], block![stone]].into(),
        place: material,
        radius: 1..=3,
        attempts: 18,
        avg_per_chunk: 0.65,
      },
    );
  }

  g.place(
    "Cedar river reeds",
    PlacerStage::Tree,
    placer::Reeds { avg_per_chunk: 5.0, attempts: 64, must_be_one_block_deep: true },
  );
}

pub fn lebanon_cedar_woodland_canyon(g: &mut BiomeBuilder) {
  lebanon_cedar_woodland(g);
}

pub fn lebanon_cedar_woodland_hills(g: &mut BiomeBuilder) {
  lebanon_cedar_woodland(g);
  add_cedar_boulders(g, 4.0);
  g.place(
    "Hill rock surface",
    PlacerStage::Sand,
    placer::RockSurface {
      avg_per_chunk: 0.8,
      size: 8,
      mix_in: vec![
        block![stone],
        block![stone[variant = "andesite"]],
        block![cobblestone],
      ],
    },
  );
}

pub fn lebanon_cedar_woodland_mountains(g: &mut BiomeBuilder) {
  lebanon_cedar_woodland(g);
  g.place(
    "Mountain rock surface",
    PlacerStage::Sand,
    placer::RockSurface {
      avg_per_chunk: 0.8,
      size: 12,
      mix_in: vec![
        block![stone],
        block![stone[variant = "andesite"]],
        block![cobblestone],
      ],
    },
  );
}

pub fn lebanon_cedar_woodland_beach(g: &mut BiomeBuilder) {
  g.id = biome![taiga];
  g.color = "#718B62";
  g.set_top_block(block![gravel]);
  g.add_layer(block![gravel], 2, 4);
  for (name, place, radius, avg_per_chunk) in [
    ("Beach coarse dirt spots", block![dirt[variant = "coarse_dirt"]], 1..=2, 2.0),
    ("Beach grass spots", block![grass], 2..=4, 3.75),
  ] {
    g.place(
      name,
      PlacerStage::Sand,
      placer::Splotch {
        replace: block![gravel].into(),
        place,
        radius,
        avg_per_chunk,
      },
    );
  }
  add_cedar_boulders(g, 4.0);

  add_cedar_trees(g, (0, 50, 50), 7.5, true);
  add_cedar_tall_bushes(g, 6.0);
  g.place(
    "Cedar beach shoreline clay",
    PlacerStage::Sand2,
    placer::ShorelineSplotch {
      replace: [block![gravel], block![grass], block![dirt[variant = "coarse_dirt"]]].into(),
      place: block![clay],
      radius: 2..=3,
      attempts: 40,
      avg_per_chunk: 1.2,
    },
  );
  g.place(
    "Beach moss carpet patches",
    PlacerStage::Sand2,
    placer::Spread {
      replace: [block![grass], block![gravel], block![clay], block![dirt[variant = "coarse_dirt"]]].into(),
      place: block![rgen:mossy_carpet],
      radius: 1..=3,
      avg_per_chunk: 1.2,
    },
  );
  g.place(
    "Beach mossy bushes",
    PlacerStage::Tree,
    placer::Scatter {
      place_above: [block![grass], block![dirt[variant = "coarse_dirt"]]].into(),
      place: block![rgen:plant],
      avg_per_chunk: 1.0,
      attempts: 80,
    },
  );
  g.place(
    "Beach ferns",
    PlacerStage::Tree,
    placer::Scatter {
      place_above: [block![grass], block![dirt[variant = "coarse_dirt"]]].into(),
      place: block![tallgrass[type = "fern"]],
      avg_per_chunk: 1.0,
      attempts: 60,
    },
  );
  g.place(
    "Beach tall grass clumps",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above: block![grass].into(),
      place_short: block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 2..=3,
      attempts: 25,
      avg_per_chunk: 1.2,
    },
  );
  g.place(
    "Rare beach green dried grass",
    PlacerStage::Tree,
    placer::Scatter {
      place_above: block![grass].into(),
      place: block![rgen:dried_grass[variant = "green_1"]],
      avg_per_chunk: 1.0,
      attempts: 15,
    },
  );
  g.place(
    "Sparse beach salt marsh grass",
    PlacerStage::Tree,
    placer::Clumps {
      place_above: [block![sand], block![grass]].into(),
      place: block![rgen:salt_marsh_grass],
      radius: 2..=3,
      attempts: 30,
      avg_per_chunk: 0.8,
    },
  );
  g.place(
    "Scattered beach salt marsh grass",
    PlacerStage::Tree,
    placer::Scatter {
      place_above: [block![grass], block![sand]].into(),
      place: block![rgen:salt_marsh_grass],
      avg_per_chunk: 1.0,
      attempts: 80,
    },
  );
  g.place(
    "Beach mossy cobblestone",
    PlacerStage::Tree,
    placer::Scatter {
      place_above: [block![grass], block![sand], block![gravel]].into(),
      place: block![rgen:mossy_cobblestone_rgen],
      avg_per_chunk: 1.5,
      attempts: 40,
    },
  );

  for (material, avg_per_chunk) in [(block![gravel], 1.4), (block![clay], 0.7), (block![stone], 0.7)] {
    g.place(
      "Cedar beach material patches",
      PlacerStage::Sand,
      placer::WaterbedPatches {
        replace: [block![sand], block![gravel], block![stone]].into(),
        place: material,
        radius: 1..=3,
        attempts: 16,
        avg_per_chunk,
      },
    );
  }
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
      ground: [block![grass], block![dirt]].into(),
      plants: [block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower]].into(),
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
    "Gravel patches under marsh channels",
    PlacerStage::Sand,
    placer::WaterbedPatches {
      replace: [block![sand], block![rgen:mud], block![clay]].into(),
      place: block![gravel],
      radius: 2..=4,
      attempts: 36,
      avg_per_chunk: 1.0,
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

  g.place(
    "Salt marsh dried grass clumps",
    PlacerStage::Tree,
    placer::DriedGrassClumps {
      place_above: [block![sand], block![grass]].into(),
      radius: 3..=5,
      attempts: 45,
      avg_per_chunk: 4.6,
    },
  );

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
      place_upper: block![double_plant[half = "upper"]],
    },
  );
}

#[derive(Clone, Copy)]
struct PlacerProfile {
  enabled: bool,
  avg_per_chunk: f64,
}

#[derive(Clone, Copy)]
struct OpenPlainsProfile {
  dry_grass_avg_per_chunk: f64,
  tall_grass_avg_per_chunk: f64,
  coarse_dirt: PlacerProfile,
  gravel_patches: PlacerProfile,
  dry_bush: PlacerProfile,
  fallen_logs: PlacerProfile,
  river_features: bool,
}

const OPEN_PLAINS_BASE: OpenPlainsProfile = OpenPlainsProfile {
  dry_grass_avg_per_chunk: 3.0,
  tall_grass_avg_per_chunk: 1.2,
  coarse_dirt: PlacerProfile { enabled: true, avg_per_chunk: 0.2 },
  gravel_patches: PlacerProfile { enabled: true, avg_per_chunk: 0.12 },
  dry_bush: PlacerProfile { enabled: true, avg_per_chunk: 0.75 },
  fallen_logs: PlacerProfile { enabled: true, avg_per_chunk: 0.15 },
  river_features: false,
};

const OPEN_PLAINS_STANDARD: OpenPlainsProfile = OPEN_PLAINS_BASE;
const OPEN_PLAINS_RIVER: OpenPlainsProfile = OpenPlainsProfile {
  dry_bush: PlacerProfile { avg_per_chunk: 3.75, ..OPEN_PLAINS_BASE.dry_bush },
  river_features: true,
  ..OPEN_PLAINS_BASE
};
const OPEN_PLAINS_CANYON: OpenPlainsProfile = OPEN_PLAINS_BASE;
const OPEN_PLAINS_HILLS: OpenPlainsProfile = OPEN_PLAINS_BASE;
const OPEN_PLAINS_MOUNTAINS: OpenPlainsProfile = OpenPlainsProfile {
  dry_bush: PlacerProfile { enabled: false, ..OPEN_PLAINS_BASE.dry_bush },
  ..OPEN_PLAINS_BASE
};

pub fn open_plains_standard(g: &mut BiomeBuilder) {
  build_open_plains(g, OPEN_PLAINS_STANDARD);
}

pub fn open_plains_river(g: &mut BiomeBuilder) {
  build_open_plains(g, OPEN_PLAINS_RIVER);
}

pub fn open_plains_canyon(g: &mut BiomeBuilder) {
  build_open_plains(g, OPEN_PLAINS_CANYON);
}

pub fn open_plains_hills(g: &mut BiomeBuilder) {
  build_open_plains(g, OPEN_PLAINS_HILLS);
}

pub fn open_plains_mountains(g: &mut BiomeBuilder) {
  build_open_plains(g, OPEN_PLAINS_MOUNTAINS);
}

fn build_open_plains(g: &mut BiomeBuilder, profile: OpenPlainsProfile) {
  g.id = biome![savanna];
  g.color = "#A9B477";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);

  if profile.river_features {
    g.set_underwater_block(block![sand]);
    g.add_underwater_layer(block![sand], 1, 2);
  }

  if profile.coarse_dirt.enabled {
    g.place(
      "Open plains coarse dirt patches",
      PlacerStage::Sand,
      placer::Splotch {
        replace: [block![grass], block![dirt]].into(),
        place: block![dirt[variant = "coarse_dirt"]],
        radius: 1..=3,
        avg_per_chunk: profile.coarse_dirt.avg_per_chunk,
      },
    );
  }

  if profile.gravel_patches.enabled {
    g.place(
      "Open plains gravel patches",
      PlacerStage::Sand,
      placer::Splotch {
        replace: [block![grass], block![dirt]].into(),
        place: block![gravel],
        radius: 1..=3,
        avg_per_chunk: profile.gravel_patches.avg_per_chunk,
      },
    );
  }

  add_open_plains_dry_grass(g, profile.dry_grass_avg_per_chunk);

  g.place(
    "Open plains tall grass clumps",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above: [block![grass]].into(),
      place_short: block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[2]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 4..=8,
      attempts: 110,
      avg_per_chunk: profile.tall_grass_avg_per_chunk,
    },
  );

  if profile.dry_bush.enabled {
    g.place(
      "Open plains dry bushes",
      PlacerStage::Tree,
      placer::BasicDryBush {
        avg_per_chunk: profile.dry_bush.avg_per_chunk,
        leaves: block![leaves[variant = "oak", check_decay = false, decayable = true]],
        place_above: block![grass].into(),
        ground: [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]]].into(),
        plants: [
          block![tallgrass],
          block![double_plant],
          block![yellow_flower],
          block![red_flower],
          block![rgen:dried_grass],
          block![rgen:double_tall_dried_grass],
        ]
        .into(),
        trunk: block![log[variant = "oak"]],
      },
    );
  }

  if profile.fallen_logs.enabled {
    g.place(
      "Open plains fallen logs",
      PlacerStage::Tree,
      placer::LongLog {
        avg_per_chunk: profile.fallen_logs.avg_per_chunk,
        ground: [block![grass], block![dirt]].into(),
        log: block![log[variant = "oak"]],
      },
    );
  }

  if profile.river_features {
    g.place(
      "Open plains yellow wildflower shrubs",
      PlacerStage::Tree,
      placer::Scatter {
        avg_per_chunk: 0.9,
        attempts: 120,
        place_above: [block![grass]].into(),
        place: block![rgen:yellow_wildflower_shrub],
      },
    );

    g.place(
      "Open plains dandelions",
      PlacerStage::Tree,
      placer::Scatter {
        avg_per_chunk: 0.8,
        attempts: 120,
        place_above: [block![grass]].into(),
        place: block![yellow_flower[type = "dandelion"]],
      },
    );

    g.place(
      "Rare open plains oxeye daisies",
      PlacerStage::Tree,
      placer::Scatter {
        avg_per_chunk: 0.15,
        attempts: 30,
        place_above: [block![grass]].into(),
        place: block![red_flower[type = "oxeye_daisy"]],
      },
    );

    g.place(
      "Open plains river gravel patches",
      PlacerStage::Sand,
      placer::WaterbedPatches {
        replace: block![sand].into(),
        place: block![gravel],
        radius: 1..=3,
        attempts: 12,
        avg_per_chunk: 0.6,
      },
    );

    g.place(
      "Open plains river clay patches",
      PlacerStage::Sand,
      placer::WaterbedPatches {
        replace: block![sand].into(),
        place: block![clay],
        radius: 1..=3,
        attempts: 10,
        avg_per_chunk: 0.4,
      },
    );
  }
}

fn add_cedar_boulders(g: &mut BiomeBuilder, avg_per_chunk: f64) {
  g.place(
    "Cedar woodland boulders",
    PlacerStage::Tree,
    placer::MossBoulder { avg_per_chunk, ..placer::MossBoulder::default() },
  );
}

fn add_cedar_woodland_standard(g: &mut BiomeBuilder, boulder_rate: f64, trees: bool, tree_mix: (u8, u8, u8), tree_rate: f64) {
  // Common coarse dirt and gravel, with less frequent podzol pockets.
  for (name, place, radius, avg_per_chunk) in [
    ("Frequent coarse dirt patches", block![dirt[variant = "coarse_dirt"]], 2..=3, 8.0),
    ("Gravel patches", block![gravel], 1..=2, 5.0),
    ("Podzol patches", block![dirt[variant = "podzol"]], 2..=5, 0.8),
  ] {
    g.place(
      name,
      PlacerStage::Sand,
      placer::Splotch {
        replace: [block![grass], block![dirt]].into(),
        place,
        radius,
        avg_per_chunk,
      },
    );
  }

  add_cedar_boulders(g, boulder_rate);

  g.place(
    "Dried grass clumps",
    PlacerStage::Tree,
    placer::DriedGrassClumps {
      place_above: [block![grass]].into(),
      radius: 2..=4,
      attempts: 36,
      avg_per_chunk: 1.5,
    },
  );
  g.place(
    "Grass clumps",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above: [block![grass]].into(),
      place_short: block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 3..=6,
      attempts: 50,
      avg_per_chunk: 3.0,
    },
  );
  g.place(
    "Fern clumps",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above: [block![grass]].into(),
      place_short: block![tallgrass[type = "fern"]],
      place_tall_lower: block![double_plant[variant = "double_fern", half = "lower"]],
      place_tall_upper: block![double_plant[half = "upper"]],
      radius: 3..=6,
      attempts: 40,
      avg_per_chunk: 2.0,
    },
  );
  g.place(
    "Brown mushrooms",
    PlacerStage::Tree,
    placer::Scatter {
      place_above: [block![grass], block![dirt], block![dirt[variant = "podzol"]]].into(),
      place: block![brown_mushroom],
      avg_per_chunk: 0.8,
      attempts: 40,
    },
  );

  g.place(
    "Fallen cedar logs",
    PlacerStage::Tree,
    placer::LogAndStump {
      log: block![rgen:log[variant = "cedar"]],
      moss_log: block![rgen:covered_cedar_log[axis = "none"]],
      ground: block![grass],
      plants: [block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower]].into(),
      avg_per_chunk: 0.45,
      chance_of_moss: 5,
      is_shrooms: true,
      shroom: block![rgen:polypore[type = "one", facing = "north"]],
    },
  );

  if trees {
    add_cedar_trees(g, tree_mix, tree_rate, false);
  }
}

fn add_cedar_trees(g: &mut BiomeBuilder, tree_mix: (u8, u8, u8), avg_per_chunk: f64, grow_on_gravel: bool) {
  let mut growable_mix: BlockFilter = [
    block![grass],
    block![dirt],
    block![dirt[variant = "coarse_dirt"]],
    block![dirt[variant = "podzol"]],
  ]
  .into();
  if grow_on_gravel {
    growable_mix = [
      block![grass],
      block![dirt],
      block![dirt[variant = "coarse_dirt"]],
      block![dirt[variant = "podzol"]],
      block![gravel],
    ]
    .into();
  }
  g.place(
    "Cedar trees",
    PlacerStage::Tree,
    placer::CedarTree {
      ground_mix: [
        block![grass],
        block![dirt],
        block![dirt[variant = "coarse_dirt"]],
        block![dirt[variant = "podzol"]],
        block![gravel],
      ]
      .into(),
      growable_mix,
      plant_mix: [
        block![tallgrass],
        block![double_plant],
        block![red_flower],
        block![yellow_flower],
        block![brown_mushroom],
        block![rgen:dried_grass],
        block![rgen:double_tall_dried_grass],
      ]
      .into(),
      trunk: block![rgen:log[variant = "cedar"]],
      leaves: block![rgen:leaves[variant = "cedar", check_decay = false, decayable = true]],
      large_chance: tree_mix.0,
      medium_chance: tree_mix.1,
      small_chance: tree_mix.2,
      avg_per_chunk,
    },
  );
}

fn add_cedar_tall_bushes(g: &mut BiomeBuilder, avg_per_chunk: f64) {
  g.place(
    "Cedar tall bushes",
    PlacerStage::Tree,
    placer::TallBush {
      ground: [
        block![grass],
        block![dirt],
        block![dirt[variant = "coarse_dirt"]],
        block![dirt[variant = "podzol"]],
        block![gravel],
        block![clay],
        block![stone],
        block![stone[variant = "andesite"]],
        block![cobblestone],
      ]
      .into(),
      plantable: [
        block![grass],
        block![dirt],
        block![dirt[variant = "coarse_dirt"]],
        block![dirt[variant = "podzol"]],
        block![gravel],
      ]
      .into(),
      plants: [
        block![tallgrass],
        block![double_plant],
        block![red_flower],
        block![yellow_flower],
        block![brown_mushroom],
        block![rgen:dried_grass],
        block![rgen:double_tall_dried_grass],
      ]
      .into(),
      log: block![rgen:log[axis = "y", variant = "cedar"]],
      leaves: block![rgen:leaves[variant = "cedar", check_decay = false, decayable = true]],
      avg_per_chunk,
    },
  );
}

fn add_open_plains_dry_grass(g: &mut BiomeBuilder, avg_per_chunk: f64) {
  g.place(
    "Open plains dried grass clumps",
    PlacerStage::Tree,
    placer::DriedGrassClumps {
      place_above: [block![grass]].into(),
      radius: 1..=2,
      attempts: 25,
      avg_per_chunk,
    },
  );
}
