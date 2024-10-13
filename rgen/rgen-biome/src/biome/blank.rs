use rgen_base::{biome, block};
use rgen_placer::chunk_placer;

use super::{color, BiomeBuilder};

pub fn blank(gen: &mut BiomeBuilder) {
  gen.id = biome![plains];
  gen.color = "#000000";

  gen.set_top_block(block![stone]);
  gen.set_underwater_block(block![stone]);

  gen.place_chunk(chunk_placer::CheckerboardSurface {
    replace: block![stone].into(),
    a:       block![concrete[color::MAGENTA]],
    b:       block![concrete[color::BLACK]],
  });
}
