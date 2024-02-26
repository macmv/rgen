use std::collections::HashMap;

use climate::Climate;
use rgen_base::{Blocks, Chunk, ChunkPos, Pos};
use rgen_placer::{
  noise::{self, NoiseGenerator},
  Placer, Random, Rng, World,
};

mod biome;
mod climate;

pub struct BiomeBuilder {
  placers: Vec<Box<dyn Placer>>,
}

pub enum PlacerStage {
  Sand,
  Sand2,
  Tree,
  Ore,
}

impl BiomeBuilder {
  pub fn new() -> Self { Self { placers: vec![] } }

  pub fn place(&mut self, name: &str, stage: PlacerStage, placer: impl Placer + 'static) {
    // TODO: Do we even need name? Its a pain to add them later, so I'm keeping them
    // for now.
    let _ = name;

    self.place0(stage, Box::new(placer));
  }

  // Don't monomorphise this.
  fn place0(&mut self, _stage: PlacerStage, placer: Box<dyn Placer>) {
    // TODO: Using the stage, insert this at the right spot.
    self.placers.push(placer);
  }

  pub fn generate(
    &self,
    _blocks: &Blocks,
    rng: &mut Rng,
    chunk_pos: ChunkPos,
    chunk: &mut rgen_base::Chunk,
  ) {
    let mut world = World::new(chunk_pos, chunk);

    for placer in &self.placers {
      for _ in 0..placer.amount_per_chunk() {
        let mut pos = chunk_pos.min_block_pos()
          + Pos::new(rng.rand_exclusive(0, 16), 255, rng.rand_exclusive(0, 16));
        while pos.y > 0 && world.get(pos) == rgen_base::Block::AIR {
          pos.y -= 1;
        }
        println!("placing at {:?}", pos);

        placer.place(&mut world, rng, pos);
      }
    }
  }
}

pub struct Biomes {
  climates: HashMap<Climate, Vec<BiomeBuilder>>,

  temperature_map: noise::OctavedNoise<noise::PerlinNoise>,
  rainfall_map:    noise::OctavedNoise<noise::PerlinNoise>,
}

impl BiomeBuilder {
  fn build(blocks: &Blocks, build: impl FnOnce(&Blocks, &mut Self)) -> Self {
    let mut builder = BiomeBuilder::new();
    build(&blocks, &mut builder);
    builder
  }
}

impl Biomes {
  pub fn new(blocks: &Blocks) -> Self {
    let mut climates = HashMap::new();

    macro_rules! biome {
      ($build:expr) => {
        BiomeBuilder::build(blocks, $build)
      };
    }

    climates.insert(Climate::Tundra, vec![biome!(biome::lush_swamp)]);

    Biomes {
      climates,
      temperature_map: noise::OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },
      rainfall_map: noise::OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },
    }
  }

  pub fn generate(&self, blocks: &Blocks, chunk_pos: ChunkPos, chunk: &mut Chunk) {
    let pos = chunk_pos.min_block_pos();

    let climate = climate::from_temperature_and_rainfall(
      self.temperature_map.generate(pos.x as f64, pos.z as f64, 1234),
      self.rainfall_map.generate(pos.x as f64, pos.z as f64, 1234),
    );
    dbg!(&climate);

    let mut rng = Rng::new(1234);

    let biomes = self.climates.get(&climate).unwrap();
    let biome = rng.choose(biomes);
    biome.generate(blocks, &mut rng, chunk_pos, chunk);
  }
}
