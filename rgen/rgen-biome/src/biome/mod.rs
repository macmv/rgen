//! Stores all the actual biome implementations.

mod lush_swamp;

use std::collections::HashMap;

use rgen_base::Blocks;
use rgen_placer::{Random, Rng};

use crate::{climate::Climate, BiomeBuilder};

/// Stores the map of climates to biomes.
pub struct ClimateMap {
  biomes: HashMap<Climate, Vec<BiomeBuilder>>,
}

impl ClimateMap {
  pub fn new(blocks: &Blocks) -> ClimateMap {
    let mut biomes = HashMap::new();

    macro_rules! biome {
      ($name:ident) => {
        BiomeBuilder::build(blocks, $name::$name)
      };
    }

    biomes.insert(Climate::Tundra, vec![biome!(lush_swamp)]);

    ClimateMap { biomes }
  }

  pub fn choose(&self, rng: &mut Rng, climate: Climate) -> &BiomeBuilder {
    rng.choose(self.biomes.get(&climate).unwrap())
  }
}
