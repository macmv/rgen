use rgen_placer::placer;

use crate::builder::{BiomeBuilder, PlacerStage};

use super::IdContext;

pub fn snowy_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.top_block = ctx.blocks.gravel.default_state;
  gen.sub_layer = ctx.blocks.stone.default_state;

  gen.place(
    "sand_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.sand.default_state,
      radius:  4..=7,
    },
  );

  // TODO: add snow ontop of everything
}

pub fn snowy_rock(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.stone_beach;
  gen.top_block = ctx.blocks.stone.default_state;
  gen.sub_layer = ctx.blocks.stone.default_state;
}

pub fn ancient_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.sand.default_state;
  gen.sub_layer = ctx.blocks.stone.default_state;

  gen.place(
    "gravel_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.gravel.default_state,
      radius:  4..=7,
    },
  );
  gen.place(
    "cobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.cobblestone.default_state,
      radius:  3..=6,
    },
  );
  gen.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block.into(),
      place:   ctx.blocks.stone.default_state,
      radius:  4..=7,
    },
  );

  gen.place(
    "loose_stone",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: ctx.blocks.stone.default_state.into(),
      place:       ctx.blocks.stone.default_state,
      attempts:    20,
    },
  );
  gen.place(
    "loose_moss",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: ctx.blocks.stone.default_state.into(),
      place:       ctx.blocks.mossy_cobblestone.default_state,
      attempts:    40,
    },
  );
  gen.place(
    "loose_cobblestone",
    PlacerStage::Sand,
    placer::Scatter {
      place_above: ctx.blocks.stone.default_state.into(),
      place:       ctx.blocks.stone.default_state,
      attempts:    20,
    },
  );
  /*
  gen.place(
    "dead tree",
    PlacerStage::Tree,
    //placer::BasicTree { trunk: ctx.blocks.log.block, leaves: ctx.blocks.air.block },
  )*/

  // gravel patches, cobblstone patches, stone patches
  // loose mossycobblestone, loose stone, loose cobblestone
  // fallen logs
  // dead trees
}

pub fn mossy_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
}

pub fn dry_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
}

pub fn bare_rock(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
}

pub fn wet_rock(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
}

pub fn sand_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
}

pub fn monument_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
}

pub fn red_sand_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.beaches;
  gen.top_block = ctx.blocks.sand.with_data(1);
  gen.sub_layer = ctx.blocks.sand.with_data(1);
}

pub fn red_monument_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
}

pub fn palm_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
}

pub fn chaparral_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
}

pub fn jungle_beach(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
}
