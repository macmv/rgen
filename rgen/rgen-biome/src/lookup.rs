use rgen_base::Pos;
use rgen_placer::noise::NoiseGenerator;

use crate::{builder::BiomeBuilder, WorldBiomes};

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
  pub fn choose_biome(&self, seed: u64, pos: Pos) -> &BiomeBuilder {
    use crate::table::*;

    if self.biome_override {
      return &self.tables.blank_table[0][0][0];
    }

    let continentalness = self.continentalness_category(seed, pos);

    let table: &BiomeTable = match continentalness {
      ContinentalnessCategory::MushroomIsland => &self.tables.blank_table,
      ContinentalnessCategory::Sea => &self.tables.sea_table,
      ContinentalnessCategory::Coast => &self.tables.beach_table,

      // Inland cases
      _ => {
        let peaks_valleys = self.peaks_valleys_category(seed, pos);

        match peaks_valleys {
          PeaksValleysCategory::Valley => {
            let erosion = self.erosion_category(seed, pos);

            if erosion <= 4 {
              // river table
              &self.tables.blank_table
            } else {
              &self.tables.valley_table
            }
          }

          PeaksValleysCategory::River => &self.tables.river_table,

          PeaksValleysCategory::LowSlice => &self.tables.standard_table,
          PeaksValleysCategory::MidSlice => &self.tables.standard_table,
          PeaksValleysCategory::HighSlice => &self.tables.standard_table,
          PeaksValleysCategory::Peak => &self.tables.blank_table,
        }
      }
    };

    // let table = &self.tables.beach_table;

    let temperature = self.temperature(seed, pos);
    let humidity = self.humidity(seed, pos);

    let biomes = &table[(temperature * table.len() as f64) as usize]
      [(humidity * table[0].len() as f64) as usize];

    let total = biomes.iter().map(|b| b.rarity).sum::<f64>();
    let mut variance = self.variance(seed, pos) * total;
    for biome in biomes {
      variance -= biome.rarity;
      if variance <= 0.0 {
        return biome;
      }
    }
    &biomes[0]
  }

  fn continentalness_category(&self, seed: u64, pos: Pos) -> ContinentalnessCategory {
    let continentalness =
      self.continentalness_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5;

    match continentalness {
      x if x < 0.02 => ContinentalnessCategory::MushroomIsland,
      x if x < 0.35 => ContinentalnessCategory::Sea,
      x if x < 0.40 => ContinentalnessCategory::Coast,
      x if x < 0.60 => ContinentalnessCategory::NearInland,
      x if x < 0.80 => ContinentalnessCategory::MidInland,
      _ => ContinentalnessCategory::FarInland,
    }
  }

  pub(crate) fn peaks_valleys(&self, seed: u64, pos: Pos) -> f64 {
    let seed = seed.wrapping_add(1);

    self.peaks_valleys_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5
  }

  fn peaks_valleys_category(&self, seed: u64, pos: Pos) -> PeaksValleysCategory {
    let seed = seed.wrapping_add(1);

    let peaks_valleys =
      self.peaks_valleys_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5;

    match peaks_valleys {
      x if x < 0.075 => PeaksValleysCategory::Valley,
      x if x < 0.37 => PeaksValleysCategory::LowSlice,
      x if x < 0.43 => PeaksValleysCategory::River,
      x if x < 0.6 => PeaksValleysCategory::MidSlice,
      x if x < 0.85 => PeaksValleysCategory::HighSlice,
      _ => PeaksValleysCategory::Peak,
    }
  }

  fn erosion_category(&self, seed: u64, pos: Pos) -> u8 {
    let seed = seed.wrapping_add(2);

    let erosion = self.erosion_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5;

    // FIXME: This is dumb
    (erosion * 6.9999) as u8
  }

  fn temperature(&self, seed: u64, pos: Pos) -> f64 {
    let seed = seed.wrapping_add(3);

    self.temperature_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5
  }

  fn humidity(&self, seed: u64, pos: Pos) -> f64 {
    let seed = seed.wrapping_add(4);

    self.humidity_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5
  }

  fn variance(&self, seed: u64, pos: Pos) -> f64 {
    let seed = seed.wrapping_add(5);

    self.variance_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5
  }
}
