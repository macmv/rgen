//! Badlands biome scaffolding. Surfaces are placeholders until each biome is designed.

use rgen_base::{biome, block};

use crate::builder::BiomeBuilder;

fn badlands_placeholder(g: &mut BiomeBuilder) {
  g.id = biome![mesa];
  g.color = "#CF8869";
  g.set_top_block(block![sand[variant = "red_sand"]]);
  g.add_layer(block![hardened_clay], 3, 5);
  g.set_underwater_block(block![sand[variant = "red_sand"]]);
}

pub fn red_scrubland(g: &mut BiomeBuilder) {
  badlands_placeholder(g);
  g.color = "#B87555";
}

pub fn arid_woodland(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#92965E";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);
}

pub fn arid_hills(g: &mut BiomeBuilder) {
  arid_woodland(g);
  g.color = "#A39B72";
}

pub fn red_rock_shore(g: &mut BiomeBuilder) {
  g.id = biome![mesa];
  g.color = "#B57B64";
  g.set_top_block(block![red_sandstone]);
  g.add_layer(block![red_sandstone], 3, 5);
  g.set_underwater_block(block![red_sandstone]);
}

pub fn red_reed_marsh(g: &mut BiomeBuilder) {
  badlands_placeholder(g);
  g.color = "#A19063";
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
      place: material,
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
      body: block![rgen:cactus[color = "green"]],
      arms: block![rgen:cactus_arm],
      avg_per_chunk: 0.15,
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

fn add_painted_mesa_waterbed_patches(g: &mut BiomeBuilder, avg_per_chunk: f64) {
  for material in [
    block![hardened_clay],
    block![stained_hardened_clay[color = "brown"]],
    block![clay],
    block![dirt],
    block![gravel],
  ] {
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
