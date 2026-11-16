use std::{cell::RefCell, num::NonZero};

use lru::LruCache;
use rgen_base::Pos;
use rgen_placer::noise::NoiseGenerator;

use crate::{
  WorldBiomes,
  builder::BiomeBuilder,
  feature,
  table::{CLIMATE_TABLE, ClimateType, GeographicType},
  terrain::{is_delta, is_river},
};

#[derive(Debug)]
pub enum ContinentalnessCategory {
  MushroomIsland,
  Sea,
  Coast,
  NearInland,
  MidInland,
  FarInland,
}

#[derive(Debug)]
pub enum PeaksValleysCategory {
  Valley,
  LowSlice,
  MidSlice,
  HighSlice,
  Peak,
}

// This is a key to lookup any biome quickly. This key is generated from all the
// noise maps and composition tables.
#[derive(Clone, Copy)]
pub struct BiomeKey {
  geographic: GeographicType,
  climate:    ClimateType,
  variance:   u32,
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

  pub fn choose_cave_biome(&self, _pos: Pos) -> &BiomeBuilder {
    // FIXME: This needs rewriting.
    /*
    let biomes = &self.old_table[(temperature * self.old_table.len() as f64) as usize]   [(humidity * self.old_table[0].len() as f64) as usize];
    */
    &self.composition_lookup.blank[0]
  }

  pub fn geographic_type(&self, pos: Pos) -> GeographicType {
    let noise = self.terrain.sample(pos.x as f64, pos.z as f64);
    if is_delta(noise) {
      return GeographicType::Delta;
    }

    // Include the dry bank around the water channel in the river region.
    if is_river(noise) {
      return GeographicType::River;
    }

    use ContinentalnessCategory::*;

    // Keep coastal geography separate. Canyon selection is disabled until
    // it describes actual canyon terrain; its type and biome mappings remain.
    // All ordinary inland terrain now uses the local hilliness measurement.
    match self.continentalness_category(pos) {
      MushroomIsland | Sea => return GeographicType::Ocean,
      Coast => return GeographicType::Beach,
      _ => {}
    }

    self.inland_hilliness(pos)
  }

  pub fn climate_type(&self, pos: Pos) -> ClimateType {
    if let Some(climate) = feature::CLIMATE_TYPE_OVERRIDE {
      return climate;
    }

    let humidity = self.humidity(pos);
    if feature::MEDITERRANEAN_BADLANDS_MIX {
      return if humidity < 0.5 { ClimateType::BadLands } else { ClimateType::Mediterranean };
    }
    let temperature = self.temperature(pos);

    CLIMATE_TABLE[(temperature * CLIMATE_TABLE.len() as f64) as usize]
      [(humidity * CLIMATE_TABLE[0].len() as f64) as usize]
  }

  pub fn choose_surface_biome(&self, pos: Pos) -> &BiomeBuilder {
    let key = self.choose_surface_biome_cached(pos);
    self.choose_surface_biome_from_key(key)
  }

  fn choose_surface_biome_from_key(&self, key: BiomeKey) -> &BiomeBuilder {
    if !enabled!(Biomes) {
      return &self.composition_lookup.blank[0];
    }

    if let Some(biome) = self.composition_lookup.override_for(key.geographic) {
      return biome;
    }

    let biomes = if crate::feature::BIOME_OVERRIDE {
      &self.composition_lookup.blank
    } else {
      self.composition_lookup.choose(key.geographic, key.climate)
    };

    let total = biomes.iter().map(|b| b.rarity).sum::<u32>();
    let mut variance = key.variance % total;
    for biome in biomes {
      variance = match variance.checked_sub(biome.rarity) {
        Some(v) => v,
        None => return biome,
      };
    }

    &biomes[0]
  }

  fn choose_surface_biome_cached(&self, pos: Pos) -> BiomeKey {
    // Inspector splines and density toggles can change without a new seed.
    if cfg!(feature = "inspector") { return self.choose_surface_biome_uncached(pos); }
    thread_local! {
      static CACHE: RefCell<LruCache<(u64, i32, i32), BiomeKey>> = RefCell::new(LruCache::new(NonZero::new(256).unwrap()));
    }

    CACHE.with(|cache| {
      let mut cache = cache.borrow_mut();
      let key = (self.seed, pos.x, pos.z);

      if let Some(biome) = cache.get(&key) {
        return *biome;
      }

      let biome = self.choose_surface_biome_uncached(pos);
      cache.put(key, biome);
      biome
    })
  }

  fn choose_surface_biome_uncached(&self, pos: Pos) -> BiomeKey {
    profile_function!();

    BiomeKey {
      geographic: feature::GEOGRAPHIC_TYPE_OVERRIDE.unwrap_or_else(|| self.geographic_type(pos)),
      climate:    self.climate_type(pos),
      variance:   self.variance(pos),
    }
  }

  pub fn continentalness_category(&self, pos: Pos) -> ContinentalnessCategory {
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

  pub fn peaks_valleys_category(&self, pos: Pos) -> PeaksValleysCategory {
    let continentalness = self.sample_continentalness(pos);
    let peaks_valleys = self.sample_peaks_valleys(pos);

    let v = if peaks_valleys > 0.5 { 1.0 - peaks_valleys } else { peaks_valleys };

    // This is a bit odd, but its required for rivers. We need to modify the
    // border of the valley edge given the continentalness, so that river
    // edges line up correctly.
    //
    // Also these numbers are really fine-tuned. Change at your own risk.
    let river_edge = 0.445 + (continentalness - 0.3) * 0.2;

    match v {
      x if x < 0.2 => PeaksValleysCategory::Peak,
      x if x < 0.3 => PeaksValleysCategory::HighSlice,
      x if x < 0.4 => PeaksValleysCategory::MidSlice,
      x if x < river_edge => PeaksValleysCategory::LowSlice,
      _ => PeaksValleysCategory::Valley,
    }
  }

  pub fn erosion_category(&self, pos: Pos) -> u8 {
    let erosion = self.sample_erosion(pos);

    match erosion {
      x if x < 0.20 => 0,
      x if x < 0.33 => 1,
      x if x < 0.40 => 2,
      x if x < 0.50 => 3,
      x if x < 0.60 => 4,
      x if x < 0.80 => 5,
      _ => 6,
    }
  }

  fn temperature(&self, pos: Pos) -> f64 {
    self.temperature_map.generate(pos.x as f64, pos.z as f64) * 0.5 + 0.5
  }

  fn humidity(&self, pos: Pos) -> f64 {
    self.humidity_map.generate(pos.x as f64, pos.z as f64) * 0.5 + 0.5
  }

  fn variance(&self, pos: Pos) -> u32 {
    self.variance_map.generate(pos.x as f64 / 128.0, pos.z as f64 / 128.0)
  }
}
