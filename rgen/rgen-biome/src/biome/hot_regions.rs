#![allow(dead_code)]
use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::BiomeBuilder;

pub fn flat_desert(gen: &mut BiomeBuilder) {
  gen.id = biome![desert];
  gen.color = "#E0705F";
  gen.set_top_block(block![sand]);
  gen.add_layer(block![sandstone], 5, 8);

  gen.place(
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

pub fn lush_desert(gen: &mut BiomeBuilder) {
  gen.id = biome![desert];
  gen.color = "#D14A3F";
  gen.set_top_block(block![sand]);
  gen.add_layer(block![sandstone], 5, 8);

  gen.place(
    "trees",
    PlacerStage::Tree,
    placer::BasicDryBush {
      place_above:  [block![sand]].into(),
      trunk:        block![log],
      leaves:       block![leaves],
      avg_in_chunk: 1.0,
    },
  );

  gen.place(
    "Large Cactus",
    PlacerStage::Tree,
    placer::Cactus {
      avg_in_chunk: 1_f64,
      arms:         block![rgen:cactus_arm],
      place_above:  block![sand].into(),
      body:         block![rgen:cactus],
    },
  );

  gen.place(
    "cactus blue",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    30,
      place_above: [block![sand]].into(),
      place:       block![rgen:cactus[1]],
    },
  );

  gen.place(
    "cactus red",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    20,
      place_above: [block![sand]].into(),
      place:       block![rgen:cactus[3]],
    },
  );
}

pub fn bad_lands(gen: &mut BiomeBuilder) {
  gen.id = biome![mesa];
  gen.color = "#C74538";
  gen.set_top_block(block![hardened_clay]);
}

pub fn dune_sea(gen: &mut BiomeBuilder) {
  gen.id = biome![desert];
  gen.color = "#EA7468";
  gen.set_top_block(block![sand]);
}
