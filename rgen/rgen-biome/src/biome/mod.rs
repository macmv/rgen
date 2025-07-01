//! Stores all the actual biome implementations.


mod cold_region;
#[allow(unused_imports)]
pub use cold_region::*;

mod frozen_region;
#[allow(unused_imports)]
pub use frozen_region::*;

mod hot_region;
#[allow(unused_imports)]
pub use hot_region::*;

mod outdated_regions_and_areas;
#[allow(unused_imports)]
pub use outdated_regions_and_areas::*;

mod temperate_region;
#[allow(unused_imports)]
pub use temperate_region::*;

mod warm_region;
#[allow(unused_imports)]
pub use warm_region::*;


mod cave;
#[allow(unused_imports)]
pub use cave::*;


use crate::builder::{BiomeBuilder, PlacerStage};

pub type BiomeFn = fn(&mut BiomeBuilder);

impl BiomeBuilder {
  pub fn build(seed: u64, name: &'static str, rarity: u32, build: BiomeFn) -> Self {
    let mut builder = BiomeBuilder::new(seed, name, rarity);
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
