//! Badlands biome scaffolding. Surfaces are placeholders until each biome is designed.

use rgen_base::{biome, block};

use crate::builder::BiomeBuilder;

/// Dry, sandy scrub with low shrubs and small exposed mineral patches.
pub fn red_scrubland(g: &mut BiomeBuilder) { red_scrub_group(g, false, false, 1.0, true); }

pub fn red_scrubland_river(g: &mut BiomeBuilder) { red_scrub_river(g, 1.0, true); }

fn red_scrub_river(g: &mut BiomeBuilder, bush_rate: f64, cacti: bool) {
  red_scrub_group(g, false, true, bush_rate, cacti);
  g.place("Red Scrubland river-edge tall bushes", crate::builder::PlacerStage::Tree,
    rgen_placer::placer::TallBush {
      ground: [block![sand], block![dirt], block![grass], block![gravel], block![clay],
        block![rgen:packed_mud], block![hardened_clay], block![stained_hardened_clay]].into(),
      plantable: [block![sand], block![dirt], block![grass], block![gravel], block![rgen:packed_mud]].into(),
      plants: [block![deadbush], block![tallgrass], block![double_plant],
        block![rgen:dried_grass], block![rgen:double_tall_dried_grass]].into(),
      log: block![log[variant = "oak", axis = "y"]],
      leaves: block![leaves[variant = "oak", check_decay = false, decayable = false]],
      allow_water_under_leaves: true,
      avg_per_chunk: 3.0 * bush_rate,
    }.near_water(),
  );
  let bank_ground = [block![sand[variant = "red_sand"]], block![dirt],
    block![rgen:packed_mud], block![hardened_clay], block![stained_hardened_clay]];
  for material in [block![dirt], block![gravel]] {
    g.place_chunk(rgen_placer::chunk_placer::ShorelineSplotch {
      replace: bank_ground.into(), place: vec![material],
      radius: 1..=3, attempts: 1, avg_per_chunk: if material == block![gravel] { 9.0 } else { 4.5 },
    });
    g.place_chunk(rgen_placer::chunk_placer::WaterbedPatches {
      replace: block![sand[variant = "red_sand"]].into(), place: material,
      radius: 2..=3, depth: 1..=2, attempts: 2, avg_per_chunk: if material == block![gravel] { 4.2 } else { 2.1 },
    });
  }
  // Individual submerged deposits, preserving the water above the bed.
  for material in [block![rgen:mud], block![dirt], block![gravel]] {
    g.place_chunk(rgen_placer::chunk_placer::WaterbedPatches {
      replace: block![sand[variant = "red_sand"]].into(), place: material,
      radius: 0..=0, depth: 1..=1, attempts: 8, avg_per_chunk: if material == block![gravel] { 12.0 } else { 6.0 },
    });
  }
  // Radius zero scatters individual grass blocks, restricted to the shoreline.
  g.place_chunk(rgen_placer::chunk_placer::ShorelineSplotch {
    replace: bank_ground.into(), place: vec![block![grass]],
    radius: 0..=0, attempts: 4, avg_per_chunk: 1.5,
  });
}


pub fn arid_woodland(g: &mut BiomeBuilder) {
  red_scrub_group(g, false, false, 1.5, false);
  add_arid_woodland_trees(g);
}

pub fn arid_woodland_river(g: &mut BiomeBuilder) {
  red_scrub_river(g, 1.5, false);
  add_arid_woodland_trees(g);
}

fn add_arid_woodland_trees(g: &mut BiomeBuilder) {
  g.color = "#92965E";
  g.place_chunk(rgen_placer::chunk_placer::Splotch {
    replace: [block![sand[variant = "red_sand"]], block![dirt], block![rgen:packed_mud]].into(),
    below_replace: [block![sand], block![dirt], block![rgen:packed_mud]].into(),
    place: block![grass], radius: 2..=4, depth: 1..=1, avg_per_chunk: 1.0,
  });
  g.place("Arid Woodland basic oak trees", crate::builder::PlacerStage::Tree,
    rgen_placer::placer::BasicTree {
      place_above: [block![sand[variant = "red_sand"]], block![dirt], block![grass],
        block![rgen:packed_mud], block![hardened_clay], block![stained_hardened_clay],
        block![gravel], block![clay]].into(),
      trunk: block![log[variant = "oak", axis = "y"]],
      leaves: block![leaves[variant = "oak", check_decay = false, decayable = false]],
      avg_per_chunk: 1.3,
      trunk_height: 4..=6,
    },
  );
}

pub fn arid_hills(g: &mut BiomeBuilder) {
  arid_woodland(g);
  g.color = "#A39B72";
}

pub fn red_rock_shore(g: &mut BiomeBuilder) {
  g.id = biome![mesa];
  g.color = "#B57B64";
  g.set_top_block(block![sand[variant = "red_sand"]]);
  g.add_layer(block![red_sandstone], 3, 5);
  g.set_underwater_block(block![sand[variant = "red_sand"]]);
  g.add_underwater_layer(block![red_sandstone], 2, 4);

  for material in [
    block![stone[variant = "granite"]],
    block![stained_hardened_clay[color = "red"]],
    block![hardened_clay],
    block![stained_hardened_clay[color = "orange"]],
    block![gravel],
  ] {
    g.place_chunk(rgen_placer::chunk_placer::Splotch {
      replace: block![sand[variant = "red_sand"]].into(),
      below_replace: [block![sand[variant = "red_sand"]], block![red_sandstone]].into(),
      place: material,
      radius: if material == block![stained_hardened_clay[color = "red"]]
        || material == block![stone[variant = "granite"]] { 1..=2 } else { 2..=4 },
      depth: 1..=2, avg_per_chunk: 1.0,
    });
  }
  g.place_icing(rgen_placer::icing_placer::DriedGrassClumps {
    place_above: block![sand[variant = "red_sand"]].into(),
    radius: 2..=4, attempts: 16, avg_per_chunk: 2.5,
  }.without_green());
  g.place_icing(rgen_placer::icing_placer::Scatter {
    place_above: [block![sand[variant = "red_sand"]], block![hardened_clay], block![stained_hardened_clay]].into(),
    place: block![deadbush], avg_per_chunk: 1.0, attempts: 18,
  });
  for variant in ["a", "b", "c"] {
    g.place_icing(rgen_placer::icing_placer::Clumps {
      place_above: block![sand[variant = "red_sand"]].into(),
      place: block![rgen:dune_grass[variant = variant]],
      radius: 1..=2, attempts: 30, avg_per_chunk: 0.3 / 3.0,
    });
  }
}

pub fn red_reed_marsh(g: &mut BiomeBuilder) { red_scrub_group(g, true, false, 1.0, false); }

// First-pass D. Scrub group: shared red sediment, wetter vegetation in deltas.
fn red_scrub_group(g: &mut BiomeBuilder, marsh: bool, river: bool, bush_rate: f64, cacti: bool) {
  let ground_rate = if river { 3.0 } else { 1.0 };
  g.id = if marsh { biome![mesa] } else { biome![savanna] };
  g.color = if marsh { "#A19063" } else { "#B87555" };
  let surface = if marsh { block![rgen:packed_mud] } else { block![sand[variant = "red_sand"]] };
  g.set_top_block(surface);
  g.add_layer(if marsh { block![dirt] } else { block![sand[variant = "red_sand"]] }, 2, 3);
  g.add_layer(block![stained_hardened_clay[color = "orange"]], 4, 8);
  set_painted_mesa_waterbed(g);

  for (material, radius, rate) in [
    (block![hardened_clay], 2..=4, if marsh { 0.6 } else { 1.2 }),
    (block![gravel], 1..=2, if marsh { 0.3 } else { 0.6 }),
  ] {
    g.place_chunk(rgen_placer::chunk_placer::Splotch {
      replace: surface.into(),
      below_replace: [surface, block![dirt], block![sand], block![stained_hardened_clay]].into(),
      place: material, radius, depth: 1..=2, avg_per_chunk: rate * ground_rate,
    });
  }
  if marsh {
    g.place_chunk(rgen_placer::chunk_placer::Splotch {
      replace: surface.into(),
      below_replace: [surface, block![dirt], block![sand], block![stained_hardened_clay]].into(),
      place: block![stained_hardened_clay[color = "brown"]],
      radius: 1..=3, depth: 1..=2, avg_per_chunk: 0.35,
    });
  } else {
    // Long narrow terracotta streaks, with irregular lobes and random rotation.
    // These explicit rates also apply to the river variant.
    for (radius, rate) in [(3..=4, 0.5 * 4.0 / 9.0), (5..=6, 0.5 / 9.0), (8..=10, 0.5 * 4.0 / 9.0)] {
      g.place_chunk(rgen_placer::chunk_placer::MixedSplotch {
        replace: surface.into(),
        below_replace: [surface, block![dirt], block![sand], block![stained_hardened_clay]].into(),
        place: scrub_terracotta_palette(), radius, depth: 1..=2, avg_per_chunk: rate,
      }.elongated(0.25));
    }
  }
  // Broader pockets with packed mud mixed through the coarse dirt.
  g.place_chunk(rgen_placer::chunk_placer::MixedSplotch {
    replace: surface.into(),
    below_replace: [surface, block![dirt], block![sand], block![stained_hardened_clay]].into(),
    place: vec![block![dirt[variant = "coarse_dirt"]], block![rgen:packed_mud]],
    radius: 2..=3,
    depth: 1..=2,
    avg_per_chunk: (if marsh { 0.4 } else { 0.75 }) * ground_rate,
  });
  if marsh {
    g.place_chunk(rgen_placer::chunk_placer::Splotch {
      replace: surface.into(), below_replace: [surface, block![dirt]].into(),
      place: block![grass], radius: 1..=3, depth: 1..=2, avg_per_chunk: 1.2,
    });
    for (material, rate) in [(block![sand[variant = "red_sand"]], 1.2), (block![clay], 0.6)] {
      g.place_chunk(rgen_placer::chunk_placer::Splotch {
        replace: surface.into(), below_replace: [surface, block![dirt]].into(),
        place: material, radius: 1..=3, depth: 1..=2, avg_per_chunk: rate * ground_rate,
      });
    }
  } else {
    g.place_chunk(rgen_placer::chunk_placer::ShorelineSplotch {
      replace: [block![sand], block![dirt], block![hardened_clay], block![stained_hardened_clay]].into(),
      place: vec![block![rgen:packed_mud]],
      radius: if river { 2..=4 } else { 1..=3 },
      attempts: 1, avg_per_chunk: (if river { 3.0 } else { 1.0 }) * ground_rate,
    });
    if cacti {
      g.place("Red Scrubland cactus", crate::builder::PlacerStage::Tree, rgen_placer::placer::Cactus {
        place_above: block![sand[variant = "red_sand"]].into(),
        avg_per_chunk: 0.08, ..Default::default()
      });
      for cactus in [
        block![rgen:blue_cactus[age = 3, location = "center"]],
        block![rgen:red_cactus[age = 3]],
      ] {
        g.place_icing(rgen_placer::icing_placer::Scatter {
          place_above: block![sand[variant = "red_sand"]].into(),
          place: cactus,
          avg_per_chunk: 0.1,
          attempts: 9, // Scatter scales attempts by 1/9: 0.1 surface attempts per chunk.
        });
      }
    }
  }
  add_waterbed_patches(g, (if marsh { 1.2 } else { 0.7 }) * ground_rate, !marsh);
  add_painted_mesa_boulders(g, if marsh { 0.05 } else { 0.1 });

  let ground = [block![sand[variant = "red_sand"]], block![dirt], block![grass],
    block![rgen:packed_mud], block![hardened_clay], block![stained_hardened_clay], block![gravel], block![clay]];
  for (name, patch_name, trunk, leaves, rate) in [
    ("D. Scrub oak bushes", "D. Scrub patch oak bushes", block![log[variant = "oak", axis = "y"]],
      block![leaves[variant = "oak", check_decay = false, decayable = false]], if marsh { 0.35 } else { 0.8 }),
    ("D. Scrub acacia bushes", "D. Scrub patch acacia bushes", block![log2[variant = "acacia", axis = "y"]],
      block![leaves2[variant = "acacia", check_decay = false, decayable = false]], if marsh { 0.25 } else { 1.4 }),
  ] {
    g.place(name, crate::builder::PlacerStage::Tree, rgen_placer::placer::BasicDryBush {
      place_above: ground.into(), ground: ground.into(), trunk, leaves, avg_per_chunk: rate * bush_rate,
      plants: [block![deadbush], block![tallgrass], block![double_plant], block![rgen:dried_grass],
        block![rgen:double_tall_dried_grass]].into(),
    });
    // Extra shrubs rooted on coarse dirt or packed mud; nearby dry terrain can
    // still support the canopy, allowing bushes along the edges of patches.
    g.place(patch_name, crate::builder::PlacerStage::Tree, rgen_placer::placer::BasicDryBush {
      place_above: [block![dirt[variant = "coarse_dirt"]], block![rgen:packed_mud]].into(),
      ground: ground.into(), trunk, leaves, avg_per_chunk: rate * 2.0 * bush_rate,
      plants: [block![deadbush], block![tallgrass], block![double_plant], block![rgen:dried_grass],
        block![rgen:double_tall_dried_grass]].into(),
    });
  }
  if !marsh {
    g.place("Red Scrubland loose leaf bushes", crate::builder::PlacerStage::Tree,
      rgen_placer::placer::LeafClumps {
        place_above: ground.into(),
        leaves: block![leaves[variant = "oak", check_decay = false, decayable = false]],
        avg_per_chunk: bush_rate,
      },
    );
  }
  let dry_grass = rgen_placer::icing_placer::DriedGrassClumps {
    place_above: [block![sand[variant = "red_sand"]], block![dirt], block![grass], block![rgen:packed_mud]].into(),
    radius: 2..=4, attempts: 16, avg_per_chunk: if marsh { 2.0 } else { 2.5 },
  };
  if marsh { g.place_icing(dry_grass); } else { g.place_icing(dry_grass.without_green()); }
  g.place_icing(rgen_placer::icing_placer::GrassClumps {
    place_above: block![grass].into(),
    place_short: block![tallgrass[type = "tall_grass"]],
    place_tall_lower: block![double_plant[variant = "double_grass", half = "lower"]],
    place_tall_upper: block![double_plant[half = "upper"]],
    radius: 1..=3, attempts: 20, avg_per_chunk: if marsh { 2.0 } else { 0.4 },
  });
  g.place_icing(rgen_placer::icing_placer::Scatter {
    place_above: [block![sand], block![dirt], block![hardened_clay], block![stained_hardened_clay]].into(),
    place: block![deadbush], attempts: 18, avg_per_chunk: if marsh { 0.25 } else { 1.0 },
  });
  g.place_icing(rgen_placer::icing_placer::Reeds {
    avg_per_chunk: if marsh { 6.0 } else { 0.8 }, attempts: 64, must_be_one_block_deep: true,
  });
}

/// Terrain-following terracotta shell, world-aligned bands, then low sand cover.
pub fn painted_mesa(g: &mut BiomeBuilder) { painted_mesa_base(g, 0.07); }

pub fn painted_mesa_river(g: &mut BiomeBuilder) {
  painted_mesa_base(g, 0.6);
  set_painted_mesa_waterbed(g);
  add_painted_mesa_waterbed_patches(g, 0.7);

  // Deposits restricted to exposed ground near in-chunk water.
  for (material, rate) in [
    (block![rgen:packed_mud], 5.8),
    (block![hardened_clay], 5.8),
    (block![stained_hardened_clay[color = "brown"]], 5.8),
  ] {
    g.place_chunk(rgen_placer::chunk_placer::ShorelineSplotch {
      replace: [block![sand[variant = "red_sand"]], block![hardened_clay], block![stained_hardened_clay]].into(),
      place: vec![material],
      radius: 1..=3,
      attempts: 1,
      avg_per_chunk: rate,
    });
  }

  let river_ground = [
    block![sand[variant = "red_sand"]], block![hardened_clay],
    block![stained_hardened_clay], block![rgen:packed_mud], block![gravel],
  ];
  for (name, trunk, leaves) in [
    (
      "Painted Mesa river oak bushes",
      block![log[variant = "oak", axis = "y"]],
      block![leaves[variant = "oak", check_decay = false, decayable = false]],
    ),
    (
      "Painted Mesa river acacia bushes",
      block![log2[variant = "acacia", axis = "y"]],
      block![leaves2[variant = "acacia", check_decay = false, decayable = false]],
    ),
  ] {
    g.place(name, crate::builder::PlacerStage::Tree, rgen_placer::placer::BasicDryBush {
      avg_per_chunk: 0.01,
      place_above: river_ground.into(),
      ground: river_ground.into(),
      trunk,
      leaves,
      plants: [
        block![deadbush], block![tallgrass], block![double_plant],
        block![rgen:dried_grass], block![rgen:double_tall_dried_grass],
      ].into(),
    });
  }
}

fn painted_mesa_base(g: &mut BiomeBuilder, boulder_rate: f64) {
  g.id = biome![mesa];
  g.color = "#CF8869";
  g.set_top_block(block![hardened_clay]);
  g.add_layer(block![hardened_clay], 18, 30);
  g.set_underwater_block(block![hardened_clay]);
  g.add_underwater_layer(block![hardened_clay], 18, 30);

  // Seed-generated 64-block sequence, scaled to RGen's shorter world.
  g.place_chunk(rgen_placer::chunk_placer::TerracottaBands::short_world(g.seed, 32));

  // Applied after banding so the low orange subsurface remains solid orange.
  g.place_chunk(rgen_placer::chunk_placer::ElevationSurface {
    elevation: 63..=75,
    // Same broad OpenSimplex field as the terracotta bends (96-block scale).
    upper_cutoff: Some(rgen_placer::chunk_placer::ElevationCutoff::new(
      g.seed ^ 0x6D657361, 69..=75, 96.0,
    )),
    replace: [block![hardened_clay], block![stained_hardened_clay], block![stone]].into(),
    surface: block![sand[variant = "red_sand"]],
    subsurface: block![stained_hardened_clay[color = "orange"]],
    depth: 2..=3,
  });

  // Plain-terracotta surface deposits.
  // Radius 4..=6 gives roughly 8–12 blocks across, with an irregular outline.
  g.place_chunk(rgen_placer::chunk_placer::Splotch {
    replace: [block![hardened_clay], block![stained_hardened_clay], block![sand]].into(),
    below_replace: [block![hardened_clay], block![stained_hardened_clay], block![sand], block![stone]].into(),
    place: block![hardened_clay],
    avg_per_chunk: 1.0 / 3.0,
    radius: 4..=6,
    depth: 5..=5,
  }.starting_at_or_above(75));

  // Very rare, small gravel patches confined to the red-sand surface.
  g.place_chunk(rgen_placer::chunk_placer::Splotch {
    replace: block![sand[variant = "red_sand"]].into(),
    below_replace: block![sand[variant = "red_sand"]].into(),
    place: block![gravel],
    avg_per_chunk: 0.05,
    radius: 1..=2,
    depth: 1..=1,
  });

  add_painted_mesa_boulders(g, boulder_rate);

  g.place(
    "Painted Mesa cactus",
    crate::builder::PlacerStage::Tree,
    rgen_placer::placer::Cactus {
      place_above: block![sand[variant = "red_sand"]].into(),
      body: block![rgen:cactus],
      arms: block![rgen:cactus_arm],
      avg_per_chunk: 0.15,
      ..Default::default()
    },
  );

  // Final-pass scattering, after all ground deposits have finished.
  g.place_icing(rgen_placer::icing_placer::Scatter {
    place_above: [block![sand[variant = "red_sand"]], block![hardened_clay], block![stained_hardened_clay]].into(),
    place: block![deadbush],
    avg_per_chunk: 1.0,
    attempts: 36, // Four surface attempts.
  });

  // Sparse clumps: brown/yellow only, with random height and shape per plant.
  g.place_icing(rgen_placer::icing_placer::DriedGrassClumps {
    place_above: block![sand[variant = "red_sand"]].into(),
    radius: 2..=4,
    attempts: 12,
    avg_per_chunk: 0.5,
  }.without_green());
}

fn add_painted_mesa_boulders(g: &mut BiomeBuilder, boulder_rate: f64) {
  g.place(
    "Painted Mesa red sandstone boulders",
    crate::builder::PlacerStage::Tree,
    rgen_placer::placer::MossBoulder {
      place_above: block![sand[variant = "red_sand"]].into(),
      structure_mix: vec![
        block![red_sandstone],
        block![hardened_clay],
        block![stained_hardened_clay[color = "orange"]],
      ],
      slab_mix: vec![block![stone_slab2[variant = "red_sandstone", half = "bottom"]]],
      stair_mix: vec![block![red_sandstone_stairs[half = "bottom", shape = "straight"]]],
      footing: block![red_sandstone],
      avg_per_chunk: boulder_rate,
      ..Default::default()
    },
  );

}

/// Packed-mud islands with varied deposits and sandy channel beds.
pub fn painted_mesa_delta(g: &mut BiomeBuilder) {
  g.id = biome![mesa];
  g.color = "#A18A70";
  g.set_top_block(block![rgen:packed_mud]);
  g.add_layer(block![stained_hardened_clay[color = "orange"]], 2, 3);
  g.add_layer(block![hardened_clay], 18, 30);
  set_painted_mesa_waterbed(g);
  g.place_chunk(rgen_placer::chunk_placer::TerracottaBands::short_world(g.seed, 36));

  // Only replace the packed-mud surface, so later patches preserve earlier ones.
  for (material, radius, depth, rate) in [
    (block![dirt[variant = "coarse_dirt"]], 2..=3, 1..=2, 0.75),
    (block![grass], 1..=3, 1..=1, 0.24),
    (block![hardened_clay], 2..=4, 1..=2, 1.5),
    (block![sand[variant = "red_sand"]], 1..=3, 1..=2, 1.35),
    (block![stone[variant = "granite"]], 1..=4, 1..=2, 0.75),
    (block![gravel], 1..=3, 1..=2, 0.75),
    (block![stained_hardened_clay[color = "brown"]], 1..=3, 1..=2, 0.75),
  ] {
    g.place_chunk(rgen_placer::chunk_placer::Splotch {
      replace: block![rgen:packed_mud].into(),
      below_replace: [block![rgen:packed_mud], block![hardened_clay], block![stained_hardened_clay]].into(),
      place: material,
      radius,
      depth,
      avg_per_chunk: rate,
    });
  }

  // Restore pockets of packed mud after the contrasting surface deposits.
  let delta_surface = [
    block![rgen:packed_mud], block![dirt], block![grass], block![hardened_clay],
    block![stained_hardened_clay], block![sand[variant = "red_sand"]],
    block![stone[variant = "granite"]], block![clay], block![gravel],
  ];
  g.place_chunk(rgen_placer::chunk_placer::Splotch {
    replace: delta_surface.into(),
    below_replace: delta_surface.into(),
    place: block![rgen:packed_mud],
    radius: 2..=4,
    depth: 1..=2,
    avg_per_chunk: 1.0,
  });

  add_painted_mesa_waterbed_patches(g, 3.5);

  add_painted_mesa_boulders(g, 0.2);
  g.place(
    "Painted Mesa delta basic bushes",
    crate::builder::PlacerStage::Tree,
    rgen_placer::placer::BasicDryBush {
      avg_per_chunk: 1.3,
      place_above: delta_surface.into(),
      ground: delta_surface.into(),
      trunk: block![log[variant = "oak", axis = "y"]],
      leaves: block![leaves[variant = "oak", check_decay = false, decayable = false]],
      plants: [
        block![deadbush], block![tallgrass], block![double_plant],
        block![rgen:dried_grass], block![rgen:double_tall_dried_grass],
      ].into(),
    },
  );
  g.place(
    "Painted Mesa delta acacia bushes",
    crate::builder::PlacerStage::Tree,
    rgen_placer::placer::BasicDryBush {
      avg_per_chunk: 1.3,
      place_above: delta_surface.into(),
      ground: delta_surface.into(),
      trunk: block![log2[variant = "acacia", axis = "y"]],
      leaves: block![leaves2[variant = "acacia", check_decay = false, decayable = false]],
      plants: [
        block![deadbush], block![tallgrass], block![double_plant],
        block![rgen:dried_grass], block![rgen:double_tall_dried_grass],
      ].into(),
    },
  );
  g.place_icing(rgen_placer::icing_placer::Reeds {
    avg_per_chunk: 0.5,
    attempts: 32,
    must_be_one_block_deep: true,
  });
  g.place_icing(rgen_placer::icing_placer::Scatter {
    place_above: [block![sand[variant = "red_sand"]], block![hardened_clay], block![stained_hardened_clay], block![dirt]].into(),
    place: block![deadbush],
    avg_per_chunk: 1.0,
    attempts: 18,
  });
  g.place_icing(rgen_placer::icing_placer::DriedGrassClumps {
    place_above: [block![sand[variant = "red_sand"]], block![dirt], block![grass], block![rgen:packed_mud]].into(),
    radius: 2..=4,
    attempts: 12,
    avg_per_chunk: 2.5,
  });
  g.place_icing(rgen_placer::icing_placer::Scatter {
    place_above: [block![grass], block![dirt]].into(),
    place: block![tallgrass[type = "tall_grass"]],
    avg_per_chunk: 1.0,
    attempts: 36,
  });
  g.place_icing(rgen_placer::icing_placer::DoubleScatter {
    place_above: [block![grass], block![dirt]].into(),
    place_lower: block![double_plant[variant = "double_grass", half = "lower"]],
    place_upper: block![double_plant[half = "upper"]],
    avg_per_chunk: 0.5,
    attempts: 18,
  });
}

/// The beach keeps the normal mesa land features, with the delta's waterbed.
pub fn painted_mesa_beach(g: &mut BiomeBuilder) {
  painted_mesa_base(g, 0.07);
  set_painted_mesa_waterbed(g);
  add_painted_mesa_waterbed_patches(g, 0.7);
}

fn set_painted_mesa_waterbed(g: &mut BiomeBuilder) {
  g.underwater_layers.truncate(1);
  g.set_underwater_block(block![sand[variant = "red_sand"]]);
  g.add_underwater_layer(block![stained_hardened_clay[color = "orange"]], 2, 3);
  g.add_underwater_layer(block![hardened_clay], 18, 30);
}

fn scrub_terracotta_palette() -> Vec<rgen_base::BlockState> {
  vec![
    block![stained_hardened_clay[color = "brown"]],
    block![stained_hardened_clay[color = "white"]],
    block![stained_hardened_clay[color = "silver"]],
    block![hardened_clay],
    block![stained_hardened_clay[color = "gray"]],
  ]
}

fn add_painted_mesa_waterbed_patches(g: &mut BiomeBuilder, avg_per_chunk: f64) {
  add_waterbed_patches(g, avg_per_chunk, false);
}

fn add_waterbed_patches(g: &mut BiomeBuilder, avg_per_chunk: f64, omit_brown_terracotta: bool) {
  let mut deposits = vec![
    (block![hardened_clay], avg_per_chunk),
    (block![clay], avg_per_chunk),
    (block![dirt], avg_per_chunk),
    (block![gravel], avg_per_chunk * if omit_brown_terracotta { 2.0 } else { 1.0 }),
  ];
  // Red Scrubland's mixed terracotta deposits are land-only.
  if !omit_brown_terracotta {
    deposits.insert(1, (block![stained_hardened_clay[color = "brown"]], avg_per_chunk));
  }
  for (material, avg_per_chunk) in deposits {
    g.place_chunk(rgen_placer::chunk_placer::WaterbedPatches {
      replace: block![sand[variant = "red_sand"]].into(),
      place: material,
      radius: 2..=4,
      depth: 1..=1,
      attempts: 2,
      avg_per_chunk,
    });
  }
}
