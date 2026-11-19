//! Savanna climate biomes.

use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::builder::{BiomeBuilder, PlacerStage};

/// The first savanna biome: grassy ground with scattered acacia trees.
pub fn wooded_savanna(g: &mut BiomeBuilder) {
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

  add_savanna_vegetation(g);
  add_acacia_trees(g, 6.0, crate::feature::ACACIA_GRASS_PANTS);

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

fn add_savanna_vegetation(g: &mut BiomeBuilder) {
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
      avg_per_chunk: 3.0,
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
  wooded_savanna(g);
  g.color = "#7788C8";
  g.set_underwater_block(block![sand]);
}

/// Grassy, sparsely vegetated savanna scrub.
pub fn savanna_scrub(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#C6A77D";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);
}

/// Open grassy savanna with a dirt subsoil.
pub fn open_savanna(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#D9B983";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);
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
  add_savanna_vegetation(g);
  add_acacia_trees(g, 0.05, false);
}

/// Sandy savanna coast.
pub fn savanna_dunes(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#D8C783";
  g.set_top_block(block![sand]);
  g.add_layer(block![sand], 2, 3);
  g.add_layer(block![sandstone], 3, 5);
  g.set_underwater_block(block![sand]);
}

/// Wet savanna delta surface.
pub fn flooded_savanna(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#B878BA";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 3, 5);
  g.set_underwater_block(block![sand]);
}
