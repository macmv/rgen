use rgen_base::{Biome, Pos};
use rgen_biome::{BiomeBuilder, WorldBiomes};
use rgen_world::Context;

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

  pub continentalness: f64,
  pub erosion:         f64,
  pub peaks_valleys:   f64,
}

impl Column {
  const EMPTY: Column = Column { height: 0.0, biome: BiomeInfo::VOID };
}

impl BiomeInfo {
  const VOID: BiomeInfo = BiomeInfo {
    biome:           Biome::VOID,
    name:            "void",
    color:           Color::BLACK,
    continentalness: 0.0,
    erosion:         0.0,
    peaks_valleys:   0.0,
  };

  pub fn new(
    biome: &BiomeBuilder,
    continentalness: f64,
    erosion: f64,
    peaks_valleys: f64,
  ) -> BiomeInfo {
    BiomeInfo {
      biome: biome.id,
      name: biome.name,
      color: Color::from_hex(biome.color()),
      continentalness,
      erosion,
      peaks_valleys,
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

    let continentalness = self.generator.sample_continentalness(pos);
    let erosion = self.generator.sample_erosion(pos);
    let peaks_valleys = self.generator.sample_peaks_valleys(pos);

    Column { height, biome: BiomeInfo::new(biome, continentalness, erosion, peaks_valleys) }
  }

  pub fn height_at(&self, pos: Pos) -> f64 { self.generator.sample_height(pos) }
}
