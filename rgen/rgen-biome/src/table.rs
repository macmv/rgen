use crate::{biome::*, builder::BiomeBuilder};

pub type BiomeList = Vec<BiomeBuilder>;
pub type BiomeTable = [[BiomeList; 8]; 12];

type BiomeFnCategory = &'static [(f64, BiomeFn)];
type BiomeFnTable = &'static [&'static [BiomeFnCategory]];

// TODO: Need all of these biomes.
/*
const VALLEY_TABLE: [[&str; 6]; 7] = [
];
*/

// === Biome categories ===

const BLANK: BiomeFnCategory = &[(1.0, birch_woodland)];
const SEA: BiomeFnCategory = &[(1.0, blank)];

const FROZEN_VALLEY: BiomeFnCategory = &[(1.0, glacier), (1.0, rockies), (1.0, broken_glacier)];
const BOG: BiomeFnCategory = &[(1.0, bog), (1.0, cold_bog), (1.0, fall_bog), (1.0, conifer_swamp)];
const ROCKY_VALLEY: BiomeFnCategory = &[(1.0, crag), (1.0, snowy_crag) /* , rocky_cedar */];
const COOL_VALLEY: BiomeFnCategory =
  &[(1.0, crag) /* , fir_wood, boreal_forest, cedar_wood, rocky_spruce */];
const SWAMP: BiomeFnCategory =
  &[(1.0, plains) /* cherry_blossom_grove, woodland, lavendar_grove, woodland, aspenwood */];
const DRY_RIVER: BiomeFnCategory = &[(1.0, swamp) /* , mangrove_woods */];
const WARM_VALLEY: BiomeFnCategory = &[(1.0, plains)];
const HOT_SWAMP: BiomeFnCategory = &[(1.0, plains)];
const TROPIC_SWAMP: BiomeFnCategory = &[(1.0, plains)];

const COLD_BEACH: BiomeFnCategory = &[(1.0, snowy_shores), (1.0, snowy_rock)];
const COOL_BEACH: BiomeFnCategory = &[
  (1.0, ancient_shores),
  (1.0, mossy_shores),
  (1.0, dry_shores),
  (1.0, bare_rock),
  (1.0, wet_rock),
];
const BEACH: BiomeFnCategory = &[(65.0, sand_beach), (5.0, monument_beach), (31.0, palm_beach)];
const DRY_BEACH: BiomeFnCategory = &[
  (1.0, sand_beach),
  (1.0, monument_beach),
  (1.0, red_sand_beach),
  (1.0, red_monument_beach),
  (1.0, dry_shores),
  (1.0, chaparral_beach),
];
const TROPIC_BEACH: BiomeFnCategory =
  &[(1.0, sand_beach), (1.0, chaparral_beach), (1.0, jungle_beach), (1.0, palm_beach)];

const ICE_CAP: BiomeFnCategory = &[
  (1.0, ice_spikes),
  (1.0, broken_glacier),
  (1.0, glacier),
  (1.0, snowy_plains),
  (1.0, rocky_plains),
];
const TUNDRA: BiomeFnCategory = &[
  (1.0, snowy_plains),
  (1.0, rocky_plains),
  (1.0, frozen_meadow),
  (1.0, frozen_desert),
  // (1.0, snowy_fir_wood),
  // (1.0, snowy_spruce_wood),
  (1.0, snowy_woodland),
];
const SUB_ARCTIC: BiomeFnCategory =
  &[(1.0, fir_grove), (1.0, spruce_grove) /* (1.0, seasonal_woodland) */];
const COOL_TEMPERATE: BiomeFnCategory = &[
  // (1.0, boreal_forest),
  // (1.0, ceader_wood),
  // (1.0, fir_wood),
  (1.0, crag),
  // (1.0, spruce_tiga),
  // (1.0, twisted_spruce_wood),
  // (1.0, rocky_spruce),
];
const DRY_TEMPERATE: BiomeFnCategory = &[
  (1.0, blank),
  // (1.0, charred_woodland),
  // (1.0, charred_birch_woodland),
  // (1.0, deadwood),
  // (1.0, dry_grassy_wood),
  // (1.0, dry_wood),
];
const SAVANNA: BiomeFnCategory = &[
  (1.0, blank),
  // (1.0, dead_wood),
  // (1.0, wooded_savanna),
  // (1.0, thorn_wood)
];
const HOT_DESERT: BiomeFnCategory = &[
  (1.0, blank),
  // (1.0, flat_desert),
  // (1.0, dune_sea),
  // (1.0, stone_desert),
  // (1.0, red_desert),
  // (1.0, petrified_forest),
  // (1.0, bone_lands),
];
const BAD_LANDS: BiomeFnCategory = &[
  (1.0, blank),
  // (1.0, boneland),
  // (1.0, bad_lands),
  // (1.0, stone_desert),
];
const WET_TEMPERATE: BiomeFnCategory = &[
  (1.0, blank),
  // (1.0, temperate_rain_forest),
  // (1.0, cedar_rock_wood),
  // (1.0, cedar_wood),
  // (1.0, elder_woodland),
  // (1.0, weeping_birchwood),
  // (1.0, lush_desert),
  // (1.0, fungal_wood),
  // (1.0, seasonal_woodland),
];
const WARM_TEMPERATE: BiomeFnCategory = &[
  // (1.0, elder_woodland),
  // (1.0, weeping_birchwood),
  // (1.0, lush_desert),
  // (1.0, cherry_blossom_wood),
  // (1.0, woodland),
  (1.0, birch_woodland),
  // (1.0, seasonal_woodland),
  // (1.0, lavedar_grove),
  // (1.0, field),
  // (1.0, aspenwood),
  // (1.0, elder_birch_woodland),
  // (1.0, valcano_growth),
];
const MEDITERANEAN: BiomeFnCategory = &[
  (1.0, blank),
  // (1.0, chaparral_flats),
  // (1.0, redwood_grove),
  // (1.0, open_plain),
  // (1.0, sunflower_plain),
  // (1.0, chaparral_woods),
];
const MONSOON: BiomeFnCategory = &[
  (1.0, blank),
  // (1.0, mangrove_woods),
  // (1.0, light_jungle)
];
const TROPICAL: BiomeFnCategory = &[
  (1.0, blank),
  // (1.0, deep_jungle),
  // (1.0, light_jungle),
  // (1.0, bamboo_jungle)
];

// === Biome tables ===

const BLANK_TABLE: BiomeFnTable = &[&[BLANK]];
const SEA_TABLE: BiomeFnTable = &[&[SEA]];

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
}

impl Tables {
  pub fn new(ctx: &IdContext) -> Tables {
    Tables {
      blank_table:    table(ctx, BLANK_TABLE),
      sea_table:      table(ctx, SEA_TABLE),
      beach_table:    table(ctx, BEACH_TABLE),
      standard_table: table(ctx, STANDARD_TABLE),
      valley_table:   table(ctx, VALLEY_TABLE),
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
              .map(|(rarity, f)| BiomeBuilder::build("blank", ctx, *rarity, *f))
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
