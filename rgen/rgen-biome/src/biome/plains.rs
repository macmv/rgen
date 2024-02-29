use super::{BiomeBuilder, IdContext, PlacerStage};
use rgen_placer::placer;

pub fn plains(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.top_block = ctx.blocks.grass;

  gen.place(
    "tree",
    PlacerStage::Tree,
    placer::BasicTree { trunk: ctx.blocks.log, leaves: ctx.blocks.leaves },
  )
}
