use rgen_base::biome;
use rgen_placer::chunk_placer;

use super::BiomeBuilder;

#[allow(dead_code)]
pub fn cave(g: &mut BiomeBuilder) {
  g.id = biome![plains];
  g.color = "#ffffff";

  g.place_chunk(chunk_placer::GlowVine::new());
}

#[allow(dead_code)]
pub fn lush_cave(g: &mut BiomeBuilder) {
  g.id = biome![plains];
  g.color = "#ffffff";

  g.place_chunk(chunk_placer::LushCaveMoss::new());
  g.place_chunk(chunk_placer::GlowVine::new());
}
