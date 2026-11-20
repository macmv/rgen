//! Savanna climate biomes.

use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::builder::{BiomeBuilder, PlacerStage};

/// The first savanna biome: grassy ground with scattered acacia trees.
pub fn wooded_savanna(g: &mut BiomeBuilder) {
  build_wooded_savanna(g, 6.0, 3.0);
}

fn build_wooded_savanna(g: &mut BiomeBuilder, trees_per_chunk: f64, tall_bushes_per_chunk: f64) {
  g.id = biome![savanna];
  g.color = "#A9A06B";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);

  for patch in [block![gravel], block![dirt[variant = "coarse_dirt"]]] {
    g.place_chunk(rgen_placer::chunk_placer::Splotch {
      replace: block![grass].into(),
      below_replace: [block![grass], block![dirt], block![gravel], block![dirt[variant = "coarse_dirt"]]].into(),
      place: patch,
      avg_per_chunk: 0.45,
      radius: 1..=2,
      depth: 1..=2,
    });
  }

  add_savanna_vegetation(g, tall_bushes_per_chunk);
  add_acacia_trees(g, trees_per_chunk, crate::feature::ACACIA_GRASS_PANTS);

  g.place(
    "Wooded savanna boulders",
    PlacerStage::Tree,
    placer::MossBoulder {
      place_above: [block![grass], block![dirt]].into(),
      avg_per_chunk: 0.1,
      ..placer::MossBoulder::default()
    },
  );
}

fn add_savanna_vegetation(g: &mut BiomeBuilder, tall_bushes_per_chunk: f64) {
  g.place(
    "Acacia tall bushes",
    PlacerStage::Tree,
    placer::TallBush {
      ground: [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]]].into(),
      plantable: [block![grass], block![dirt]].into(),
      plants: [block![tallgrass], block![double_plant], block![rgen:dried_grass], block![rgen:double_tall_dried_grass]].into(),
      log: block![log2[variant = "acacia", axis = "y"]],
      leaves: block![leaves2[variant = "acacia", check_decay = false, decayable = true]],
      allow_water_under_leaves: false,
      avg_per_chunk: tall_bushes_per_chunk,
    },
  );

  // Single and double tall grass in the same clumps.
  g.place_icing(rgen_placer::icing_placer::GrassClumps {
    place_above: block![grass].into(),
    place_short: block![tallgrass[type = "tall_grass"]],
    place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
    place_tall_upper: block![double_plant[half = "upper"]],
    radius: 4..=8,
    attempts: 110,
    avg_per_chunk: 1.2,
  });

  // Fern clumps include both single-height and double-height ferns.
  g.place_icing(rgen_placer::icing_placer::GrassClumps {
    place_above: block![grass].into(),
    place_short: block![tallgrass[type = "fern"]],
    place_tall_lower: block![double_plant[variant = "double_fern", half = "lower"]],
    place_tall_upper: block![double_plant[half = "upper"]],
    radius: 3..=6,
    attempts: 80,
    avg_per_chunk: 0.6,
  });

  g.place_icing(rgen_placer::icing_placer::DriedGrassClumps {
    place_above: block![grass].into(),
    radius: 1..=2,
    attempts: 25,
    avg_per_chunk: 3.0,
  });
}

fn add_acacia_trees(g: &mut BiomeBuilder, avg_per_chunk: f64, grass_pants: bool) {
  g.place(
    "Acacia trees",
    PlacerStage::Tree,
    placer::AcaciaTree {
      avg_per_chunk,
      grass_pants,
      ..placer::AcaciaTree::default()
    },
  );
}

/// River variant used by the wooded savanna group.
pub fn riparian_savanna_woodland(g: &mut BiomeBuilder) {
  build_wooded_savanna(g, 1.0, 0.1);
  g.color = "#7788C8";
  g.set_underwater_block(block![dirt]);
  g.place(
    "Riparian savanna basic acacia bushes",
    PlacerStage::Tree,
    placer::BasicDryBush {
      place_above: [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]]].into(),
      ground: [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]]].into(),
      plants: [block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower], block![rgen:dried_grass]].into(),
      trunk: block![log2[variant = "acacia", axis = "y"]],
      leaves: block![leaves2[variant = "acacia", check_decay = false, decayable = false]],
      avg_per_chunk: 0.2,
    },
  );
  for (material, avg_per_chunk) in [
    (block![rgen:mud], 2.0),
    (block![gravel], 1.5),
    (block![sand], 0.8),
  ] {
    g.place_chunk(rgen_placer::chunk_placer::ShorelineSplotch {
      replace: [block![grass], block![dirt], block![sand], block![gravel]].into(),
      place: material,
      radius: 1..=3,
      attempts: 2,
      avg_per_chunk,
    });
  }
  for (material, avg_per_chunk) in [(block![gravel], 1.0), (block![sand], 0.5)] {
    g.place_chunk(rgen_placer::chunk_placer::WaterbedPatches {
      replace: [block![dirt], block![rgen:mud], block![gravel], block![sand], block![clay]].into(),
      place: material,
      radius: 1..=3,
      depth: 1..=2,
      attempts: 16,
      avg_per_chunk,
    });
  }
}

/// Grassy, sparsely vegetated savanna scrub.
pub fn savanna_scrub(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#C6A77D";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);

  for patch in [block![dirt[variant = "coarse_dirt"]], block![gravel]] {
    g.place_chunk(rgen_placer::chunk_placer::Splotch {
      replace: block![grass].into(),
      below_replace: [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]], block![gravel]].into(),
      place: patch,
      avg_per_chunk: 0.6,
      radius: 1..=2,
      depth: 1..=2,
    });
  }

  // Small exposed-rock patches mixed from stone, gravel, and andesite.
  g.place_chunk(rgen_placer::chunk_placer::MixedSplotch {
    replace: block![grass].into(),
    below_replace: [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]], block![gravel]].into(),
    place: vec![block![stone], block![gravel], block![stone[variant = "andesite"]]],
    avg_per_chunk: 0.25,
    radius: 1..=2,
    depth: 1..=1,
  });

  let scrub_ground: rgen_base::BlockFilter = [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]]].into();
  let scrub_plants: rgen_base::BlockFilter = [block![tallgrass], block![double_plant], block![red_flower], block![yellow_flower]].into();
  g.place(
    "Savanna scrub basic acacia bushes",
    PlacerStage::Tree,
    placer::BasicDryBush {
      place_above: scrub_ground.clone(),
      ground: scrub_ground.clone(),
      plants: scrub_plants.clone(),
      trunk: block![log2[variant = "acacia", axis = "y"]],
      leaves: block![leaves2[variant = "acacia", check_decay = false, decayable = false]],
      avg_per_chunk: 0.25,
    },
  );
  g.place(
    "Savanna scrub tall acacia bushes",
    PlacerStage::Tree,
    placer::TallBush {
      ground: scrub_ground,
      plantable: [block![grass], block![dirt]].into(),
      plants: scrub_plants,
      log: block![log2[variant = "acacia", axis = "y"]],
      leaves: block![leaves2[variant = "acacia", check_decay = false, decayable = false]],
      allow_water_under_leaves: false,
      avg_per_chunk: 0.25,
    },
  );
  g.place(
    "Savanna scrub rock boulders",
    PlacerStage::Tree,
    placer::MossBoulder {
      place_above: [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]], block![gravel]].into(),
      structure_mix: vec![block![stone], block![stone[variant = "andesite"]], block![cobblestone]],
      slab_mix: vec![block![stone_slab[variant = "cobblestone", half = "bottom"]]],
      stair_mix: vec![block![stone_stairs[half = "bottom", shape = "straight"]]],
      footing: block![stone],
      avg_per_chunk: 0.05,
      ..placer::MossBoulder::default()
    },
  );
  add_acacia_trees(g, 0.25, crate::feature::ACACIA_GRASS_PANTS);
  g.place(
    "Savanna scrub leaf bushes",
    PlacerStage::Tree,
    placer::LeafClumps {
      place_above: [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]]].into(),
      leaves: block![leaves2[variant = "acacia", check_decay = false, decayable = false]],
      avg_per_chunk: 0.8,
    },
  );
  g.place_icing(rgen_placer::icing_placer::GrassClumps {
    place_above: block![grass].into(),
    place_short: block![tallgrass[type = "tall_grass"]],
    place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
    place_tall_upper: block![double_plant[half = "upper"]],
    radius: 4..=8,
    attempts: 110,
    avg_per_chunk: 1.2,
  });
  g.place_icing(rgen_placer::icing_placer::DriedGrassClumps {
    place_above: block![grass].into(),
    radius: 1..=2,
    attempts: 25,
    avg_per_chunk: 0.6,
  });
  g.place_icing(rgen_placer::icing_placer::GrassClumps {
    place_above: block![grass].into(),
    place_short: block![tallgrass[type = "fern"]],
    place_tall_lower: block![double_plant[variant = "double_fern", half = "lower"]],
    place_tall_upper: block![double_plant[half = "upper"]],
    radius: 3..=6,
    attempts: 80,
    avg_per_chunk: 0.2,
  });
}

/// Open grassy savanna with a dirt subsoil.
pub fn open_savanna(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#D9B983";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);
  add_acacia_trees(g, 0.2, crate::feature::ACACIA_GRASS_PANTS);
  g.place_icing(rgen_placer::icing_placer::GrassClumps {
    place_above: block![grass].into(),
    place_short: block![tallgrass[type = "tall_grass"]],
    place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
    place_tall_upper: block![double_plant[half = "upper"]],
    radius: 3..=6,
    attempts: 80,
    avg_per_chunk: 0.8,
  });
}

/// Exposed stone and grass for steep savanna hills and mountains.
pub fn rocky_scrubland(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#D99A91";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 1, 2);
  g.add_layer(block![stone], 5, 8);
  g.place_chunk(rgen_placer::chunk_placer::RockSurface {
    avg_per_chunk: 0.8,
    size: 12,
    mix_in: vec![block![stone], block![stone[variant = "andesite"]], block![cobblestone]],
  });
  add_savanna_vegetation(g, 3.0);
  add_acacia_trees(g, 0.05, false);
  g.place(
    "Rocky scrubland creeping leaves",
    PlacerStage::Tree,
    placer::CreepingLeaves {
      place_above: block![grass].into(),
      leaves: block![leaves[variant = "oak", check_decay = false, decayable = false]],
      avg_per_chunk: 2.0,
    },
  );
}

/// Sandy savanna coast.
pub fn savanna_dunes(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#D8C783";
  g.set_top_block(block![sand]);
  g.add_layer(block![sand], 2, 3);
  g.add_layer(block![sandstone], 3, 5);
  g.set_underwater_block(block![sand]);
  g.place(
    "Savanna dune grass clumps",
    PlacerStage::Tree,
    placer::Clumps {
      place_above: block![sand].into(),
      place: block![rgen:dune_grass[variant = "a"]],
      radius: 1..=2,
      attempts: 2,
      avg_per_chunk: 0.8,
    },
  );
}

/// Flooded grassland with muddy banks, shallow-bed deposits, and reeds.
pub fn flooded_savanna(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#B878BA";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 2, 4);
  g.set_underwater_block(block![dirt]);

  // Muddy patches break up the grassy floodplain surface.
  g.place_chunk(rgen_placer::chunk_placer::Splotch {
    replace: block![grass].into(),
    below_replace: [block![grass], block![dirt], block![sand], block![gravel], block![rgen:mud]].into(),
    place: block![rgen:mud],
    avg_per_chunk: 1.2,
    radius: 2..=4,
    depth: 1..=2,
  });
  g.place_chunk(rgen_placer::chunk_placer::Splotch {
    replace: block![grass].into(),
    below_replace: [block![grass], block![dirt], block![dirt[variant = "coarse_dirt"]], block![rgen:mud]].into(),
    place: block![dirt[variant = "coarse_dirt"]],
    avg_per_chunk: 0.6,
    radius: 1..=2,
    depth: 1..=1,
  });

  // Mud, gravel, and sand form irregular strips along exposed water edges.
  for (material, avg_per_chunk) in [
    (block![rgen:mud], 2.0),
    (block![gravel], 0.8),
    (block![sand], 0.5),
  ] {
    g.place_chunk(rgen_placer::chunk_placer::ShorelineSplotch {
      replace: [block![grass], block![dirt], block![sand], block![gravel], block![rgen:mud]].into(),
      place: material,
      radius: 1..=3,
      attempts: 2,
      avg_per_chunk,
    });
  }

  // Add occasional mud and gravel deposits to the mostly dirt channel beds.
  for (material, avg_per_chunk) in [(block![rgen:mud], 0.35), (block![gravel], 0.2)] {
    g.place_chunk(rgen_placer::chunk_placer::WaterbedPatches {
      replace: [block![dirt], block![rgen:mud], block![clay]].into(),
      place: material,
      radius: 1..=3,
      depth: 1..=2,
      attempts: 16,
      avg_per_chunk,
    });
  }

  g.place_icing(rgen_placer::icing_placer::Reeds {
    avg_per_chunk: 2.0,
    attempts: 64,
    must_be_one_block_deep: true,
  });
  g.place_icing(rgen_placer::icing_placer::GrassClumps {
    place_above: [block![grass], block![rgen:covered_acacia_log]].into(),
    place_short: block![tallgrass[type = "tall_grass"]],
    place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
    place_tall_upper: block![double_plant[half = "upper"]],
    radius: 3..=6,
    attempts: 80,
    avg_per_chunk: 4.0,
  });

  g.place(
    "Flooded savanna acacia trees",
    PlacerStage::Tree,
    placer::FloodedAcaciaTree {
      avg_per_chunk: 0.8,
      ..placer::FloodedAcaciaTree::default()
    },
  );
  g.place(
    "Flooded savanna full acacia trees",
    PlacerStage::Tree,
    placer::AcaciaTree {
      avg_per_chunk: 0.2,
      grass_pants: crate::feature::ACACIA_GRASS_PANTS,
      ..placer::AcaciaTree::default()
    },
  );
  g.place(
    "Flooded savanna Acacia stumps",
    PlacerStage::Tree,
    placer::AcaciaStump {
      place_above: [block![grass], block![dirt], block![rgen:mud]].into(),
      avg_per_chunk: 2.7,
    },
  );
  g.place(
    "Flooded savanna bank bushes",
    PlacerStage::Tree,
    placer::TallBush {
      ground: [block![grass], block![dirt], block![rgen:mud]].into(),
      plantable: [block![grass], block![dirt]].into(),
      plants: [block![tallgrass], block![double_plant], block![rgen:dried_grass]].into(),
      log: block![log2[variant = "acacia", axis = "y"]],
      leaves: block![leaves2[variant = "acacia", check_decay = false, decayable = true]],
      allow_water_under_leaves: true,
      avg_per_chunk: 0.2,
    }.near_water(),
  );
  g.place(
    "Flooded savanna basic acacia bushes",
    PlacerStage::Tree,
    placer::BasicDryBush {
      place_above: [block![grass], block![dirt], block![rgen:mud]].into(),
      ground: [block![grass], block![dirt], block![rgen:mud]].into(),
      plants: [block![tallgrass], block![double_plant], block![rgen:dried_grass]].into(),
      trunk: block![log2[variant = "acacia", axis = "y"]],
      leaves: block![leaves2[variant = "acacia", check_decay = false, decayable = false]],
      avg_per_chunk: 0.8,
    },
  );
  g.place(
    "Flooded savanna leaf clumps",
    PlacerStage::Tree,
    placer::LeafClumps {
      place_above: [block![grass], block![dirt], block![rgen:mud]].into(),
      leaves: block![leaves2[variant = "acacia", check_decay = false, decayable = false]],
      avg_per_chunk: 1.2,
    },
  );
}
