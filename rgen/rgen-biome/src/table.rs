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
  pub blank: BiomeComposition,
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

// This branch intentionally generates only Mediterranean biomes. Climate and
// geographic enums remain shared with the terrain lookup and viewer.
biome_categories!(
  fn build() {
    let (GeographicType::Standard, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland),
      b!(45, chaparral_flats),
      b!(35, open_plains_standard),
    ];
    let (GeographicType::River, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland_river),
      b!(45, chaparral_flats_river),
      b!(35, open_plains_river),
    ];
    let (GeographicType::Canyon, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland_canyon),
      b!(45, chaparral_flats_canyon),
      b!(35, open_plains_canyon),
    ];
    let (GeographicType::Hills, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland_hills),
      b!(45, chaparral_flats_hills),
      b!(35, open_plains_hills),
    ];
    let (GeographicType::Mountains, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland_mountains),
      b!(45, chaparral_flats_mountains),
      b!(35, open_plains_mountains),
    ];
    let (GeographicType::Beach, ClimateType::Mediterranean) = &[
      b!(20, lebanon_cedar_woodland_beach),
      b!(80, coastal_dunes),
    ];
    let (GeographicType::Delta, ClimateType::Mediterranean) = &[
      b!(20, riparian_woodland),
      b!(80, salt_marsh),
    ];
  }
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GeographicType {
  MushroomIsland,
  Ocean,
  Beach,
  Delta,
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
  DebugClimate,
  GeoTest,
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
