use rgen_base::{biome, block};
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

pub fn salt_marsh(g: &mut BiomeBuilder) {
  g.id = biome![savanna];
  g.color = "#A99A67";
  g.set_top_block(block![grass]);
  g.set_underwater_block(block![sand]);
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
