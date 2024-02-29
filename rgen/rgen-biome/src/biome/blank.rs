use crate::BiomeBuilder;
use rgen_base::{Biomes, Blocks};

pub fn blank(blocks: &Blocks, biomes: &Biomes, gen: &mut BiomeBuilder) {
  gen.id = biomes.plains;
  gen.top_block = blocks.grass;
}
