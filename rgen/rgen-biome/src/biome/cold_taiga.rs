use crate::BiomeBuilder;
use rgen_base::{Biomes, Blocks};

pub fn cold_taiga(blocks: &Blocks, biomes: &Biomes, gen: &mut BiomeBuilder) {
  gen.id = biomes.cold_taiga;
  gen.top_block = blocks.snow;
}
