use crate::BiomeBuilder;
use rgen_base::Blocks;

pub fn ice_plains(blocks: &Blocks, gen: &mut BiomeBuilder) { gen.top_block = blocks.snow; }
