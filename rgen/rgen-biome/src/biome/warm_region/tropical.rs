use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::{BiomeBuilder, builder::PlacerStage};

pub fn deep_jungle(g: &mut BiomeBuilder) {
  g.id = biome![jungle];
  g.color = "#E0705F";
  g.set_top_block(block![grass]);
  g.add_layer(block![dirt], 5, 8);

  g.place("Large Jungle Tree", PlacerStage::Tree, placer::JungleTree::default());

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
