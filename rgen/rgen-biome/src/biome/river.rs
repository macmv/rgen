use super::{BiomeBuilder, IdContext};

pub fn river(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.river;
  gen.color = "#ffffff";
  gen.top_block = ctx.blocks.grass.default_state;
  gen.sub_layer = ctx.blocks.dirt.default_state;
}
