use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{color, BiomeBuilder, IdContext};

pub fn crag(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#8CB4B9";
  gen.set_top_block(ctx.blocks.stone.default_state);

  gen.place("Mossy Bolders", PlacerStage::Tree, placer::MossBoulder::new(ctx.blocks));
  gen.place("Mossy Pool", PlacerStage::Tree, placer::Pool::new(ctx.blocks));

  gen.place(
    "cobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.cobblestone.default_state,
      radius:        1..=3,
      avg_per_chunk: 2.0,
    },
  );

  gen.place(
    "gravel",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.gravel.default_state,
      radius:        1..=3,
      avg_per_chunk: 2.0,
    },
  );

  gen.place(
    "andesite",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.stone.with_data(5),
      radius:        1..=3,
      avg_per_chunk: 2.0,
    },
  );
  gen.place(
    "andesite little",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.stone.with_data(5),
      radius:        1..=2,
      avg_per_chunk: 6.0,
    },
  );

  gen.place(
    "iron ore",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.iron_ore.default_state,
      radius:        1..=3,
      avg_per_chunk: 0.1,
    },
  );

  gen.place(
    "mossystone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.rgen_mossy_stone.default_state,
      radius:        1..=2,
      avg_per_chunk: 0.05,
    },
  );
  gen.place(
    "mossycobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace:       gen.top_block().into(),
      place:         ctx.blocks.rgen_mossy_cobblestone.default_state,
      radius:        1..=3,
      avg_per_chunk: 1.0,
    },
  );

  gen.place(
    "grass",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    130,
      place_above: [
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place:       ctx.blocks.tallgrass.with_data(2),
    },
  );

  gen.place(
    "ferns",
    PlacerStage::Tree,
    placer::Scatter {
      attempts:    230,
      place_above: [
        ctx.blocks.grass.block,
        ctx.blocks.rgen_mossy_stump.block,
        ctx.blocks.rgen_mossy_cobblestone.block,
        ctx.blocks.rgen_mossy_stone.block,
      ]
      .into(),
      place:       ctx.blocks.tallgrass.with_data(1),
    },
  );
}

pub fn bog(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#7FAFB4";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::BROWN));
}
pub fn cold_bog(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#5F9CA1";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::BLUE));
}
pub fn fall_bog(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#4F8D96";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::LIGHT_BLUE));
}
pub fn conifer_swamp(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#93C0C7";
  gen.set_top_block(ctx.blocks.concrete.with_data(color::GREEN));
}
