use rgen_placer::placer;

use crate::builder::{BiomeBuilder, PlacerStage};

use super::IdContext;

pub fn snowy_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.ice_plains;
  gen.top_block = ctx.blocks.gravel.default_state;
  // gen.sub_layer = ctx.blocks.stone;

  gen.place(
    "sand_patches",
    PlacerStage::Sand,
    placer::Splotch { replace: gen.top_block, place: ctx.blocks.sand.default_state, radius: 6 },
  );

  // TODO: add snow ontop of everything
}

pub fn snowy_rock(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.stone_beach;
  gen.top_block = ctx.blocks.stone.default_state;
  // gen.sub_layer = ctx.blocks.stone;

  gen.place(
    "gravel_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block,
      place:   ctx.blocks.gravel.default_state,
      radius:  4,
    },
  );
  gen.place(
    "cobblestone_patches",
    PlacerStage::Sand,
    placer::Splotch {
      replace: gen.top_block,
      place:   ctx.blocks.cobblestone.default_state,
      radius:  2,
    },
  );
  gen.place(
    "stone_patches",
    PlacerStage::Sand,
    placer::Splotch { replace: gen.top_block, place: ctx.blocks.stone.default_state, radius: 2 },
  );

  gen.place(
    "loose_stone",
    PlacerStage::Sand,
    placer::Loose{};
  )

  // gravel patches, cobblstone patches, stone patches
  // loose mossycobblestone, loose stone, loose cobblestone
  // fallen logs
  // dead trees
}

pub fn ancient_shores(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
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
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.wool.default_state;
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
