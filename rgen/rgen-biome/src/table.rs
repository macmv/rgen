use std::collections::HashMap;

use crate::{biome::*, builder::BiomeBuilder};

type BiomeFnCategory = &'static [(u32, &'static str, BiomeFn)];

pub type BiomeComposition = Vec<BiomeBuilder>;
type ClimateTable = &'static [&'static [ClimateType]];

macro_rules! b {
  ($rarity:expr, $f:expr) => {
    ($rarity, stringify!($f), $f as BiomeFn)
  };
}

pub struct CompositionLookup {
  pub blank:  BiomeComposition,
  pub lookup: HashMap<(GeographicType, ClimateType), BiomeComposition>,
}

macro_rules! biome_categories {
  (
    fn build() {
      $(
        let ($geographic:expr, $climate:expr) = &[$($biome:expr),* $(,)?];
      )*
    }
  ) => {
    impl CompositionLookup {
      pub fn new(seed: u64) -> CompositionLookup {
        let mut lookup = HashMap::new();
        $(
          if lookup.insert(($geographic, $climate), composition(seed, &[$($biome),*])).is_some() {
            panic!("Duplicate biome for {:?}, {:?}", $geographic, $climate);
          }
        )*
        CompositionLookup { blank: composition(seed, &[b!(1, blank)]), lookup }
      }
    }
  };
}

impl CompositionLookup {
  pub fn choose(&self, geographic: GeographicType, climate: ClimateType) -> &BiomeComposition {
    self.lookup.get(&(geographic, climate)).unwrap_or(&self.blank)
  }
}

// === Biome categories ===
biome_categories!(
  fn build() {
    // IceCap vvv

    let (GeographicType::Standard, ClimateType::IceCap) =
      &[b!(7, ice_spikes), b!(5, boulder_field), b!(8, glacier)];

    let (GeographicType::River, ClimateType::IceCap) = &[b!(20, deep_snow_beach)];

    let (GeographicType::Canyon, ClimateType::IceCap) = &[b!(20, deep_snow_beach)];

    let (GeographicType::Hills, ClimateType::IceCap) = &[b!(20, alps)];

    let (GeographicType::Mountains, ClimateType::IceCap) = &[b!(10, alps), b!(10, frozen_peak)];

    let (GeographicType::Beach, ClimateType::IceCap) =
      &[b!(7, ice_spike_beach), b!(13, deep_snow_beach)];

    // SubArtic vvv

    let (GeographicType::Standard, ClimateType::SubArctic) =
      &[b!(10, fir_grove), b!(10, spruce_grove)];

    let (GeographicType::River, ClimateType::SubArctic) =
      &[b!(10, fir_river), b!(10, spruce_river)];

    let (GeographicType::Canyon, ClimateType::SubArctic) =
      &[b!(10, windswept_fir_grove), b!(10, windswept_spruce_grove)];

    let (GeographicType::Hills, ClimateType::SubArctic) =
      &[b!(7, windswept_fir_grove), b!(4, windswept_hill)];

    let (GeographicType::Mountains, ClimateType::SubArctic) =
      &[b!(7, windswept_fir_grove), b!(6, windswept_hill)];

    let (GeographicType::Beach, ClimateType::SubArctic) = &[b!(2, fir_grove), b!(3, mossy_shores)];

    // WarmTemperate vvv

    let (GeographicType::Standard, ClimateType::WarmTemperate) =
      &[b!(10, woodland), b!(3, birch_woodland), b!(2, aspen_wood), b!(5, birch_woodland)];

    let (GeographicType::River, ClimateType::WarmTemperate) =
      &[b!(10, woodland_river), b!(3, birch_river), b!(2, birch_river), b!(5, birch_river)];

    let (GeographicType::Canyon, ClimateType::WarmTemperate) =
      &[b!(10, woodland), b!(3, birch_woodland), b!(2, aspen_wood), b!(5, birch_woodland)];

    let (GeographicType::Hills, ClimateType::WarmTemperate) =
      &[b!(10, woodland), b!(3, birch_woodland), b!(2, aspen_wood), b!(5, birch_woodland)];

    let (GeographicType::Mountains, ClimateType::WarmTemperate) =
      &[b!(10, woodland), b!(3, birch_woodland), b!(2, aspen_wood), b!(5, birch_woodland)];

    let (GeographicType::Beach, ClimateType::WarmTemperate) =
      &[b!(10, woodland), b!(3, birch_woodland), b!(2, aspen_wood), b!(5, birch_woodland)];

    // Tropical vvv

    let (GeographicType::Standard, ClimateType::Tropical) = &[b!(20, terraced_jungle_wood)];

    let (GeographicType::River, ClimateType::Tropical) = &[b!(20, terraced_jungle_wood)];

    let (GeographicType::Canyon, ClimateType::Tropical) = &[b!(20, terraced_jungle_wood)];

    let (GeographicType::Hills, ClimateType::Tropical) = &[b!(20, blank)];

    let (GeographicType::Mountains, ClimateType::Tropical) = &[b!(20, blank)];

    let (GeographicType::Beach, ClimateType::Tropical) = &[b!(20, blank)];
  }
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GeographicType {
  MushroomIsland,
  Ocean,
  Beach,
  Canyon,
  River,
  Standard,
  Hills,
  Mountains,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClimateType {
  IceCap,
  Tundra,
  SubArctic,
  CoolTemperate,
  DryTemperate,
  WarmTemperate,
  WetTemperate,
  Mediterranean,
  Monsoon,
  Savanna,
  HotDesert,
  BadLands,
  Tropical,
}

use ClimateType::*;
#[rustfmt::skip]
pub const CLIMATE_TABLE: ClimateTable = &[
  &[IceCap, Tundra, Tundra, DryTemperate, Savanna, HotDesert, BadLands, BadLands],
  &[IceCap, Tundra, Tundra, DryTemperate, Savanna, HotDesert, HotDesert, BadLands],
  &[IceCap, Tundra, SubArctic, DryTemperate, DryTemperate, Savanna, HotDesert, HotDesert],
  &[IceCap, Tundra, SubArctic, DryTemperate, WarmTemperate, Mediterranean, Savanna, HotDesert],
  &[IceCap, Tundra, SubArctic, CoolTemperate, WarmTemperate, Mediterranean, Mediterranean, Savanna],
  &[IceCap, Tundra, SubArctic, CoolTemperate, WarmTemperate, WarmTemperate, Mediterranean, Mediterranean],
  &[IceCap, Tundra, SubArctic, CoolTemperate, WarmTemperate, WarmTemperate, WarmTemperate, Mediterranean],
  &[IceCap, Tundra, SubArctic, CoolTemperate, WarmTemperate, WarmTemperate, WetTemperate, WetTemperate],
  &[IceCap, Tundra, SubArctic, CoolTemperate, WarmTemperate, WetTemperate, WetTemperate, Monsoon],
  &[Tundra, SubArctic, CoolTemperate, CoolTemperate, WetTemperate, WetTemperate, Monsoon, Tropical],
  &[Tundra, SubArctic, CoolTemperate, WetTemperate, WetTemperate, Monsoon, Tropical, Tropical],
  &[Tundra, SubArctic, CoolTemperate, WetTemperate, WetTemperate, Monsoon, Tropical, Tropical],
];

fn composition(seed: u64, biome: BiomeFnCategory) -> BiomeComposition {
  biome.iter().map(|(rarity, name, f)| BiomeBuilder::build(seed, name, *rarity, *f)).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn composition() { CompositionLookup::new(0); }
}
