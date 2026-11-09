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
      place_upper: block![double_plant[10]],
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
  dry_bush: PlacerProfile { enabled: true, avg_per_chunk: 0.3 },
  fallen_logs: PlacerProfile { enabled: true, avg_per_chunk: 0.15 },
  river_features: false,
};

const OPEN_PLAINS_STANDARD: OpenPlainsProfile = OPEN_PLAINS_BASE;
const OPEN_PLAINS_RIVER: OpenPlainsProfile = OpenPlainsProfile {
  dry_bush: PlacerProfile { avg_per_chunk: 0.75, ..OPEN_PLAINS_BASE.dry_bush },
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
      place_tall_upper: block![double_plant[10]],
      radius: 4..=8,
      attempts: 60,
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
        attempts: 120,
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

fn add_open_plains_dry_grass(g: &mut BiomeBuilder, avg_per_chunk: f64) {
  g.place(
    "Open plains dried grass clumps",
    PlacerStage::Tree,
    placer::DriedGrassClumps {
      place_above: [block![grass]].into(),
      radius: 2..=4,
      attempts: 24,
      avg_per_chunk,
    },
  );
}
