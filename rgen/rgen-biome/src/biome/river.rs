use rgen_base::{biome, block};

use super::BiomeBuilder;

#[allow(dead_code)]
pub fn river(g: &mut BiomeBuilder) {
  g.id = biome![river];
  g.color = "#76B49C";
  g.set_top_block(block![grass]);
}
