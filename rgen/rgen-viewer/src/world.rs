use rgen_base::{Biome, Pos};
use rgen_biome::{BiomeBuilder, WorldBiomes};
use rgen_world::Context;

use crate::color::Color;

pub struct World {
  pub context:   Context,
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
  pub biome: Biome,
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
    ctx: &Context,
    biome: &BiomeBuilder,
    continentalness: f64,
    erosion: f64,
    peaks_valleys: f64,
  ) -> BiomeInfo {
    BiomeInfo {
      biome: biome.id,
      name: biome.name,
      color: biome_color(ctx, biome),
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
  pub fn new(context: Context, generator: WorldBiomes) -> World { World { context, generator } }
}

impl World {
  pub fn column_at(&self, pos: Pos) -> Column {
    let biome = self.generator.choose_biome(pos);

    let height = self.generator.sample_height(pos);

    let continentalness = self.generator.sample_continentalness(pos);
    let erosion = self.generator.sample_erosion(pos);
    let peaks_valleys = self.generator.sample_peaks_valleys(pos);

    Column {
      height,
      biome: BiomeInfo::new(&self.context, biome, continentalness, erosion, peaks_valleys),
    }
  }

  pub fn height_at(&self, pos: Pos) -> f64 { self.generator.sample_height(pos) }
}

fn biome_color(ctx: &Context, biome: &BiomeBuilder) -> Color {
  Color::from_hex(match biome.id {
    b if b == ctx.biomes.ice_plains => 0x518ded,
    b if b == ctx.biomes.cold_taiga => 0x3265db,
    b if b == ctx.biomes.extreme_hills => 0x4f6aab,
    b if b == ctx.biomes.plains => 0x61b086,
    b if b == ctx.biomes.savanna => 0xa19d55,
    b if b == ctx.biomes.river => 0x3487ba,
    b if b == ctx.biomes.stone_beach => 0x527185,
    b if b == ctx.biomes.birch_forest => 0x3fba7b,
    b if b == ctx.biomes.beaches => 0xd6bf6d,
    b => {
      println!("no color for biome {}", ctx.biomes.name_of(b));
      0x000000
    }
  })
}
