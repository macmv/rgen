use crate::BiomeBuilder;
use rgen_base::{Biomes, Blocks};

pub fn forest(blocks: &Blocks, biomes: &Biomes, gen: &mut BiomeBuilder) {
  gen.id = biomes.extreme_hills;
  gen.top_block = blocks.grass;
}
