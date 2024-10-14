use rgen_base::{biome, block};

use super::BiomeBuilder;

#[allow(dead_code)]
pub fn river(gen: &mut BiomeBuilder) {
  gen.id = biome![river];
  gen.color = "#76B49C";
  gen.set_top_block(block![grass]);
}
