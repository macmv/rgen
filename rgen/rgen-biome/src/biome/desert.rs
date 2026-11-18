//! Hot desert biomes: open dunes, seasonal green corridors, salt flats, and dry mountains.

use rgen_base::{biome, block};
use rgen_placer::{chunk_placer, icing_placer, placer};

use crate::{builder::{BiomeBuilder, PlacerStage}};

pub fn open_dune(g: &mut BiomeBuilder) {
  build_open_dune(g);
}

pub fn open_dune_river(g: &mut BiomeBuilder) {
  build_open_dune(g);
  for (count, rate) in [(2, 0.125), (3, 0.125)] {
    g.place(
      if count == 2 { "Open Dune River small dune grass clumps" } else { "Open Dune River larger dune grass clumps" },
      PlacerStage::Tree,
      placer::Clumps {
        place_above: block![sand].into(),
        place: block![rgen:dune_grass[variant = "a"]],
        radius: 1..=2,
        attempts: count,
        avg_per_chunk: rate,
      },
    );
  }
}

pub fn open_dune_delta(g: &mut BiomeBuilder) {
  build_open_dune(g);
}

/// Green Desert standard biome.
pub fn lush_desert(g: &mut BiomeBuilder) {
  build_lush_desert(g, 0.8, 0.8, false);
}

pub fn lush_desert_canyon(g: &mut BiomeBuilder) {
  build_lush_desert(g, 0.8, 0.8, false);
}

/// The Salt Desert standard assignment remains an Open Dune.
pub fn open_dune_salt_transition(g: &mut BiomeBuilder) {
  build_open_dune(g);
}

pub fn open_dune_canyon(g: &mut BiomeBuilder) {
  build_open_dune(g);
}

pub fn open_dune_beach(g: &mut BiomeBuilder) {
  build_open_dune(g);
}

fn build_open_dune(g: &mut BiomeBuilder) {
  g.id = biome![desert];
  g.color = "#D9C47D";
  g.set_top_block(block![sand]);
  g.layers[0].min_depth = 4;
  g.layers[0].max_depth = 7;
  // The transition block counts toward the following layer's depth.
  g.add_layer(block![sandstone], 2, 5);
  g.set_underwater_block(block![sand]);
  g.underwater_layers[0].min_depth = 4;
  g.underwater_layers[0].max_depth = 7;
  g.add_underwater_layer(block![sandstone], 2, 5);

  // Very rare isolated short grass blades on the open sand.
  g.place(
    "Open Dune single short grass",
    PlacerStage::Tree,
    placer::Scatter {
      place_above: block![sand].into(),
      place: block![rgen:short_grass],
      avg_per_chunk: 0.6,
      attempts: 1,
    },
  );
  g.place(
    "Open Dune single dead bush",
    PlacerStage::Tree,
    placer::Scatter {
      place_above: block![sand].into(),
      place: block![deadbush],
      avg_per_chunk: 0.08,
      attempts: 1,
    },
  );

  // Rare mature green column cacti, including their arms.
  g.place(
    "Open Dune mature green cacti",
    PlacerStage::Tree,
    placer::Cactus {
      place_above: block![sand].into(),
      body: block![rgen:cactus[color = "green"]],
      arms: block![rgen:cactus_arm],
      avg_per_chunk: 0.06,
      ..Default::default()
    },
  );
}

pub fn lush_desert_river(g: &mut BiomeBuilder) {
  build_lush_desert(g, 1.25, 1.4, false);
}

pub fn lush_desert_beach(g: &mut BiomeBuilder) {
  build_lush_desert(g, 0.45, 0.3, false);
}

pub fn lush_desert_delta(g: &mut BiomeBuilder) {
  build_lush_desert(g, 2.0, 2.0, true);
}

fn build_lush_desert(g: &mut BiomeBuilder, bush_rate: f64, water_feature_rate: f64, reeds: bool) {
  g.id = biome![desert];
  g.color = "#A9A66D";
  g.set_top_block(block![sand]);
  g.add_layer(block![sand], 2, 4);
  g.add_layer(block![sandstone], 3, 5);
  g.set_underwater_block(block![sand]);
  g.add_underwater_layer(block![gravel], 1, 2);

  g.place_chunk(chunk_placer::Splotch {
    replace: block![sand].into(),
    below_replace: [block![sand], block![rgen:packed_mud]].into(),
    place: block![rgen:packed_mud],
    radius: 2..=3,
    depth: 1..=2,
    avg_per_chunk: bush_rate * 0.5,
  });

  // Mature multi-block green cacti, with a higher chance of pink flowers.
  g.place(
    "Lush Desert mature green cacti",
    PlacerStage::Tree,
    placer::Cactus {
      place_above: block![sand].into(),
      body: block![rgen:cactus[color = "green"]],
      arms: block![rgen:cactus_arm],
      avg_per_chunk: 0.3,
      arm_flower_chance: 2,
      trunk_flower_chance: 2,
    },
  );

  // Green ground follows exposed channels rather than covering the desert floor.
  g.place_chunk(chunk_placer::ShorelineSplotch {
    replace: [block![sand], block![dirt], block![gravel]].into(),
    place: block![grass],
    radius: 1..=3,
    attempts: 2,
    avg_per_chunk: water_feature_rate,
  });
  g.place_chunk(chunk_placer::ShorelineSplotch {
    replace: [block![sand], block![dirt], block![grass]].into(),
    place: block![rgen:packed_mud],
    radius: 1..=3,
    attempts: 2,
    avg_per_chunk: water_feature_rate * 0.65,
  });
  g.place_chunk(chunk_placer::WaterbedPatches {
    replace: [block![sand], block![rgen:mud], block![clay]].into(),
    place: block![gravel],
    radius: 1..=3,
    depth: 1..=2,
    attempts: 18,
    avg_per_chunk: water_feature_rate * 0.5,
  });

  g.place(
    "Lush Desert waterline shrubs",
    PlacerStage::Tree,
    placer::TallBush {
      ground: [block![sand], block![grass], block![dirt], block![gravel], block![rgen:packed_mud]].into(),
      plantable: [block![sand], block![grass], block![dirt], block![rgen:packed_mud]].into(),
      plants: [block![deadbush], block![tallgrass], block![double_plant], block![rgen:dried_grass]].into(),
      log: block![log2[variant = "acacia", axis = "y"]],
      leaves: block![leaves2[variant = "acacia", check_decay = false, decayable = false]],
      allow_water_under_leaves: true,
      avg_per_chunk: bush_rate,
    }.near_water(),
  );
  g.place_icing(icing_placer::GrassClumps {
    place_above: [block![grass], block![rgen:packed_mud]].into(),
    place_short: block![tallgrass[type = "tall_grass"]],
    place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
    place_tall_upper: block![double_plant[half = "upper"]],
    radius: 2..=4,
    attempts: 48,
    avg_per_chunk: bush_rate * 0.6,
  });
  g.place_icing(icing_placer::DriedGrassClumps {
    place_above: [block![sand], block![grass]].into(),
    radius: 2..=4,
    attempts: 32,
    avg_per_chunk: bush_rate * 0.35,
  });
  if reeds {
    g.place_icing(icing_placer::Reeds {
      avg_per_chunk: 1.5,
      attempts: 64,
      must_be_one_block_deep: true,
    });
  }

  // Same-color cactus patches on exposed sand.
  for cactus in [
    block![rgen:cactus[color = "blue"]],
    block![rgen:cactus[color = "yellow"]],
    // The red cactus uses the legacy orange block-state name.
    block![rgen:cactus[color = "orange"]],
  ] {
    g.place_icing(icing_placer::Clumps {
      place_above: block![sand].into(),
      place: cactus,
      radius: 1..=2,
      attempts: 18,
      avg_per_chunk: 0.1,
    });
  }
}

pub fn salt_flats_beach(g: &mut BiomeBuilder) {
  build_salt_flats(g, 0.06);
  g.place_chunk(chunk_placer::Splotch {
    replace: block![rgen:salt_on_sand].into(),
    below_replace: [block![rgen:salt_on_sand], block![sand], block![sandstone]].into(),
    place: block![rgen:salt],
    radius: 2..=3,
    depth: 2..=2,
    avg_per_chunk: 0.3,
  }.below_height(65));
}

pub fn salt_flats_standard(g: &mut BiomeBuilder) {
  build_salt_flats(g, 0.06);
}

pub fn salt_flats_river(g: &mut BiomeBuilder) {
  build_salt_flats(g, 0.1);
}

pub fn salt_flats_canyon(g: &mut BiomeBuilder) {
  build_salt_flats(g, 0.06);
}

pub fn salt_flats_delta(g: &mut BiomeBuilder) {
  build_salt_flats(g, 0.16);
}

fn build_salt_flats(g: &mut BiomeBuilder, brine_rate: f64) {
  g.id = biome![desert];
  g.color = "#D8D4B3";
  g.set_top_block(block![sand]);
  g.add_layer(block![sand], 2, 4);
  g.add_layer(block![sandstone], 3, 5);
  g.set_underwater_block(block![sand]);
  g.add_underwater_layer(block![clay], 1, 3);

  // Salt crust and salt deposits stay at sea level and below.
  g.place_chunk(chunk_placer::ElevationSurface {
    elevation: 1..=64,
    upper_cutoff: None,
    replace: block![sand].into(),
    surface: block![rgen:salt_on_sand],
    subsurface: block![sand],
    depth: 0..=0,
  });

  // Salt deposits in shallow, irregular patches.
  for (material, radius, rate) in [
    (block![rgen:salt], 2..=4, 0.55),
  ] {
    g.place_chunk(chunk_placer::Splotch {
      replace: [block![rgen:salt_on_sand], block![sand]].into(),
      below_replace: [block![rgen:salt_on_sand], block![sand], block![sandstone], block![clay]].into(),
      place: material,
      radius,
      depth: 1..=2,
      avg_per_chunk: rate,
    }.below_height(65));
  }
  // Rare one-block pools read as shallow brine basins on otherwise dry flats.
  g.place_chunk(chunk_placer::Splotch {
    replace: block![rgen:salt_on_sand].into(),
    below_replace: [block![rgen:salt_on_sand], block![sand], block![clay]].into(),
    place: block![water],
    radius: 1..=2,
    depth: 1..=1,
    avg_per_chunk: brine_rate,
  }.below_height(64));
}

pub fn desert_mountain(g: &mut BiomeBuilder) {
  build_desert_mountain(g);
}

pub fn desert_mountain_scrub(g: &mut BiomeBuilder) {
  build_desert_mountain(g);
}

pub fn desert_mountain_salt(g: &mut BiomeBuilder) {
  build_desert_mountain(g);
}

pub fn desert_mountain_hills(g: &mut BiomeBuilder) {
  build_desert_mountain(g);
}

pub fn desert_mountain_hills_salt(g: &mut BiomeBuilder) {
  build_desert_mountain(g);
}

fn build_desert_mountain(g: &mut BiomeBuilder) {
  g.id = biome![desert];
  g.color = "#A99B78";
  g.set_top_block(block![sand]);
  g.layers[0].min_depth = 1;
  g.layers[0].max_depth = 2;
  g.add_layer(block![sandstone], 7, 11);
  g.set_underwater_block(block![sand]);
  g.underwater_layers[0].min_depth = 1;
  g.underwater_layers[0].max_depth = 2;
  g.add_underwater_layer(block![sandstone], 7, 11);
}
