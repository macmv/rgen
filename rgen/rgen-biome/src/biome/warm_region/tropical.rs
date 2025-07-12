use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::{BiomeBuilder, builder::PlacerStage};
// big_canopy_jungle
pub fn deep_canopy_jungle_wood(g: &mut BiomeBuilder) {
  g.id = biome![jungle];
  g.color = "#E0705F"; //variant: ["dirt", "coarse_dirt", "podzol"],
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 5, 8);

  g.place("Large Jungle Tree", PlacerStage::Tree, placer::BetterBush::default());

}

// light_jungle_wood
pub fn light_jungle_wood(g: &mut BiomeBuilder) {
  g.id = biome![jungle];
  g.color = "#E0705F";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 5, 8);
    

    g.place(
    "basic jungle",
    PlacerStage::Tree,
    placer::BasicJungle {
      trunk:         block![log[variant = "jungle"]],
      leaves:        block![leaves[variant = "jungle"]],
      avg_per_chunk: 7.0,
      is_cocoa:      true,
      shroom:        block![cocoa],
      ground:        block![grass],
      vine:          block![vine],
    },
  );

  g.place(
    "Jungle bushes",
    PlacerStage::Tree,
    placer::BushClumps {
      place_above:   [block![grass]].into(),
      log:           block![log[variant = "jungle"]],
      leaves:        block![leaves[variant = "jungle"]],
      avg_per_chunk: 6.0,
      radius:        3..=5,
    },
  );

    g.place(
    "jungle log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            block![log[variant = "jungle"]],
      moss_log:       block![rgen:covered_jungle_log],
      ground:         block![grass],
      plants:         block![tallgrass[type="tall_grass"]].into(),
      avg_per_chunk:  0.75,
      chance_of_moss: 5,
      is_shrooms:     false,
      shroom:         block![rgen:polypore],
    },
  );

  g.place(
    "Grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      g.top_block().into(),
      place_short:      block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[half = "lower", variant = "double_grass"]],
      place_tall_upper: block![double_plant[half = "upper"]],

      radius:        4..=10,
      attempts:      50,
      avg_per_chunk: 8.0,
    },
  );
}

// terraced_jungle_wood

pub fn terraced_jungle_wood(g: &mut BiomeBuilder) {
  g.id = biome![jungle];
  g.color = "#E0705F";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 5, 8);

  g.place("Large Jungle Tree", PlacerStage::Tree, placer::TerraceJungleTree::default());

  g.place(
    "Jungle bushes",
    PlacerStage::Tree,
    placer::BushClumps {
      place_above:   [block![grass]].into(),
      log:           block![log[variant = "jungle"]],
      leaves:        block![leaves[variant = "jungle"]],
      avg_per_chunk: 6.0,
      radius:        3..=5,
    },
  );

  g.place(
    "Grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      g.top_block().into(),
      place_short:      block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[half = "lower", variant = "double_grass"]],
      place_tall_upper: block![double_plant[half = "upper"]],

      radius:        4..=10,
      attempts:      50,
      avg_per_chunk: 8.0,
    },
  );
}
