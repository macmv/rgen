use crate::BiomeBuilder;
use rgen_base::{Biomes, Blocks};

pub fn savanna(blocks: &Blocks, biomes: &Biomes, gen: &mut BiomeBuilder) {
  gen.id = biomes.savanna;
  gen.top_block = blocks.grass;
}
