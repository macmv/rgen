#![allow(dead_code)]
use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::BiomeBuilder;

pub fn flat_desert(g: &mut BiomeBuilder) {
  g.id = biome![desert];
  g.color = "#E0705F";
  g.set_top_block(block![sand]);
  g.add_layer(block![sandstone], 5, 8);

  g.place(
    "Large Cactus",
    PlacerStage::Tree,
    placer::Cactus {
      avg_in_chunk: 0.5_f64,
      arms:         block![rgen:cactus_arm],
      place_above:  block![sand].into(),
      body:         block![rgen:cactus],
    },
  );
}

pub fn lush_desert(g: &mut BiomeBuilder) {
  g.id = biome![desert];
  g.color = "#D14A3F";
  g.set_top_block(block![sand]);
  g.add_layer(block![sandstone], 5, 8);

  g.place(
    "trees",
    PlacerStage::Tree,
    placer::BasicDryBush {
      place_above:  [block![sand]].into(),
      trunk:        block![log],
      leaves:       block![leaves],
      avg_in_chunk: 1.0,
    },
  );

  g.place(
    "Large Cactus",
    PlacerStage::Tree,
    placer::Cactus {
      avg_in_chunk: 1_f64,
      arms:         block![rgen:cactus_arm],
      place_above:  block![sand].into(),
      body:         block![rgen:cactus],
    },
  );

  g.place(
    "cactus blue",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    30,
      place_above: [block![sand]].into(),
      place:       block![rgen:cactus[color = "blue"]],
    },
  );

  g.place(
    "cactus red",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    20,
      place_above: [block![sand]].into(),
      place:       block![rgen:cactus[color = "orange"]],
    },
  );
}

pub fn bad_lands(g: &mut BiomeBuilder) {
  g.id = biome![mesa];
  g.color = "#C74538";
  g.set_top_block(block![hardened_clay]);
}

pub fn dune_sea(g: &mut BiomeBuilder) {
  g.id = biome![desert];
  g.color = "#EA7468";
  g.set_top_block(block![sand]);
}
