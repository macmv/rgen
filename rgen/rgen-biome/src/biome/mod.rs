//! Stores all the actual biome implementations.

mod blank;
mod coast_regions;
mod cold_regions;
mod cool_regions;
mod dry_regions;
mod frozen_region;
mod hot_regions;
mod river;
mod temprate_regions;
mod warm_regions;

pub use blank::*;
pub use coast_regions::*;
pub use cold_regions::*;
pub use cool_regions::*;
pub use dry_regions::*;
pub use frozen_region::*;
pub use hot_regions::*;
pub use river::*;
pub use temprate_regions::*;
pub use warm_regions::*;

pub struct IdContext<'a> {
  pub biomes: &'a Biomes,
  pub blocks: &'a Blocks,
}

use rgen_base::{Biomes, Blocks};

use crate::builder::{BiomeBuilder, PlacerStage};

pub type BiomeFn = fn(&IdContext, &mut BiomeBuilder);

impl BiomeBuilder {
  pub fn build(name: &'static str, ctx: &IdContext, rarity: f64, build: BiomeFn) -> Self {
    let mut builder = BiomeBuilder::new(name, ctx.blocks, rarity);
    build(ctx, &mut builder);
    builder
  }
}
