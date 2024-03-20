use super::{BiomeBuilder, IdContext};

pub fn cave(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.grass.default_state;
  gen.sub_layer = ctx.blocks.dirt.default_state;
}

pub fn lush_cave(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.grass.default_state;
  gen.sub_layer = ctx.blocks.dirt.default_state;
}
