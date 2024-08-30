use rgen_placer::placer;

use crate::builder::PlacerStage;

use super::{color, BiomeBuilder, IdContext};

pub fn crag(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#ffffff";
  gen.top_block = ctx.blocks.stone.default_state;

  gen.place("Mossy Bolders", PlacerStage::Tree, placer::MossBoulder::new(ctx.blocks));
  gen.place("Mossy Pool", PlacerStage::Tree, placer::Pool::new(ctx.blocks));
}

pub fn bog(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#ffffff";
  gen.top_block = ctx.blocks.concrete.with_data(color::BROWN);
}
pub fn cold_bog(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#ffffff";
  gen.top_block = ctx.blocks.concrete.with_data(color::BLUE);
}
pub fn fall_bog(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#ffffff";
  gen.top_block = ctx.blocks.concrete.with_data(color::LIGHT_BLUE);
}
pub fn conifer_swamp(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.plains;
  gen.color = "#ffffff";
  gen.top_block = ctx.blocks.concrete.with_data(color::GREEN);
}
