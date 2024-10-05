use rgen_placer::placer;

use crate::{builder::PlacerStage, BiomeBuilder};

use super::IdContext;

pub fn deep_jungle(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.jungle;
  gen.color = "#E0705F";
  gen.set_top_block(ctx.blocks.grass.default_state);
  gen.add_layer(ctx.blocks.dirt.default_state, 5, 8);

  gen.place("Large Jungle Tree", PlacerStage::Tree, placer::JungleTree::new(&ctx.blocks));

  gen.place(
    "Jungle bushes",
    PlacerStage::Tree,
    placer::BushClumps {
      place_above:   [ctx.blocks.grass.block].into(),
      log:           ctx.blocks.log.with_data(3),
      leaves:        ctx.blocks.leaves.with_data(3),
      avg_per_chunk: 6.0,
      radius:        3..=5,
    },
  );

  gen.place(
    "Grass",
    PlacerStage::Tree,
    placer::GrassClumps {
      place_above:      gen.top_block().into(),
      place_short:      ctx.blocks.tallgrass.with_data(1), // Grass
      place_tall_lower: ctx.blocks.double_plant.with_data(2), // Tall grass lower
      place_tall_upper: ctx.blocks.double_plant.with_data(10), // Tall grass upper

      radius:        4..=10,
      attempts:      50,
      avg_per_chunk: 8.0,
    },
  );
}
