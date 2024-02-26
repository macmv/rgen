use crate::{BiomeBuilder, PlacerStage};
use rgen_base::{Biomes, Blocks};
use rgen_placer::placer;

pub fn plains(blocks: &Blocks, biomes: &Biomes, gen: &mut BiomeBuilder) {
  gen.id = biomes.plains;
  gen.top_block = blocks.grass;

  gen.place(
    "tree",
    PlacerStage::Tree,
    placer::BasicTree { trunk: blocks.log, leaves: blocks.leaves },
  )
}
