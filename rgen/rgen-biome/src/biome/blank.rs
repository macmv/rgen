use rgen_placer::chunk_placer;

use super::{color, BiomeBuilder, IdContext};

pub fn blank(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#000000";

  gen.place_chunk(chunk_placer::CheckerboardSurface {
    a: ctx.blocks.concrete.with_data(color::MAGENTA),
    b: ctx.blocks.concrete.with_data(color::BLACK),
  });
}
