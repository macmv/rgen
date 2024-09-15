use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{BiomeBuilder, IdContext};

pub fn flat_desert(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.desert;
  gen.color = "#E0705F";
  gen.set_top_block(ctx.blocks.sand.default_state);
  gen.add_layer(ctx.blocks.sandstone.default_state, 5, 8);

  gen.place(
    "Large Cactus",
    PlacerStage::Tree,
    placer::Cactus {
      avg_in_chunk: 0.5 as f64,
      arms:         ctx.blocks.rgen_cactus_arm.default_state,
      place_above:  ctx.blocks.sand.block.into(),
      body:         ctx.blocks.rgen_cactus.default_state,
    },
  );
}

pub fn lush_desert(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.desert;
  gen.color = "#D14A3F";
  gen.set_top_block(ctx.blocks.sand.default_state);
  gen.add_layer(ctx.blocks.sandstone.default_state, 5, 8);

  gen.place(
    "trees",
    PlacerStage::Tree,
    placer::BasicDryBush {
      place_above:  [ctx.blocks.sand.block].into(),
      trunk:        ctx.blocks.log.default_state,
      leaves:       ctx.blocks.leaves.default_state,
      avg_in_chunk: 1.0,
    },
  );

  gen.place(
    "Large Cactus",
    PlacerStage::Tree,
    placer::Cactus {
      avg_in_chunk: 1 as f64,
      arms:         ctx.blocks.rgen_cactus_arm.default_state,
      place_above:  ctx.blocks.sand.block.into(),
      body:         ctx.blocks.rgen_cactus.default_state,
    },
  );

  gen.place(
    "cactus blue",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    30,
      place_above: [ctx.blocks.sand.block].into(),
      place:       ctx.blocks.rgen_cactus.with_data(1),
    },
  );

  gen.place(
    "cactus red",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    20,
      place_above: [ctx.blocks.sand.block].into(),
      place:       ctx.blocks.rgen_cactus.with_data(3),
    },
  );
}

pub fn bad_lands(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.mesa;
  gen.color = "#C74538";
  gen.set_top_block(ctx.blocks.hardened_clay.default_state);
}

pub fn dune_sea(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.desert;
  gen.color = "#EA7468";
  gen.set_top_block(ctx.blocks.sand.default_state);
}
