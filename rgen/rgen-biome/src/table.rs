use crate::{biome::*, builder::BiomeBuilder};

pub type BiomeList = Vec<BiomeBuilder>;
pub type BiomeTable = [[BiomeList; 8]; 12];

type BiomeFnCategory = &'static [(f64, &'static str, BiomeFn)];
type BiomeFnTable = &'static [&'static [BiomeFnCategory]];

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

// === Biome categories ===

const BLANK: BiomeFnCategory = &[b!(1.0, dry_wood)];
const SEA: BiomeFnCategory = &[b!(1.0, blank)];
const RIVER: BiomeFnCategory = &[b!(1.0, river)];

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
const TROPIC_BEACH: BiomeFnCategory =
  &[b!(1.0, sand_beach), b!(1.0, chaparral_beach), b!(1.0, jungle_beach), b!(1.0, palm_beach)];

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
const SUB_ARCTIC: BiomeFnCategory =
  &[b!(1.0, fir_grove), b!(1.0, spruce_grove) /* b!(1.0, seasonal_woodland) */];
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
  b!(1.0, blank),
  // b!(1.0, charred_woodland),
  // b!(1.0, charred_birch_woodland),
  // b!(1.0, deadwood),
  b!(1.0, dry_grassy_wood),
  b!(1.0, dry_wood),
];
const SAVANNA: BiomeFnCategory = &[
  //b!(1.0, blank),
  //b!(1.0, dead_wood),
  b!(1.0, wooded_savanna),
  b!(1.0, thorn_wood),
];
const HOT_DESERT: BiomeFnCategory = &[
  b!(1.0, blank),
  // b!(1.0, flat_desert),
  // b!(1.0, dune_sea),
  // b!(1.0, stone_desert),
  // b!(1.0, red_desert),
  // b!(1.0, petrified_forest),
  // b!(1.0, bone_lands),
];
const BAD_LANDS: BiomeFnCategory = &[
  b!(1.0, blank),
  // b!(1.0, boneland),
  // b!(1.0, bad_lands),
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
  // b!(1.0, cherry_blossom_wood),
  // b!(1.0, woodland),
  b!(1.0, birch_woodland),
  // b!(1.0, seasonal_woodland),
  // b!(1.0, lavedar_grove),
  // b!(1.0, field),
  // b!(1.0, aspenwood),
  // b!(1.0, elder_birch_woodland),
  // b!(1.0, valcano_growth),
];
const MEDITERANEAN: BiomeFnCategory = &[
  b!(1.0, blank),
  // b!(1.0, chaparral_flats),
  // b!(1.0, redwood_grove),
  // b!(1.0, open_plain),
  // b!(1.0, sunflower_plain),
  // b!(1.0, chaparral_woods),
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

// === Biome tables ===

const BLANK_TABLE: BiomeFnTable = &[&[BLANK]];
const SEA_TABLE: BiomeFnTable = &[&[SEA]];
const RIVER_TABLE: BiomeFnTable = &[&[RIVER]];

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

#[rustfmt::skip]
const STANDARD_TABLE: BiomeFnTable = &[
  &[ICE_CAP, TUNDRA, TUNDRA, DRY_TEMPERATE, SAVANNA, HOT_DESERT, BAD_LANDS, BAD_LANDS],
  &[ICE_CAP, TUNDRA, TUNDRA, DRY_TEMPERATE, SAVANNA, HOT_DESERT, HOT_DESERT, BAD_LANDS],
  &[ICE_CAP, TUNDRA, SUB_ARCTIC, DRY_TEMPERATE, DRY_TEMPERATE, SAVANNA, HOT_DESERT, HOT_DESERT],
  &[ICE_CAP, TUNDRA, SUB_ARCTIC, DRY_TEMPERATE, WARM_TEMPERATE, MEDITERANEAN, SAVANNA, HOT_DESERT],
  &[ICE_CAP, TUNDRA, SUB_ARCTIC, COOL_TEMPERATE, WARM_TEMPERATE, MEDITERANEAN, MEDITERANEAN, SAVANNA],
  &[ICE_CAP, TUNDRA, SUB_ARCTIC, COOL_TEMPERATE, WARM_TEMPERATE, WARM_TEMPERATE, MEDITERANEAN, MEDITERANEAN],
  &[ICE_CAP, TUNDRA, SUB_ARCTIC, COOL_TEMPERATE, WARM_TEMPERATE, WARM_TEMPERATE, WARM_TEMPERATE, MEDITERANEAN],
  &[ICE_CAP, TUNDRA, SUB_ARCTIC, COOL_TEMPERATE, WARM_TEMPERATE, WARM_TEMPERATE, WET_TEMPERATE, WET_TEMPERATE],
  &[ICE_CAP, TUNDRA, SUB_ARCTIC, COOL_TEMPERATE, WARM_TEMPERATE, WET_TEMPERATE, WET_TEMPERATE, MONSOON],
  &[TUNDRA, SUB_ARCTIC, COOL_TEMPERATE, COOL_TEMPERATE, WET_TEMPERATE, WET_TEMPERATE, MONSOON, TROPICAL],
  &[TUNDRA, SUB_ARCTIC, COOL_TEMPERATE, WET_TEMPERATE, WET_TEMPERATE, MONSOON, TROPICAL, TROPICAL],
  &[TUNDRA, SUB_ARCTIC, COOL_TEMPERATE, WET_TEMPERATE, WET_TEMPERATE, MONSOON, TROPICAL, TROPICAL],
];

pub struct Tables {
  pub blank_table:    BiomeTable,
  pub sea_table:      BiomeTable,
  pub beach_table:    BiomeTable,
  pub standard_table: BiomeTable,
  pub valley_table:   BiomeTable,
  pub river_table:    BiomeTable,
}

impl Tables {
  pub fn new(ctx: &IdContext) -> Tables {
    Tables {
      blank_table:    table(ctx, BLANK_TABLE),
      sea_table:      table(ctx, SEA_TABLE),
      beach_table:    table(ctx, BEACH_TABLE),
      standard_table: table(ctx, STANDARD_TABLE),
      valley_table:   table(ctx, VALLEY_TABLE),
      river_table:    table(ctx, RIVER_TABLE),
    }
  }
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
