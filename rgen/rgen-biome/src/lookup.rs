use std::{cell::RefCell, num::NonZero};

use lru::LruCache;
use rgen_base::Pos;
use rgen_placer::noise::NoiseGenerator;

use crate::{
  WorldBiomes,
  builder::BiomeBuilder,
  table::{CLIMATE_TABLE, ClimateType, GeographicType},
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
    let continentalness = self.continentalness_category(pos);
    let peaks_valleys = self.peaks_valleys_category(pos);
    let erosion = self.erosion_category(pos);

    use ContinentalnessCategory::*;
    use PeaksValleysCategory::*;

    match (continentalness, peaks_valleys, erosion) {
      (_, _, 7..) => unreachable!(), // Erosion is 0..=6

      (MushroomIsland, _, _) => GeographicType::Ocean,
      (Sea, _, _) => GeographicType::Ocean,
      (Coast, _, _) => GeographicType::Beach,

      (NearInland, Valley, 0) => GeographicType::Standard,
      (NearInland, Valley, 1..=6) => GeographicType::River,
      (NearInland, LowSlice, 0..=5) => GeographicType::Standard,
      (NearInland, LowSlice, 6) => GeographicType::River,
      (NearInland, MidSlice, 0) => GeographicType::Mountains,
      (NearInland, MidSlice, 1..=3) => GeographicType::Hills,
      (NearInland, MidSlice, 4..=5) => GeographicType::Standard,
      (NearInland, MidSlice, 6) => GeographicType::River,
      (NearInland, HighSlice, 0..=1) => GeographicType::Mountains,
      (NearInland, HighSlice, 2..=3) => GeographicType::Hills,
      (NearInland, HighSlice, 4..=6) => GeographicType::Standard,
      (NearInland, Peak, 0..=1) => GeographicType::Mountains,
      (NearInland, Peak, 2..=5) => GeographicType::Hills,
      (NearInland, Peak, 6) => GeographicType::Standard,

      (MidInland, Valley, 0..=3) => GeographicType::Standard,
      (MidInland, Valley, 4..=6) => GeographicType::Canyon,
      (MidInland, LowSlice, 0..=3) => GeographicType::Hills,
      (MidInland, LowSlice, 4..=5) => GeographicType::Standard,
      (MidInland, LowSlice, 6) => GeographicType::Canyon,
      (MidInland, MidSlice, 0..=1) => GeographicType::Mountains,
      (MidInland, MidSlice, 2..=3) => GeographicType::Hills,
      (MidInland, MidSlice, 4..=5) => GeographicType::Standard,
      (MidInland, MidSlice, 6) => GeographicType::Canyon,
      (MidInland, HighSlice, 0..=1) => GeographicType::Mountains,
      (MidInland, HighSlice, 2..=3) => GeographicType::Hills,
      (MidInland, HighSlice, 4..=6) => GeographicType::Standard,
      (MidInland, Peak, 0..=1) => GeographicType::Mountains,
      (MidInland, Peak, 2..=3) => GeographicType::Hills,
      (MidInland, Peak, 4..=6) => GeographicType::Standard,

      (FarInland, Valley, 0..=3) => GeographicType::Standard,
      (FarInland, Valley, 4..=6) => GeographicType::Canyon,
      (FarInland, LowSlice, 0..=3) => GeographicType::Hills,
      (FarInland, LowSlice, 4..=5) => GeographicType::Standard,
      (FarInland, LowSlice, 6) => GeographicType::Canyon,
      (FarInland, MidSlice, 0..=2) => GeographicType::Hills,
      (FarInland, MidSlice, 3..=6) => GeographicType::Standard,
      (FarInland, HighSlice, 0..=1) => GeographicType::Mountains,
      (FarInland, HighSlice, 2..=3) => GeographicType::Hills,
      (FarInland, HighSlice, 4..=6) => GeographicType::Standard,
      (FarInland, Peak, 0..=2) => GeographicType::Mountains,
      (FarInland, Peak, 3..=4) => GeographicType::Hills,
      (FarInland, Peak, 5..=6) => GeographicType::Standard,
    }
  }

  pub fn climate_type(&self, pos: Pos) -> ClimateType {
    let temperature = self.temperature(pos);
    let humidity = self.humidity(pos);

    CLIMATE_TABLE[(temperature * CLIMATE_TABLE.len() as f64) as usize]
      [(humidity * CLIMATE_TABLE[0].len() as f64) as usize]
  }

  pub fn choose_surface_biome(&self, pos: Pos) -> &BiomeBuilder {
    let key = self.choose_surface_biome_cached(pos);
    self.choose_surface_biome_from_key(key)
  }

  fn choose_surface_biome_from_key(&self, key: BiomeKey) -> &BiomeBuilder {
    if self.biome_override {
      return &self.composition_lookup.blank[0];
    }

    let biomes = self.composition_lookup.choose(key.geographic, key.climate);

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
    thread_local! {
      static CACHE: RefCell<LruCache<(i32, i32), BiomeKey>> = RefCell::new(LruCache::new(NonZero::new(256).unwrap()));
    }

    CACHE.with(|cache| {
      let mut cache = cache.borrow_mut();
      let key = (pos.x, pos.z);

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
      geographic: self.geographic_type(pos),
      climate:    /* self.climate_type(pos) */ ClimateType::Tropical, /* ClimateType option */
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

    // This is a bit odd, but its required for rivers. We need to modify the border
    // of the valley edge given the continentalness, so that river edges line up
    // correctly.
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
