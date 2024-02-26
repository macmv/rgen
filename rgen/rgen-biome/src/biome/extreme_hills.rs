use crate::BiomeBuilder;
use rgen_base::Blocks;

pub fn extreme_hills(blocks: &Blocks, gen: &mut BiomeBuilder) { gen.top_block = blocks.gravel; }
