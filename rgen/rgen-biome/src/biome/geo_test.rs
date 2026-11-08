//! Solid diagnostic surfaces for inspecting geographic type boundaries.

use rgen_base::{biome, block};

use crate::builder::BiomeBuilder;

macro_rules! geo_test_biome {
  ($name:ident, $color_name:literal, $hex:literal) => {
    pub fn $name(g: &mut BiomeBuilder) {
      g.id = biome![plains];
      g.color = $hex;
      g.set_top_block(block![concrete[color = $color_name]]);
      g.set_underwater_block(block![concrete[color = $color_name]]);
    }
  };
}

geo_test_biome!(geo_test_mushroom_island, "purple", "#AA55CC");
geo_test_biome!(geo_test_ocean, "blue", "#4263A8");
geo_test_biome!(geo_test_beach, "yellow", "#E8D34A");
geo_test_biome!(geo_test_delta, "orange", "#E58A32");
geo_test_biome!(geo_test_canyon, "red", "#C74440");
geo_test_biome!(geo_test_river, "cyan", "#55B8C8");
geo_test_biome!(geo_test_standard, "green", "#70A65B");
geo_test_biome!(geo_test_hills, "lime", "#A2C943");
geo_test_biome!(geo_test_mountains, "gray", "#777777");
