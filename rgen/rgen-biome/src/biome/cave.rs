use rgen_placer::chunk_placer;

use super::{BiomeBuilder, IdContext};

pub fn cave(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#ffffff";
  gen.top_block = ctx.blocks.grass.default_state;
  gen.sub_layer = ctx.blocks.dirt.default_state;

  gen.place_chunk(chunk_placer::GlowVine::new(ctx.blocks));
}

pub fn lush_cave(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#ffffff";
  gen.top_block = ctx.blocks.grass.default_state;
  gen.sub_layer = ctx.blocks.dirt.default_state;

  gen.place_chunk(chunk_placer::LushCaveMoss::new(ctx.blocks));
  gen.place_chunk(chunk_placer::GlowVine::new(ctx.blocks));
}
