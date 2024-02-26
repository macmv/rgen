use crate::{BiomeBuilder, PlacerStage};
use rgen_base::Blocks;
use rgen_placer::placer;

pub fn lush_swamp(blocks: &Blocks, gen: &mut BiomeBuilder) {
  gen.place(
    "mud",
    PlacerStage::Sand2,
    placer::Splatter { attempts: 64, place: blocks.dirt, replace: blocks.stone },
  );
}
