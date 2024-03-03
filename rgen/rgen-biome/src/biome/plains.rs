use super::{BiomeBuilder, IdContext, PlacerStage};
use rgen_placer::placer;

pub fn plains(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.grass.default_state;

  gen.place(
    "poppy",
    PlacerStage::Tree,
    placer::Clumps {
      place_above: gen.top_block.into(),
      place:       ctx.blocks.red_flower.default_state,

      radius:        3..=6,
      attempts:      20,
      avg_per_chunk: 0.08,
    },
  );

  gen.place(
    "dandelion",
    PlacerStage::Tree,
    placer::Clumps {
      place_above: gen.top_block.into(),
      place:       ctx.blocks.yellow_flower.default_state,

      radius:        2..=3,
      attempts:      20,
      avg_per_chunk: 0.08,
    },
  );

  gen.place(
    "oxeye_daisy",
    PlacerStage::Tree,
    placer::Clumps {
      place_above: gen.top_block.into(),
      place:       ctx.blocks.red_flower.with_data(8),

      radius:        2..=4,
      attempts:      20,
      avg_per_chunk: 0.08,
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block.into(),
      place_short:      ctx.blocks.tallgrass.with_data(1), // Grass
      place_tall_lower: ctx.blocks.double_plant.with_data(2), // Tall grass lower
      place_tall_upper: ctx.blocks.double_plant.with_data(10), // Tall grass upper

      radius:   4..=10,
      attempts: 20,
    },
  );
}
