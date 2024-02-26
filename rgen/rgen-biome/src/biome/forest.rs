use crate::BiomeBuilder;
use rgen_base::Blocks;

pub fn forest(blocks: &Blocks, gen: &mut BiomeBuilder) { gen.top_block = blocks.grass; }
