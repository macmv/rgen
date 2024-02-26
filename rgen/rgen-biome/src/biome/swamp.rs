use crate::BiomeBuilder;
use rgen_base::{Biomes, Blocks};

pub fn swamp(blocks: &Blocks, biomes: &Biomes, gen: &mut BiomeBuilder) {
  gen.id = biomes.swamp;
  gen.top_block = blocks.grass;
}
