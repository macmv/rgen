use rgen_base::{biome, block};
use rgen_placer::placer;

use crate::{builder::PlacerStage, BiomeBuilder};

#[allow(dead_code)]
pub fn deep_jungle(gen: &mut BiomeBuilder) {
  gen.id = biome![jungle];
  gen.color = "#E0705F";
  gen.set_top_block(block![grass]);
  gen.add_layer(block![dirt], 5, 8);

  gen.place("Large Jungle Tree", PlacerStage::Tree, placer::JungleTree::new());

  gen.place(
    "Jungle bushes",
    PlacerStage::Tree,
    placer::BushClumps {
      place_above:   [block![grass]].into(),
      log:           block![log[3]],
      leaves:        block![leaves[3]],
      avg_per_chunk: 6.0,
      radius:        3..=5,
    },
  );

  gen.place(
    "Grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block().into(),
      place_short:      block![tallgrass[1]],     // Grass
      place_tall_lower: block![double_plant[2]],  // Tall grass lower
      place_tall_upper: block![double_plant[10]], // Tall grass upper

      radius:        4..=10,
      attempts:      50,
      avg_per_chunk: 8.0,
    },
  );
}
