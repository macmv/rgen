use std::collections::HashMap;

use crate::{biome::*, builder::BiomeBuilder};

pub type BiomeList = Vec<BiomeBuilder>;
pub type BiomeTable = [[BiomeList; 8]; 12];

type BiomeFnCategory = &'static [(u32, &'static str, BiomeFn)];
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
        CompositionLookup { blank: composition(ctx, &[b!(1, blank)]), lookup }
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
    //// CLIMATE: WARMTEMPERATE
    let (GeographicType::Standard, ClimateType::WarmTemperate) = &[
      b!(6, woodland),
      b!(1, birch_woodland),
      b!(2, aspen_wood),
      b!(5, birch_woodland),
      b!(4, cherry_blossom_grove),
      b!(2, volcano_growth),
    ];

    let (GeographicType::River, ClimateType::WarmTemperate) = &[
      b!(6, woodland_river),
      b!(1, birch_river),
      b!(2, birch_river),
      b!(5, birch_river),
      b!(4, cherry_blossom_river),
      b!(2, volcano_river),
    ];

    let (GeographicType::Canyon, ClimateType::WarmTemperate) = &[
      b!(6, woodland),
      b!(1, birch_woodland),
      b!(2, aspen_wood),
      b!(5, birch_woodland),
      b!(4, cherry_blossom_grove),
      b!(2, volcano_growth),
    ];

    let (GeographicType::Standard, ClimateType::IceCap) =
      &[b!(7, ice_spikes), b!(5, boulder_field), b!(8, glacier)];

    // The river biome is now the valley  let (GeographicType::River,
    // ClimateType::IceCap) = &[b!(20, glacier)];

    let (GeographicType::River, ClimateType::IceCap) = &[b!(20, deep_snow_beach)]; //&[b!(20, hard_frozen_river)];
    let (GeographicType::Canyon, ClimateType::IceCap) = &[b!(20, deep_snow_beach)]; //&[b!(20, hard_frozen_river)];

    let (GeographicType::Hills, ClimateType::IceCap) = &[b!(20, alps)];

    let (GeographicType::Mountains, ClimateType::IceCap) = &[b!(10, alps), b!(10, frozen_peak)];
    //deep_snow_beach
    let (GeographicType::Beach, ClimateType::IceCap) =
      &[b!(7, ice_spike_beach), b!(5, deep_snow_beach), b!(8, deep_snow_beach)];
    //

    // SUB ARCTIC,
    let (GeographicType::River, ClimateType::SubArctic) =
      &[b!(10, fir_river), b!(10, spruce_river)]; //&[b!(20, hard_frozen_river)];
    let (GeographicType::Canyon, ClimateType::SubArctic) =
      &[b!(10, windswept_fir_grove), b!(10, windswept_spruce_grove)];
    //&[b!(10, fir_grove), b!(10, spruce_grove)];

    let (GeographicType::Standard, ClimateType::SubArctic) =
      &[b!(10, fir_grove), b!(10, spruce_grove)];

    let (GeographicType::Hills, ClimateType::SubArctic) = &[
      b!(7, windswept_fir_grove),
      b!(4, windswept_hill),
      b!(2, crag),
      b!(7, windswept_spruce_grove),
    ];

    let (GeographicType::Mountains, ClimateType::SubArctic) = &[b!(20, snowy_peak)];

    let (GeographicType::Beach, ClimateType::SubArctic) = &[
      b!(2, fir_grove), // maybe a fir beach in the future
      b!(3, mossy_shores),
      b!(10, tiaga_beach),
      b!(3, mossy_shores), // used to be wet rock
      b!(2, spruce_grove), // maybe a spruce beach in the future
    ];

    // let (GeographicType::Valley, ClimateType::WarmTemperate) = &[
    //   //b!(6, woodland_hills),
    //   //b!(1, birch_woodland_hills),
    //   b!(2, aspen_wood),
    //   //b!(5, birch_woodland_hills),
    //   b!(4, cherry_blossom_wood),
    //   //b!(2, warm_volcano),
    // ];

    let (GeographicType::Standard, ClimateType::CoolTemperate) = &[b!(1, glacier)];
    let (GeographicType::Standard, ClimateType::Tundra) =
      &[b!(1, bog), b!(1, cold_bog), b!(1, fall_bog), b!(1, conifer_swamp)];
    let (GeographicType::Hills, ClimateType::Tundra) =
      &[b!(1, crag), b!(1, snowy_crag) /* , rocky_cedar */];

    let (GeographicType::Beach, ClimateType::CoolTemperate) = &[
      b!(1, ancient_shores),
      b!(1, mossy_shores),
      b!(1, dry_shores),
      b!(1, bare_rock),
      b!(1, wet_rock),
    ];
    let (GeographicType::Beach, ClimateType::WarmTemperate) =
      &[b!(65, sand_beach), b!(5, monument_beach), b!(31, palm_beach)];
    let (GeographicType::Beach, ClimateType::DryTemperate) = &[
      b!(1, sand_beach),
      b!(1, monument_beach),
      b!(1, red_sand_beach),
      b!(1, red_monument_beach),
      b!(1, dry_shores),
      b!(1, chaparral_beach),
    ];
    let (GeographicType::Beach, ClimateType::Tropical) = &[
      b!(1, sand_beach),
      b!(1, chaparral_beach),
      b!(1, jungle_beach),
      b!(1, palm_beach),
      //bladjlaf
    ];

    let (GeographicType::Mountains, ClimateType::Tundra) = &[
      b!(1, snowy_plains),
      b!(1, frozen_meadow),
      b!(1, frozen_desert),
      // b!(1, snowy_fir_wood),
      // b!(1, snowy_spruce_wood),
      b!(1, snowy_woodland),
    ];
  }
);

const COOL_TEMPERATE: BiomeFnCategory = &[
  // b!(1, boreal_forest),
  // b!(1, ceader_wood),
  // b!(1, fir_wood),
  b!(1, crag),
  // b!(1, spruce_tiga),
  // b!(1, twisted_spruce_wood),
  // b!(1, rocky_spruce),
];
const DRY_TEMPERATE: BiomeFnCategory = &[
  // b!(1, charred_woodland),
  // b!(1, charred_birch_woodland),
  // b!(1, deadwood),
  b!(1, dry_grassy_wood),
  b!(1, dry_wood),
  b!(1, thorn_wood),
  b!(1, chaparral_woods),
];
const SAVANNA: BiomeFnCategory = &[
  //b!(1, dead_wood),
  b!(1, wooded_savanna),
  b!(1, open_savanna),
  b!(1, thorn_wood),
  b!(1, chaparral_woods),
];
const HOT_DESERT: BiomeFnCategory = &[
  // b!(1, blank),
  b!(1, flat_desert),
  b!(1, lush_desert),
  b!(1, dune_sea),
  // b!(1, stone_desert),
  // b!(1, red_desert),
  // b!(1, petrified_forest),
  // b!(1, bone_lands),
];
const BAD_LANDS: BiomeFnCategory = &[
  // b!(1, boneland),
  b!(1, bad_lands),
  // b!(1, stone_desert),
];
const WET_TEMPERATE: BiomeFnCategory = &[
  b!(1, blank),
  // b!(1, temperate_rain_forest),
  // b!(1, cedar_rock_wood),
  // b!(1, cedar_wood),
  // b!(1, elder_woodland),
  // b!(1, weeping_birchwood),
  // b!(1, lush_desert),
  // b!(1, fungal_wood),
  // b!(1, seasonal_woodland),
];
const WARM_TEMPERATE: BiomeFnCategory = &[
  // b!(1, elder_woodland),
  // b!(1, weeping_birchwood),
  // b!(1, lush_desert),
  b!(1, cherry_blossom_wood),
  b!(1, cherry_blossom_grove),
  b!(80, woodland),
  b!(1, birch_woodland),
  // b!(1, seasonal_woodland),
  b!(1, lavender_grove),
  // b!(1, field),
  b!(1, aspen_wood),
  // b!(1, elder_birch_woodland),
  b!(1, volcano_growth),
];
const MEDITERANEAN: BiomeFnCategory = &[
  //b!(1, blank),
  b!(1, chaparral_flats),
  b!(1, redwood_grove),
  b!(1, open_plain),
  b!(1, sunflower_plain),
  b!(1, chaparral_woods),
];
const MONSOON: BiomeFnCategory = &[
  b!(1, blank),
  // b!(1, mangrove_woods),
  // b!(1, light_jungle)
];
const TROPICAL: BiomeFnCategory = &[
  b!(1, blank),
  // b!(1, deep_jungle),
  // b!(1, light_jungle),
  // b!(1, bamboo_jungle)
];
const CAVE: BiomeFnCategory = &[b!(1, cave), b!(1, lush_cave)];

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
