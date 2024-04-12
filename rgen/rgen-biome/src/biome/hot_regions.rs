use super::{BiomeBuilder, IdContext};

pub fn bad_lands(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.mesa;
  gen.top_block = ctx.blocks.hardened_clay.default_state;
}

pub fn dune_sea(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.desert;
  gen.top_block = ctx.blocks.sand.default_state;
}
