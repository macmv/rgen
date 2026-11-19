use rgen_base::{Biome, Pos};
use rgen_biome::{BiomeBuilder, WorldBiomes};

use crate::color::Color;

pub struct World {
  pub generator: WorldBiomes,
}

#[derive(Clone, Copy)]
pub struct Column {
  /// The height of this column, in blocks.
  pub height: f64,

  /// The biome at this column.
  pub biome: BiomeInfo,
}

#[derive(Clone, Copy)]
pub struct BiomeInfo {
  #[allow(dead_code)]
  pub biome: Biome,
  #[allow(dead_code)]
  pub name:  &'static str,
  pub color: Color,

  pub land: bool,
}

impl Column {
  const EMPTY: Column = Column { height: 0.0, biome: BiomeInfo::VOID };
}

impl BiomeInfo {
  const VOID: BiomeInfo = BiomeInfo {
    biome:           Biome::Void,
    name:            "void",
    color:           Color::BLACK,
    land: false,
  };

  pub fn new(
    biome: &BiomeBuilder,
    land: bool,
  ) -> BiomeInfo {
    BiomeInfo {
      biome: biome.id,
      name: biome.name,
      color: Color::from_hex(biome.color()),
      land,
    }
  }
}

impl Default for Column {
  fn default() -> Column { Column::EMPTY }
}

impl World {
  pub fn new(generator: WorldBiomes) -> World { World { generator } }
}

impl World {
  pub fn column_at(&self, pos: Pos) -> Column {
    let biome = self.generator.choose_biome(pos);

    let height = self.generator.sample_height(pos);

    let land = self.generator.is_continent_land(pos);

    Column { height, biome: BiomeInfo::new(biome, land) }
  }

  pub fn height_at(&self, pos: Pos) -> f64 { self.generator.sample_height(pos) }
}
