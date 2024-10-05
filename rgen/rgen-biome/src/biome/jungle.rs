use rgen_placer::placer;

use crate::{builder::PlacerStage, BiomeBuilder};

use super::IdContext;

pub fn deep_jungle(ctx: &IdContext, gen: &mut BiomeBuilder) {
  gen.id = ctx.biomes.jungle;
  gen.color = "#E0705F";
  gen.set_top_block(ctx.blocks.grass.default_state);
  gen.add_layer(ctx.blocks.dirt.default_state, 5, 8);

  gen.place("Large Jungle Tree", PlacerStage::Tree, placer::JungleTree::new(&ctx.blocks));
}
