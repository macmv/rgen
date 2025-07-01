use rgen_base::{biome, block};
use rgen_placer::{chunk_placer, placer};

use crate::builder::{BiomeBuilder, PlacerStage};

pub fn blank(g: &mut BiomeBuilder) {
  g.id = biome![plains];
  g.color = "#000000";

  g.set_top_block(block![stone]);
  g.set_underwater_block(block![stone]);

  g.place_chunk(chunk_placer::CheckerboardSurface {
    replace: block![stone].into(),
    a:       block![concrete[color = "magenta"]],
    b:       block![concrete[color = "black"]],
  });
}
