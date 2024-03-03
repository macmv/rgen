use crate::{biome::*, builder::BiomeBuilder};

pub type BiomeList = Vec<BiomeBuilder>;
pub type BiomeTable = [[BiomeList; 8]; 12];

type BiomeFnCategory = &'static [BiomeFn];
type BiomeFnTable = &'static [&'static [BiomeFnCategory]];

// TODO: Need all of these biomes.
/*
const VALLEY_TABLE: [[&str; 6]; 7] = [
];
*/

// === Biome categories ===

const BLANK: BiomeFnCategory = &[plains];

const FROZEN_VALLEY: BiomeFnCategory = &[glacier, rockies, broken_glacier];
const BOG: BiomeFnCategory = &[bog, cold_bog, fall_bog, conifer_swamp];
const ROCKY_VALLEY: BiomeFnCategory = &[crag, snowy_crag /* , rocky_cedar */];
const COOL_VALLEY: BiomeFnCategory =
  &[crag /* , fir_wood, boreal_forest, cedar_wood, rocky_spruce */];
const SWAMP: BiomeFnCategory =
  &[plains /* cherry_blossom_grove, woodland, lavendar_grove, woodland, aspenwood */];
const DRY_RIVER: BiomeFnCategory = &[swamp /* , mangrove_woods */];
const WARM_VALLEY: BiomeFnCategory = &[plains];
const HOT_SWAMP: BiomeFnCategory = &[plains];
const TROPIC_SWAMP: BiomeFnCategory = &[plains];

const COLD_BEACH: BiomeFnCategory = &[snowy_shores, snowy_rock];
const COOL_BEACH: BiomeFnCategory =
  &[ancient_shores, mossy_shores, dry_shores, bare_rock, wet_rock];
const BEACH: BiomeFnCategory =
  &[sand_beach, monument_beach, red_sand_beach, red_monument_beach, palm_beach];
const DRY_BEACH: BiomeFnCategory =
  &[sand_beach, monument_beach, red_sand_beach, red_monument_beach, dry_shores, chaparral_beach];
const TROPIC_BEACH: BiomeFnCategory = &[sand_beach, chaparral_beach, jungle_beach, palm_beach];

// === Biome tables ===

const BLANK_TABLE: BiomeFnTable = &[&[BLANK]];

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

pub struct Tables {
  pub blank_table:  BiomeTable,
  pub beach_table:  BiomeTable,
  pub valley_table: BiomeTable,
}

impl Tables {
  pub fn new(ctx: &IdContext) -> Tables {
    Tables {
      blank_table:  table(ctx, BLANK_TABLE),
      beach_table:  table(ctx, BEACH_TABLE),
      valley_table: table(ctx, VALLEY_TABLE),
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
            biomes.iter().map(|f| BiomeBuilder::build("blank", ctx, *f)).collect::<BiomeList>()
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
