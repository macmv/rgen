//! Stores all the actual biome implementations.

mod beach;
mod blank;
mod cold;
mod plains;

pub use beach::*;
pub use blank::*;
pub use cold::*;
pub use plains::*;

pub struct IdContext<'a> {
  pub biomes: &'a Biomes,
  pub blocks: &'a Blocks,
}

use rgen_base::{Biomes, Blocks};

use crate::builder::{BiomeBuilder, PlacerStage};

pub type BiomeFn = fn(&IdContext, &mut BiomeBuilder);

impl BiomeBuilder {
  pub fn build(name: &'static str, ctx: &IdContext, build: BiomeFn) -> Self {
    let mut builder = BiomeBuilder::new(name, ctx.blocks);
    build(ctx, &mut builder);
    builder
  }
}
