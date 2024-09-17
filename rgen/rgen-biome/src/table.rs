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
        let (GeographicType::$geographic:ident, ClimateType::$climate:ident) = &[$($biome:expr),* $(,)?];
      )*
    }
  ) => {
    impl CompositionLookup {
      pub fn new(ctx: &IdContext) -> CompositionLookup {
        let mut lookup = HashMap::new();
        $(
          lookup.insert((GeographicType::$geographic, ClimateType::$climate), composition(ctx, &[$($biome),*]));
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
  }
);

// let (GeographicType::River, ClimateType::WarmTemperate) = &[
//   b!(1.0, river),
// ];
//
// let (GeographicType::River, ClimateType::WarmTemperate) = &[
//   b!(1.0, glacier), b!(1.0, rockies), b!(1.0, broken_glacier)];

const FROZEN_VALLEY: BiomeFnCategory =
  &[b!(1.0, glacier), b!(1.0, rockies), b!(1.0, broken_glacier)];
const BOG: BiomeFnCategory =
  &[b!(1.0, bog), b!(1.0, cold_bog), b!(1.0, fall_bog), b!(1.0, conifer_swamp)];
const ROCKY_VALLEY: BiomeFnCategory =
  &[b!(1.0, crag), b!(1.0, snowy_crag) /* , rocky_cedar */];
const COOL_VALLEY: BiomeFnCategory =
  &[b!(1.0, crag) /* , fir_wood, boreal_forest, cedar_wood, rocky_spruce */];
const SWAMP: BiomeFnCategory =
  &[b!(1.0, plains) /* cherry_blossom_grove, woodland, lavendar_grove, woodland, aspenwood */];
const DRY_RIVER: BiomeFnCategory = &[b!(1.0, plains) /* swamp, mangrove_woods */];
const WARM_VALLEY: BiomeFnCategory = &[b!(1.0, plains)];
const HOT_SWAMP: BiomeFnCategory = &[b!(1.0, plains)];
const TROPIC_SWAMP: BiomeFnCategory = &[b!(1.0, plains)];

const COLD_BEACH: BiomeFnCategory = &[b!(1.0, snowy_shores), b!(1.0, snowy_rock)];
const COOL_BEACH: BiomeFnCategory = &[
  b!(1.0, ancient_shores),
  b!(1.0, mossy_shores),
  b!(1.0, dry_shores),
  b!(1.0, bare_rock),
  b!(1.0, wet_rock),
];
const BEACH: BiomeFnCategory =
  &[b!(65.0, sand_beach), b!(5.0, monument_beach), b!(31.0, palm_beach)];
const DRY_BEACH: BiomeFnCategory = &[
  b!(1.0, sand_beach),
  b!(1.0, monument_beach),
  b!(1.0, red_sand_beach),
  b!(1.0, red_monument_beach),
  b!(1.0, dry_shores),
  b!(1.0, chaparral_beach),
];
const TROPIC_BEACH: BiomeFnCategory = &[
  b!(1.0, sand_beach),
  b!(1.0, chaparral_beach),
  b!(1.0, jungle_beach),
  b!(1.0, palm_beach),
  //bladjlaf
];

const ICE_CAP: BiomeFnCategory = &[
  b!(1.0, ice_spikes),
  b!(1.0, broken_glacier),
  b!(1.0, glacier),
  b!(1.0, snowy_plains),
  b!(1.0, rocky_plains),
];
const TUNDRA: BiomeFnCategory = &[
  b!(1.0, snowy_plains),
  b!(1.0, rocky_plains),
  b!(1.0, frozen_meadow),
  b!(1.0, frozen_desert),
  // b!(1.0, snowy_fir_wood),
  // b!(1.0, snowy_spruce_wood),
  b!(1.0, snowy_woodland),
];
const SUB_ARCTIC: BiomeFnCategory = &[
  b!(1.0, fir_grove),
  b!(1.0, spruce_grove),
  //b!(1.0, seasonal_woodland)
];
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

// === Biome tables ===

const VALLEY_TABLE: BiomeFnTable = &[
  &[ROCKY_VALLEY, ROCKY_VALLEY, ROCKY_VALLEY, COOL_VALLEY, SWAMP, SWAMP, DRY_RIVER, DRY_RIVER],
  &[FROZEN_VALLEY, ROCKY_VALLEY, COOL_VALLEY, COOL_VALLEY, WARM_VALLEY, SWAMP, SWAMP, DRY_RIVER],
  &[FROZEN_VALLEY, BOG, COOL_VALLEY, WARM_VALLEY, WARM_VALLEY, WARM_VALLEY, SWAMP, SWAMP],
  &[FROZEN_VALLEY, BOG, COOL_VALLEY, WARM_VALLEY, WARM_VALLEY, SWAMP, SWAMP, HOT_SWAMP],
  &[FROZEN_VALLEY, BOG, BOG, COOL_VALLEY, SWAMP, SWAMP, HOT_SWAMP, HOT_SWAMP],
  &[BOG, BOG, BOG, SWAMP, SWAMP, HOT_SWAMP, HOT_SWAMP, TROPIC_SWAMP],
];

const BEACH_TABLE: BiomeFnTable = &[
  &[COLD_BEACH, COLD_BEACH, BEACH, BEACH, BEACH, DRY_BEACH, DRY_BEACH, DRY_BEACH],
  &[COLD_BEACH, COLD_BEACH, COOL_BEACH, BEACH, BEACH, BEACH, DRY_BEACH, DRY_BEACH],
  &[COLD_BEACH, COOL_BEACH, COOL_BEACH, COOL_BEACH, BEACH, BEACH, BEACH, BEACH],
  &[COLD_BEACH, COOL_BEACH, COOL_BEACH, COOL_BEACH, BEACH, BEACH, BEACH, BEACH],
  &[COLD_BEACH, COOL_BEACH, COOL_BEACH, BEACH, BEACH, BEACH, BEACH, TROPIC_BEACH],
  &[COOL_BEACH, COOL_BEACH, COOL_BEACH, BEACH, BEACH, BEACH, TROPIC_BEACH, TROPIC_BEACH],
];

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

const CAVE_TABLE: BiomeFnTable = &[&[CAVE]];

pub fn build(ctx: &IdContext) -> BiomeTable { todo!() }

fn composition(ctx: &IdContext, biome: BiomeFnCategory) -> BiomeComposition {
  biome.iter().map(|(rarity, name, f)| BiomeBuilder::build(name, ctx, *rarity, *f)).collect()
}

fn table(ctx: &IdContext, table: BiomeFnTable) -> BiomeTable {
  let table = match table.len() {
    1 => vec![&table[0]; 12],
    6 => table.iter().flat_map(|row| [row, row]).collect::<Vec<_>>(),
    12 => table.iter().collect::<Vec<_>>(),
    _ => panic!("table must be 1, 6 or 12 rows"),
  };

  let table = table
    .iter()
    .map(|row| {
      let row = match row.len() {
        1 => vec![&row[0]; 8],
        8 => row.iter().collect::<Vec<_>>(),
        _ => panic!("row must be 1 or 8 items"),
      };

      let items = row
        .iter()
        .map(|&biomes| {
          if biomes.is_empty() {
            panic!("biome category cannot be empty");
          } else {
            biomes
              .iter()
              .map(|(rarity, name, f)| BiomeBuilder::build(name, ctx, *rarity, *f))
              .collect::<BiomeList>()
          }
        })
        .collect::<Vec<_>>();
      match items.try_into() {
        Ok(v) => v,
        Err(_) => unreachable!(),
      }
    })
    .collect::<Vec<_>>();

  match table.try_into() {
    Ok(v) => v,
    Err(_) => unreachable!(),
  }
}

#[cfg(test)]
mod tests {
  use rgen_base::{Biomes, Blocks};

  use super::*;

  #[test]
  fn biomes_can_build() {
    let ctx = IdContext { biomes: &Biomes::test_blocks(), blocks: &Blocks::test_blocks() };
    Tables::new(&ctx);
  }
}
