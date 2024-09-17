use std::collections::HashMap;

use crate::{biome::*, builder::BiomeBuilder};

pub type BiomeList = Vec<BiomeBuilder>;
pub type BiomeTable = [[BiomeList; 8]; 12];

type BiomeFnCategory = &'static [(f64, &'static str, BiomeFn)];
type BiomeFnTable = &'static [&'static [BiomeFnCategory]];

pub type BiomeComposition = Vec<BiomeBuilder>;
type ClimateTable = &'static [&'static [ClimateType]];

// TODO: Need all of these biomes.
/*
const VALLEY_TABLE: [[&str; 6]; 7] = [
];
*/

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
      pub fn new(ctx: &IdContext) -> CompositionLookup {
        let mut lookup = HashMap::new();
        $(
          if lookup.insert(($geographic, $climate), composition(ctx, &[$($biome),*])).is_some() {
            panic!("Duplicate biome for {:?}, {:?}", $geographic, $climate);
          }
        )*
        CompositionLookup { blank: composition(ctx, &[b!(1.0, blank)]), lookup }
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
    let (GeographicType::Standard, ClimateType::WarmTemperate) = &[
      // b!(1.0, elder_woodland),
      // b!(1.0, weeping_birchwood),
      // b!(1.0, lush_desert),
      b!(1.0, cherry_blossom_wood),
      b!(1.0, cherry_blossom_grove),
      // b!(1.0, woodland),
      b!(1.0, birch_woodland),
      // b!(1.0, seasonal_woodland),
      b!(1.0, lavender_grove),
      // b!(1.0, field),
      b!(1.0, aspen_wood),
      // b!(1.0, elder_birch_woodland),
      b!(80.0, volcano_growth),
    ];

    let (GeographicType::River, ClimateType::WarmTemperate) = &[b!(1.0, river)];
    let (GeographicType::Standard, ClimateType::CoolTemperate) =
      &[b!(1.0, glacier), b!(1.0, rockies), b!(1.0, broken_glacier)];
    let (GeographicType::Standard, ClimateType::IceCap) =
      &[b!(1.0, glacier), b!(1.0, rockies), b!(1.0, broken_glacier)];
    let (GeographicType::Standard, ClimateType::Tundra) =
      &[b!(1.0, bog), b!(1.0, cold_bog), b!(1.0, fall_bog), b!(1.0, conifer_swamp)];
    let (GeographicType::Hills, ClimateType::Tundra) =
      &[b!(1.0, crag), b!(1.0, snowy_crag) /* , rocky_cedar */];

    let (GeographicType::Beach, ClimateType::SubArctic) =
      &[b!(1.0, snowy_shores), b!(1.0, snowy_rock)];
    let (GeographicType::Beach, ClimateType::CoolTemperate) = &[
      b!(1.0, ancient_shores),
      b!(1.0, mossy_shores),
      b!(1.0, dry_shores),
      b!(1.0, bare_rock),
      b!(1.0, wet_rock),
    ];
    let (GeographicType::Beach, ClimateType::WarmTemperate) =
      &[b!(65.0, sand_beach), b!(5.0, monument_beach), b!(31.0, palm_beach)];
    let (GeographicType::Beach, ClimateType::DryTemperate) = &[
      b!(1.0, sand_beach),
      b!(1.0, monument_beach),
      b!(1.0, red_sand_beach),
      b!(1.0, red_monument_beach),
      b!(1.0, dry_shores),
      b!(1.0, chaparral_beach),
    ];
    let (GeographicType::Beach, ClimateType::Tropical) = &[
      b!(1.0, sand_beach),
      b!(1.0, chaparral_beach),
      b!(1.0, jungle_beach),
      b!(1.0, palm_beach),
      //bladjlaf
    ];

    let (GeographicType::Mountains, ClimateType::IceCap) = &[
      b!(1.0, ice_spikes),
      b!(1.0, broken_glacier),
      b!(1.0, glacier),
      b!(1.0, snowy_plains),
      b!(1.0, rocky_plains),
    ];
    let (GeographicType::Mountains, ClimateType::Tundra) = &[
      b!(1.0, snowy_plains),
      b!(1.0, rocky_plains),
      b!(1.0, frozen_meadow),
      b!(1.0, frozen_desert),
      // b!(1.0, snowy_fir_wood),
      // b!(1.0, snowy_spruce_wood),
      b!(1.0, snowy_woodland),
    ];
    let (GeographicType::Mountains, ClimateType::SubArctic) = &[
      b!(1.0, fir_grove),
      b!(1.0, spruce_grove),
      //b!(1.0, seasonal_woodland)
    ];
  }
);

const COOL_TEMPERATE: BiomeFnCategory = &[
  // b!(1.0, boreal_forest),
  // b!(1.0, ceader_wood),
  // b!(1.0, fir_wood),
  b!(1.0, crag),
  // b!(1.0, spruce_tiga),
  // b!(1.0, twisted_spruce_wood),
  // b!(1.0, rocky_spruce),
];
const DRY_TEMPERATE: BiomeFnCategory = &[
  // b!(1.0, charred_woodland),
  // b!(1.0, charred_birch_woodland),
  // b!(1.0, deadwood),
  b!(1.0, dry_grassy_wood),
  b!(1.0, dry_wood),
  b!(1.0, thorn_wood),
  b!(1.0, chaparral_woods),
];
const SAVANNA: BiomeFnCategory = &[
  //b!(1.0, dead_wood),
  b!(1.0, wooded_savanna),
  b!(1.0, open_savanna),
  b!(1.0, thorn_wood),
  b!(1.0, chaparral_woods),
];
const HOT_DESERT: BiomeFnCategory = &[
  // b!(1.0, blank),
  b!(1.0, flat_desert),
  b!(1.0, lush_desert),
  b!(1.0, dune_sea),
  // b!(1.0, stone_desert),
  // b!(1.0, red_desert),
  // b!(1.0, petrified_forest),
  // b!(1.0, bone_lands),
];
const BAD_LANDS: BiomeFnCategory = &[
  // b!(1.0, boneland),
  b!(1.0, bad_lands),
  // b!(1.0, stone_desert),
];
const WET_TEMPERATE: BiomeFnCategory = &[
  b!(1.0, blank),
  // b!(1.0, temperate_rain_forest),
  // b!(1.0, cedar_rock_wood),
  // b!(1.0, cedar_wood),
  // b!(1.0, elder_woodland),
  // b!(1.0, weeping_birchwood),
  // b!(1.0, lush_desert),
  // b!(1.0, fungal_wood),
  // b!(1.0, seasonal_woodland),
];
const WARM_TEMPERATE: BiomeFnCategory = &[
  // b!(1.0, elder_woodland),
  // b!(1.0, weeping_birchwood),
  // b!(1.0, lush_desert),
  b!(1.0, cherry_blossom_wood),
  b!(1.0, cherry_blossom_grove),
  b!(80.0, woodland),
  b!(1.0, birch_woodland),
  // b!(1.0, seasonal_woodland),
  b!(1.0, lavender_grove),
  // b!(1.0, field),
  b!(1.0, aspen_wood),
  // b!(1.0, elder_birch_woodland),
  b!(1.0, volcano_growth),
];
const MEDITERANEAN: BiomeFnCategory = &[
  //b!(1.0, blank),
  b!(1.0, chaparral_flats),
  b!(1.0, redwood_grove),
  b!(1.0, open_plain),
  b!(1.0, sunflower_plain),
  b!(1.0, chaparral_woods),
];
const MONSOON: BiomeFnCategory = &[
  b!(1.0, blank),
  // b!(1.0, mangrove_woods),
  // b!(1.0, light_jungle)
];
const TROPICAL: BiomeFnCategory = &[
  b!(1.0, blank),
  // b!(1.0, deep_jungle),
  // b!(1.0, light_jungle),
  // b!(1.0, bamboo_jungle)
];
const CAVE: BiomeFnCategory = &[b!(1.0, cave), b!(1.0, lush_cave)];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GeographicType {
  MushroomIsland,
  Ocean,
  Beach,
  Valley,
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

fn composition(ctx: &IdContext, biome: BiomeFnCategory) -> BiomeComposition {
  biome.iter().map(|(rarity, name, f)| BiomeBuilder::build(name, ctx, *rarity, *f)).collect()
}

#[cfg(test)]
mod tests {
  use rgen_base::{Biomes, Blocks};

  use super::*;

  #[test]
  fn composition() {
    let ctx = IdContext { biomes: &Biomes::test_blocks(), blocks: &Blocks::test_blocks() };
    CompositionLookup::new(&ctx);
  }
}
