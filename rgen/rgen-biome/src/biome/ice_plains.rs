use crate::BiomeBuilder;
use rgen_base::{Biomes, Blocks};

pub fn ice_plains(blocks: &Blocks, biomes: &Biomes, gen: &mut BiomeBuilder) {
  gen.id = biomes.ice_plains;
  gen.top_block = blocks.snow;
}
