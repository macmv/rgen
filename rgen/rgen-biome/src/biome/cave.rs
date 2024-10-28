use rgen_base::biome;
use rgen_placer::chunk_placer;

use super::BiomeBuilder;

#[allow(dead_code)]
pub fn cave(gen: &mut BiomeBuilder) {
  gen.id = biome![plains];
  gen.color = "#ffffff";

  gen.place_chunk(chunk_placer::GlowVine::new());
}

#[allow(dead_code)]
pub fn lush_cave(gen: &mut BiomeBuilder) {
  gen.id = biome![plains];
  gen.color = "#ffffff";

  gen.place_chunk(chunk_placer::LushCaveMoss::new());
  gen.place_chunk(chunk_placer::GlowVine::new());
}
