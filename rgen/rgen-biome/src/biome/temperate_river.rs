#![allow(dead_code)]

use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::BiomeBuilder;

pub fn cherry_blossom_river(gen: &mut BiomeBuilder) {
  println!("hey neil, hey neil, how you doing?");
  gen.id = biome![birch_forest_hills];
  gen.color = "#A3B5A0";
  gen.set_top_block(block![grass]);

  gen.place("Small Cherry Tree", PlacerStage::Tree, placer::Sakura::new());
  gen.place(
    "sprinkling of bamboo",
    PlacerStage::Sand,
    placer::BambooClump {
      attempts:      10,
      avg_per_chunk: 3.0,
      place_above:   gen.top_block().into(),
      radius:        1..=4,
      bamboo:        placer::Bamboo {
        place_above:  gen.top_block().into(),
        stalk:        block![rgen:bamboo],
        pint_size:    true,
        avg_in_chunk: 0.0,
      },
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    800,
      place_above: [block![grass]].into(),
      place:       block![tallgrass[1]],
    },
  );

  gen.place(
    "tall grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block().into(),
      place_short:      block![tallgrass[1]],     // Grass
      place_tall_lower: block![double_plant[2]],  // Tall grass lower
      place_tall_upper: block![double_plant[10]], // Tall grass upper

      radius:        4..=10,
      attempts:      100,
      avg_per_chunk: 3.0,
    },
  );

  gen.place(
    "Lilac",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block().into(),
      place_short:      block![tallgrass[1]],     // Grass
      place_tall_lower: block![double_plant[1]],  // lilac bottom
      place_tall_upper: block![double_plant[10]], // double plant top

      radius:        4..=10,
      attempts:      40,
      avg_per_chunk: 3.0,
    },
  );
}

pub fn lavender_river(gen: &mut BiomeBuilder) {
  gen.id = biome![birch_forest_hills];
  gen.color = "#899781";
  gen.set_top_block(block![grass]);

  gen.place(
    "SmallLavenderScatter",
    PlacerStage::Tree,
    placer::LavenderScatter {
      attempts:    900,
      place_above: [block![grass]].into(),
      is_large:    false,
      place:       block![rgen:lavender_plant],
    },
  );
  gen.place(
    "LargeLavenderScatter",
    PlacerStage::Tree,
    placer::LavenderScatter {
      attempts:    600,
      place_above: [block![grass]].into(),
      is_large:    true,
      place:       block![rgen:double_tall_lavender_plant],
    },
  );
}
pub fn volcano_river(gen: &mut BiomeBuilder) {
  gen.id = biome![plains];
  gen.color = "#899781";
  gen.set_top_block(block![grass]);
  gen.add_layer(block![rgen:basalt], 5, 8);
  gen.add_underwater_layer(block![rgen:basalt], 3, 4);

  gen.place("Basalt Pillar", PlacerStage::Tree, placer::Pillar::new());
  gen.place(
    "basalt_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         block![rgen:basalt[0]],
      radius:        2..=4,
      avg_per_chunk: 1.0,
    },
  );
  gen.place(
    "sprinkling of bamboo",
    PlacerStage::Sand,
    placer::BambooClump {
      attempts:      1,
      avg_per_chunk: 1.0,
      place_above:   gen.top_block().into(),
      radius:        1..=2,
      bamboo:        placer::Bamboo {
        place_above:  gen.top_block().into(),
        stalk:        block![rgen:bamboo],
        pint_size:    true,
        avg_in_chunk: 0.0,
      },
    },
  );
  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    800,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[1]],
    },
  );

  gen.place(
    "tall grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block().into(),
      place_short:      block![tallgrass[1]],     // Grass
      place_tall_lower: block![double_plant[2]],  // Tall grass lower
      place_tall_upper: block![double_plant[10]], // Tall grass upper

      radius:        4..=10,
      attempts:      200,
      avg_per_chunk: 3.0,
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    80,
      place_above: [block![grass], block![rgen:mossy_stump]].into(),
      place:       block![tallgrass[2]],
    },
  );
}
//Field, Volcano growth
