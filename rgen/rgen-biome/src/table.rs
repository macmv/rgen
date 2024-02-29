use crate::{biome::*, builder::BiomeBuilder};

pub type BiomeTable = [[BiomeBuilder; 8]; 12];
type BiomeFnTable = &'static [&'static [BiomeFn]];

// TODO: Need all of these biomes.
/*
const VALLEY_TABLE: [[&str; 6]; 7] = [
];
*/

const BLANK_TABLE: BiomeFnTable = &[&[blank]];

const VALLEY_TABLE: BiomeFnTable = &[
  &[rocky_valley, rocky_valley, rocky_valley, cool_valley, swamp, swamp, dry_river, dry_river],
  &[rocky_valley, rocky_valley, cool_valley, cool_valley, warm_valley, swamp, swamp, dry_river],
  &[bog, bog, cool_valley, warm_valley, warm_valley, warm_valley, swamp, swamp],
  &[bog, bog, cool_valley, warm_valley, warm_valley, swamp, swamp, hot_swamp],
  &[bog, bog, bog, bog, swamp, swamp, hot_swamp, hot_swamp],
  &[bog, bog, bog, swamp, swamp, hot_swamp, hot_swamp, tropic_swamp],
];

pub struct Tables {
  pub blank_table:  BiomeTable,
  pub valley_table: BiomeTable,
}

impl Tables {
  pub fn new(ctx: &IdContext) -> Tables {
    Tables { blank_table: table(ctx, BLANK_TABLE), valley_table: table(ctx, VALLEY_TABLE) }
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

      let items = row.iter().map(|&f| BiomeBuilder::build("blank", ctx, *f)).collect::<Vec<_>>();
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
