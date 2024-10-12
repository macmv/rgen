use rgen_placer::chunk_placer;

use super::{BiomeBuilder, IdContext};

#[allow(dead_code)]
pub fn cave(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#ffffff";

  gen.place_chunk(chunk_placer::GlowVine::new(ctx.blocks));
}

#[allow(dead_code)]
pub fn lush_cave(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#ffffff";

  gen.place_chunk(chunk_placer::LushCaveMoss::new(ctx.blocks));
  gen.place_chunk(chunk_placer::GlowVine::new(ctx.blocks));
}
