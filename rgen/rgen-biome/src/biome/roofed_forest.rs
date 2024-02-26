use crate::BiomeBuilder;
use rgen_base::{Biomes, Blocks};

pub fn roofed_forest(blocks: &Blocks, biomes: &Biomes, gen: &mut BiomeBuilder) {
  gen.id = biomes.roofed_forest;
  gen.top_block = blocks.grass;
}
