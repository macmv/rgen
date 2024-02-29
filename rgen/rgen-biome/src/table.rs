use crate::biome::*;

// TODO: Need all of these biomes.
/*
const VALLEY_TABLE: [[&str; 6]; 7] = [
  [rocky_valley, rocky_valley, bog, bog, bog, bog],
  [rocky_valley, cool_valley, cool_valley, cool_valley, bog, bog],
  [cool_valley, cool_valley, warm_valley, warm_valley, bog, swamp],
  [dry_river, warm_valley, warm_valley, warm_valley, swamp, swamp],
  [dry_river, swamp, warm_valley, swamp, swamp, hot_swamp],
  [dry_river, dry_river, swamp, swamp, hot_swamp, hot_swamp],
  [dry_river, dry_river, swamp, hot_swamp, hot_swamp, tropic_swamp],
];
*/

const VALLEY_TABLE: [[BiomeFn; 6]; 7] = [
  [plains, plains, plains, plains, plains, plains],
  [plains, plains, plains, plains, plains, plains],
  [plains, plains, plains, plains, plains, plains],
  [plains, plains, plains, plains, plains, plains],
  [plains, plains, plains, plains, plains, plains],
  [plains, plains, plains, plains, plains, plains],
  [plains, plains, plains, plains, plains, plains],
];
