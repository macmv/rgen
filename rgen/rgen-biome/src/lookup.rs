use rgen_base::Pos;
use rgen_placer::noise::NoiseGenerator;

use crate::{builder::BiomeBuilder, WorldBiomes};

enum ContinentalnessCategory {
  Sea,
  Coast,
  NearInland,
  MidInland,
  FarInland,
}

enum PeaksValleysCategory {
  Valley,
  LowSlice,
  MidSlice,
  HighSlice,
  Peak,
}

impl WorldBiomes {
  pub fn choose_biome(&self, seed: u64, pos: Pos) -> &BiomeBuilder {
    use crate::table::*;

    let continentalness = self.continentalness_category(seed, pos);

    let table: &BiomeTable = match continentalness {
      ContinentalnessCategory::Sea => &self.tables.blank_table,
      ContinentalnessCategory::Coast => {
        // todo: coast table
        &self.tables.blank_table
      }

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

          PeaksValleysCategory::LowSlice => &self.tables.blank_table,
          PeaksValleysCategory::MidSlice => &self.tables.blank_table,
          PeaksValleysCategory::HighSlice => &self.tables.blank_table,
          PeaksValleysCategory::Peak => &self.tables.blank_table,
        }
      }
    };

    let temperature = self.temperature(seed, pos);
    let humidity = self.humidity(seed, pos);

    let biomes = &table[(temperature * table.len() as f64) as usize]
      [(humidity * table[0].len() as f64) as usize];

    let variance = self.variance(seed, pos);
    /*
    let biome = biomes[(variance * biomes.len() as f64) as usize];

    biome
    */
    biomes
  }

  fn continentalness_category(&self, seed: u64, pos: Pos) -> ContinentalnessCategory {
    let continentalness =
      self.continentalness_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5;

    match continentalness {
      x if x < 0.1 => ContinentalnessCategory::Sea,
      x if x < 0.3 => ContinentalnessCategory::Coast,
      x if x < 0.6 => ContinentalnessCategory::NearInland,
      x if x < 0.8 => ContinentalnessCategory::MidInland,
      _ => ContinentalnessCategory::FarInland,
    }
  }

  fn peaks_valleys_category(&self, seed: u64, pos: Pos) -> PeaksValleysCategory {
    let seed = seed.wrapping_add(1);

    let peaks_valleys =
      self.peaks_valleys_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5;

    match peaks_valleys {
      x if x < 0.075 => PeaksValleysCategory::Valley,
      x if x < 0.2 => PeaksValleysCategory::LowSlice,
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
    let seed = seed.wrapping_add(4);

    self.temperature_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5
  }

  fn humidity(&self, seed: u64, pos: Pos) -> f64 {
    let seed = seed.wrapping_add(5);

    self.humidity_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5
  }

  fn variance(&self, seed: u64, pos: Pos) -> f64 {
    let seed = seed.wrapping_add(3);

    self.variance_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5
  }
}
