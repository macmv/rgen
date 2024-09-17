use rgen_base::Pos;
use rgen_placer::noise::NoiseGenerator;

use crate::{
  builder::BiomeBuilder,
  table::{ClimateType, GeographicType, CLIMATE_TABLE},
  WorldBiomes,
};

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
    // FIXME: This needs rewriting.
    /*
    let biomes = &self.old_table[(temperature * self.old_table.len() as f64) as usize]   [(humidity * self.old_table[0].len() as f64) as usize];
    */
    let biomes = &self.composition_lookup.blank;

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

  pub fn geographic_type(&self, pos: Pos) -> GeographicType {
    let continentalness = self.continentalness_category(pos);

    match continentalness {
      ContinentalnessCategory::MushroomIsland => GeographicType::MushroomIsland,
      ContinentalnessCategory::Sea => GeographicType::Ocean,
      ContinentalnessCategory::Coast => GeographicType::Beach,

      // Inland cases
      _ => {
        let peaks_valleys = self.peaks_valleys_category(pos);

        match peaks_valleys {
          PeaksValleysCategory::Valley => {
            let erosion = self.erosion_category(pos);

            if erosion <= 4 {
              // river table
              GeographicType::River
            } else {
              GeographicType::Valley
            }
          }

          PeaksValleysCategory::River => GeographicType::River,

          PeaksValleysCategory::LowSlice => GeographicType::Standard,
          PeaksValleysCategory::MidSlice => GeographicType::Standard,
          PeaksValleysCategory::HighSlice => GeographicType::Hills,
          PeaksValleysCategory::Peak => GeographicType::Mountains,
        }
      }
    }
  }

  pub fn climate_type(&self, pos: Pos) -> ClimateType {
    let temperature = self.temperature(pos);
    let humidity = self.humidity(pos);

    CLIMATE_TABLE[(temperature * CLIMATE_TABLE.len() as f64) as usize]
      [(humidity * CLIMATE_TABLE[0].len() as f64) as usize]
  }

  fn choose_surface_biome(&self, pos: Pos) -> &BiomeBuilder {
    if self.biome_override {
      return &self.composition_lookup.blank[0];
    }

    let geographic_type = self.geographic_type(pos);
    let _climate_type = self.climate_type(pos);

    let biomes = self.composition_lookup.choose(geographic_type, ClimateType::WarmTemperate); // climate_type

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
