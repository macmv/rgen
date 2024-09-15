use super::{BiomeBuilder, IdContext};

pub fn river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.river;
  gen.color = "#76B49C";
  gen.set_top_block(ctx.blocks.grass.default_state);
}
