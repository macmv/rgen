//! Stores all the actual biome implementations.

mod blank;
mod cave;
mod coast_regions;
mod cold_region;

mod forests;
mod hot_regions;
mod jungle;
mod river;
mod temperate_river;
mod tropical_regions;
mod warm_temperate_regions;

pub use blank::*;
#[allow(unused_imports)]
pub use cave::*;
#[allow(unused_imports)]
pub use coast_regions::*;
pub use cold_region::*;
#[allow(unused_imports)]
pub use forests::*;
#[allow(unused_imports)]
pub use hot_regions::*;
#[allow(unused_imports)]
pub use jungle::*;
#[allow(unused_imports)]
pub use river::*;
#[allow(unused_imports)]
pub use temperate_river::*;
#[allow(unused_imports)]
pub use tropical_regions::*;
#[allow(unused_imports)]
pub use warm_temperate_regions::*;

pub struct IdContext<'a> {
  pub biomes: &'a Biomes,
  pub blocks: &'a Blocks,
}

use rgen_base::{Biomes, Blocks};

use crate::builder::{BiomeBuilder, PlacerStage};

pub type BiomeFn = fn(&mut BiomeBuilder);

impl BiomeBuilder {
  pub fn build(name: &'static str, rarity: u32, build: BiomeFn) -> Self {
    let mut builder = BiomeBuilder::new(name, rarity);
    build(&mut builder);
    if builder.color.is_empty() {
      panic!("biome {} has no color", name);
    }
    builder.color();
    builder.finish();
    builder
  }

  pub fn color(&self) -> u32 {
    assert_eq!(self.color.len(), 7);
    assert!(&self.color[0..1] == "#", "color must start with #");

    let mut color = 0_u32;
    for c in self.color[1..].bytes() {
      color = (color << 4)
        | match c {
          b'0'..=b'9' => u32::from(c - b'0'),
          b'a'..=b'f' => u32::from(c - b'a' + 10),
          b'A'..=b'F' => u32::from(c - b'A' + 10),
          _ => panic!("invalid color character"),
        };
    }

    color
  }
}

#[allow(dead_code)]
pub mod color {
  pub const WHITE: u8 = 0;
  pub const ORANGE: u8 = 1;
  pub const MAGENTA: u8 = 2;
  pub const LIGHT_BLUE: u8 = 3;
  pub const YELLOW: u8 = 4;
  pub const LIME: u8 = 5;
  pub const PINK: u8 = 6;
  pub const GRAY: u8 = 7;
  pub const SILVER: u8 = 8;
  pub const CYAN: u8 = 9;
  pub const PURPLE: u8 = 10;
  pub const BLUE: u8 = 11;
  pub const BROWN: u8 = 12;
  pub const GREEN: u8 = 13;
  pub const RED: u8 = 14;
  pub const BLACK: u8 = 15;
}
