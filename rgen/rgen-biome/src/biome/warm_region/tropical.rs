use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::{BiomeBuilder, builder::PlacerStage};
// Canopied Jungle
pub fn canopied_jungle(g: &mut BiomeBuilder) {
  g.id = biome![jungle];
  g.color = "#E0705F"; //variant: ["dirt", "coarse_dirt", "podzol"],
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 5, 8);

  g.place("Jungle Bush", PlacerStage::Tree, placer::BetterBush::default());
  g.place("Small Jungle Tree", PlacerStage::Tree, placer::BasicJungle::default());
  g.place("Large Jungle Tree", PlacerStage::Tree, placer::WideCanopyJungle::default());
  g.place("Jungle Floor", PlacerStage::Tree, placer::JungleFloorPlace::default());
}

pub fn flower_canopied_jungle(g: &mut BiomeBuilder) {
  g.id = biome![jungle];
  g.color = "#E0705F"; //variant: ["dirt", "coarse_dirt", "podzol"],
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 5, 8);

  g.place("Jungle Bush", PlacerStage::Tree, placer::BetterBush::default());
  g.place("Large Jungle Tree", PlacerStage::Tree, placer::WideCanopyJungle::default());
  g.place("Small Jungle Tree", PlacerStage::Tree, placer::BasicJungle::default());
  g.place(
    "Jungle Flower Floor",
    PlacerStage::Tree,
    placer::JungleFloorPlace {
      attempts:           2,
      place_above:        [
        block![grass],
        block![dirt],
        block![rgen:mossy_cobblestone_rgen],
        block![rgen:covered_jungle_log],
      ]
      .into(),
      is_large:           false,
      place:              block![rgen:lavender_plant],
      pink_orchid:        block![rgen:pink_orchid],
      passion_flower:     block![rgen:passion_flower],
      heliconia:          block![rgen:heliconia],
      pink_heart:         block![rgen:pink_heart],
      torch_ginger:       block![rgen:torch_ginger],
      orchidaceae:        block![rgen:orchidaceae],
      ipomoea:            block![rgen:ipomoea],
      bromeliads:         block![rgen:bromeliads],
      ficus_elastica:     block![rgen:ficus_elastica],
      yellow_jungle_rose: block![rgen:yellow_jungle_rose],
      bird_of_paradise:   block![rgen:bird_of_paradise],
      jungle_bush:        block![rgen:jungle_bush],
      grass:              block![tallgrass],
      tall_grass:         block![double_plant],
      is_flower_floor:    true,
    },
  );
}

// light_jungle_wood
pub fn light_jungle_wood(g: &mut BiomeBuilder) {
  g.id = biome![jungle];
  g.color = "#E0705F";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 5, 8);

  g.place("Small Jungle Tree", PlacerStage::Tree, placer::BasicJungle::default());
  g.place("Jungle Bush", PlacerStage::Tree, placer::BetterBush::default());
  g.place("Jungle Floor", PlacerStage::Tree, placer::JungleFloorPlace::default());

  g.place(
    "jungle log",
    PlacerStage::Tree,
    placer::LogAndStump {
      log:            block![log[variant = "jungle"]],
      moss_log:       block![rgen:covered_jungle_log],
      ground:         block![grass],
      plants:         block![tallgrass[type="tall_grass"]].into(),
      avg_per_chunk:  2.0,
      chance_of_moss: 5,
      is_shrooms:     false,
      shroom:         block![rgen:polypore],
    },
  );
}
//jungle_beach
//jungle_rocks


// terraced_jungle_wood

pub fn terraced_jungle(g: &mut BiomeBuilder) {
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
}
//flowered_terraced_jungle