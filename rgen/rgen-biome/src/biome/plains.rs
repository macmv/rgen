use crate::{BiomeBuilder, PlacerStage};
use rgen_base::Blocks;
use rgen_placer::placer;

pub fn plains(blocks: &Blocks, gen: &mut BiomeBuilder) {
  gen.top_block = blocks.grass;

  gen.place(
    "tree",
    PlacerStage::Tree,
    placer::BasicTree { trunk: blocks.log, leaves: blocks.leaves },
  )
}
