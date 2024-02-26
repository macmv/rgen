use crate::BiomeBuilder;
use rgen_base::Blocks;

pub fn cold_taiga(blocks: &Blocks, gen: &mut BiomeBuilder) { gen.top_block = blocks.snow; }
