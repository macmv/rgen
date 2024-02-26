//! Stores all the actual biome implementations.

mod cold_taiga;
mod extreme_hills;
mod forest;
mod ice_plains;
mod lush_swamp;
mod plains;
mod roofed_forest;
mod savanna;
mod swamp;

use std::collections::HashMap;

use rgen_base::Blocks;
use rgen_placer::{Random, Rng};

use crate::{climate::Climate, BiomeBuilder};

/// Stores the map of climates to biomes.
pub struct ClimateMap {
  biomes: HashMap<Climate, Vec<BiomeBuilder>>,
}

impl ClimateMap {
  pub fn new(blocks: &Blocks, biome_ids: &rgen_base::Biomes) -> ClimateMap {
    let mut biomes = HashMap::new();

    macro_rules! biome {
      ($name:ident) => {
        BiomeBuilder::build(stringify!($name), blocks, biome_ids, $name::$name)
      };
    }

    biomes.insert(Climate::IceCap, vec![biome!(ice_plains)]);
    biomes.insert(Climate::Tundra, vec![biome!(cold_taiga)]);
    biomes.insert(Climate::SubArctic, vec![biome!(extreme_hills)]);
    biomes.insert(Climate::ColdSwamp, vec![biome!(swamp)]);
    biomes.insert(Climate::DryTemperate, vec![biome!(plains)]);
    biomes.insert(Climate::CoolTemperate, vec![biome!(forest)]);
    biomes.insert(Climate::WetTemperate, vec![biome!(roofed_forest)]);
    biomes.insert(Climate::Savanna, vec![biome!(savanna)]);
    biomes.insert(Climate::WarmTemperate, vec![biome!(plains)]);
    biomes.insert(Climate::HotDesert, vec![biome!(plains)]);
    biomes.insert(Climate::Mediteranean, vec![biome!(plains)]);
    biomes.insert(Climate::HotSwamp, vec![biome!(plains)]);
    biomes.insert(Climate::HighDesert, vec![biome!(plains)]);
    biomes.insert(Climate::Tropical, vec![biome!(plains)]);

    ClimateMap { biomes }
  }

  pub fn choose(&self, rng: &mut Rng, climate: Climate) -> &BiomeBuilder {
    rng.choose(self.biomes.get(&climate).unwrap())
  }
}
