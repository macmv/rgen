use crate::BiomeBuilder;
use rgen_base::{Biomes, Blocks};

pub fn extreme_hills(blocks: &Blocks, biomes: &Biomes, gen: &mut BiomeBuilder) {
  gen.id = biomes.extreme_hills;
  gen.top_block = blocks.gravel;
}
