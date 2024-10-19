use super::{BiomeBuilder, PlacerStage};
use rgen_base::{biome, block};
use rgen_placer::placer;

#[allow(dead_code)]
pub fn plains(gen: &mut BiomeBuilder) {
  gen.id = biome![plains];
  gen.color = "#ffffff";
  gen.set_top_block(block![grass]);

  gen.place(
    "poppy",
    PlacerStage::Tree,
    placer::Clumps {
      place_above: gen.top_block().into(),
      place:       block![red_flower],

      radius:        3..=6,
      attempts:      20,
      avg_per_chunk: 0.08,
    },
  );

  gen.place(
    "dandelion",
    PlacerStage::Tree,
    placer::Clumps {
      place_above: gen.top_block().into(),
      place:       block![yellow_flower],

      radius:        2..=3,
      attempts:      20,
      avg_per_chunk: 0.08,
    },
  );

  gen.place(
    "oxeye_daisy",
    PlacerStage::Tree,
    placer::Clumps {
      place_above: gen.top_block().into(),
      place:       block![red_flower[type = "oxeye_daisy"]],

      radius:        2..=4,
      attempts:      20,
      avg_per_chunk: 0.08,
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block().into(),
      place_short:      block![tallgrass[type = "tall_grass"]],
      place_tall_lower: block![double_plant[half = "lower", variant = "double_grass"]],
      place_tall_upper: block![double_plant[half = "upper"]],

      radius:        4..=10,
      attempts:      20,
      avg_per_chunk: 3.0,
    },
  );

  gen.place(
    "bushes",
    PlacerStage::Tree,
    placer::BushClumps {
      place_above: gen.top_block().into(),
      log:         block![log],
      leaves:      block![leaves],

      radius:        10..=20,
      avg_per_chunk: 0.2,
    },
  );
}
