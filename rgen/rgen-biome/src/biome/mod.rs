//! Stores all the actual biome implementations.

mod blank;
mod cold;
mod plains;

pub use blank::*;
pub use cold::*;
pub use plains::*;

pub struct IdContext<'a> {
  pub biomes: &'a Biomes,
  pub blocks: &'a Blocks,
}

use rgen_base::{Biomes, Blocks};
use rgen_placer::Rng;

use crate::{
  builder::{BiomeBuilder, PlacerStage},
  climate::Climate,
};

/// Stores the map of climates to biomes.
pub struct ClimateMap {
  default: BiomeBuilder,
}

pub type BiomeFn = fn(&IdContext, &mut BiomeBuilder);

impl BiomeBuilder {
  pub fn build(name: &'static str, ctx: &IdContext, build: BiomeFn) -> Self {
    let mut builder = BiomeBuilder::new(name, ctx.blocks);
    build(ctx, &mut builder);
    builder
  }
}

impl ClimateMap {
  pub fn new(blocks: &Blocks, biome_ids: &rgen_base::Biomes) -> ClimateMap {
    let ctx = IdContext { biomes: biome_ids, blocks };

    ClimateMap { default: BiomeBuilder::build("blank", &ctx, blank) }
  }

  pub fn choose(&self, _rng: &mut Rng, _climate: Climate) -> &BiomeBuilder {
    &self.default
    // rng.choose(self.biomes.get(&climate).unwrap())
  }
}
