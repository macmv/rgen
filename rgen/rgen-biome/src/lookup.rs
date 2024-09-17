use rgen_base::Pos;
use rgen_placer::noise::NoiseGenerator;

use crate::{builder::BiomeBuilder, table::BiomeTable, WorldBiomes};

enum ContinentalnessCategory {
  MushroomIsland,
  Sea,
  Coast,
  NearInland,
  MidInland,
  FarInland,
}

enum PeaksValleysCategory {
  Valley,
  LowSlice,
  River,
  MidSlice,
  HighSlice,
  Peak,
}

impl WorldBiomes {
  pub fn choose_biome(&self, pos: Pos) -> &BiomeBuilder {
    // Check for Y + 1 to avoid caves showing up on the surface.
    if self.height_info(pos + Pos::new(0, 1, 0)).underground() {
      self.choose_cave_biome(pos)
    } else {
      self.choose_surface_biome(pos)
    }
  }

  fn choose_cave_biome(&self, pos: Pos) -> &BiomeBuilder {
    let temperature = self.temperature(pos);
    let humidity = self.humidity(pos);

    // FIXME: This needs rewriting.
    let biomes = &self.old_table[(temperature * self.old_table.len() as f64) as usize]
      [(humidity * self.old_table[0].len() as f64) as usize];

    let total = biomes.iter().map(|b| b.rarity).sum::<f64>();
    let mut variance = self.variance(pos) * total;
    for biome in biomes {
      variance -= biome.rarity;
      if variance <= 0.0 {
        return biome;
      }
    }
    &biomes[0]
  }

  fn choose_surface_biome(&self, pos: Pos) -> &BiomeBuilder {
    if self.biome_override {
      return &self.old_table[0][0][0];
    }

    let temperature = self.temperature(pos);
    let humidity = self.humidity(pos);

    let biomes = &self.old_table[(temperature * self.old_table.len() as f64) as usize]
      [(humidity * self.old_table[0].len() as f64) as usize];

    let total = biomes.iter().map(|b| b.rarity).sum::<f64>();
    let mut variance = self.variance(pos) * total;
    for biome in biomes {
      variance -= biome.rarity;
      if variance <= 0.0 {
        return biome;
      }
    }
    &biomes[0]
  }

  fn continentalness_category(&self, pos: Pos) -> ContinentalnessCategory {
    let continentalness = self.sample_continentalness(pos);

    match continentalness {
      x if x < 0.02 => ContinentalnessCategory::MushroomIsland,
      x if x < 0.32 => ContinentalnessCategory::Sea,
      x if x < 0.37 => ContinentalnessCategory::Coast,
      x if x < 0.60 => ContinentalnessCategory::NearInland,
      x if x < 0.80 => ContinentalnessCategory::MidInland,
      _ => ContinentalnessCategory::FarInland,
    }
  }

  fn peaks_valleys_category(&self, pos: Pos) -> PeaksValleysCategory {
    let peaks_valleys = self.sample_peaks_valleys(pos);

    match peaks_valleys {
      x if x < 0.075 => PeaksValleysCategory::Valley,
      x if x < 0.48 => PeaksValleysCategory::LowSlice,
      x if x < 0.52 => PeaksValleysCategory::River,
      x if x < 0.6 => PeaksValleysCategory::MidSlice,
      x if x < 0.85 => PeaksValleysCategory::HighSlice,
      _ => PeaksValleysCategory::Peak,
    }
  }

  fn erosion_category(&self, pos: Pos) -> u8 {
    let erosion = self.sample_erosion(pos);

    // FIXME: This is dumb
    (erosion * 6.9999) as u8
  }

  fn temperature(&self, pos: Pos) -> f64 {
    self.temperature_map.generate(pos.x as f64, pos.z as f64) * 0.5 + 0.5
  }

  fn humidity(&self, pos: Pos) -> f64 {
    self.humidity_map.generate(pos.x as f64, pos.z as f64) * 0.5 + 0.5
  }

  fn variance(&self, pos: Pos) -> f64 {
    self.variance_map.generate(pos.x as f64, pos.z as f64) * 0.5 + 0.5
  }
}
