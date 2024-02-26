use biome::ClimateMap;
use rgen_base::{Block, Blocks, Chunk, ChunkPos, Pos};
use rgen_placer::{
  noise::{self, NoiseGenerator},
  Placer, Random, Rng, World,
};

mod biome;
mod climate;

pub struct BiomeBuilder {
  pub top_block: Block,

  placers: Vec<Box<dyn Placer>>,
}

pub enum PlacerStage {
  Sand,
  Sand2,
  Tree,
  Ore,
}

impl BiomeBuilder {
  pub fn new(blocks: &Blocks) -> Self { Self { top_block: blocks.grass, placers: vec![] } }

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

  pub fn decorate(&self, _blocks: &Blocks, rng: &mut Rng, chunk_pos: ChunkPos, world: &mut World) {
    for placer in &self.placers {
      for _ in 0..placer.amount_per_chunk() {
        let pos = world.top_block(
          chunk_pos.min_block_pos()
            + Pos::new(rng.rand_exclusive(0, 16), 0, rng.rand_exclusive(0, 16)),
        );

        placer.place(world, rng, pos);
      }
    }
  }
}

pub struct Biomes {
  climates: ClimateMap,

  temperature_map: noise::OctavedNoise<noise::PerlinNoise>,
  rainfall_map:    noise::OctavedNoise<noise::PerlinNoise>,
}

impl BiomeBuilder {
  fn build(blocks: &Blocks, build: impl FnOnce(&Blocks, &mut Self)) -> Self {
    let mut builder = BiomeBuilder::new(blocks);
    build(blocks, &mut builder);
    builder
  }
}

impl Biomes {
  pub fn new(blocks: &Blocks) -> Self {
    Biomes {
      climates:        ClimateMap::new(blocks),
      temperature_map: noise::OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },
      rainfall_map:    noise::OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },
    }
  }

  pub fn generate(&self, blocks: &Blocks, seed: u64, chunk_pos: ChunkPos, chunk: &mut Chunk) {
    let mut world = World::new(chunk_pos, chunk);

    let temperature_seed = seed.wrapping_add(1);
    let rainfall_seed = seed.wrapping_add(2);

    // For each column in the chunk, fill in the top layers.
    for x in 0..16 {
      for z in 0..16 {
        let pos = world.top_block(chunk_pos.min_block_pos() + Pos::new(x, 0, z));

        let climate = climate::from_temperature_and_rainfall(
          (self.temperature_map.generate(pos.x as f64, pos.z as f64, temperature_seed) + 1.0) / 2.0,
          (self.rainfall_map.generate(pos.x as f64, pos.z as f64, rainfall_seed) + 1.0) / 2.0,
        );

        let mut rng = Rng::new(seed);
        let biome = self.climates.choose(&mut rng, climate);

        world.set(pos, biome.top_block);
      }
    }

    // FIXME: Need to decorate with all biomes in a chunk.
    let pos = chunk_pos.min_block_pos();
    let climate = climate::from_temperature_and_rainfall(
      (self.temperature_map.generate(pos.x as f64, pos.z as f64, temperature_seed) + 1.0) / 2.0,
      (self.rainfall_map.generate(pos.x as f64, pos.z as f64, rainfall_seed) + 1.0) / 2.0,
    );

    // FIXME: How do we switch up biomes within a given climate?
    let mut rng = Rng::new(seed);
    let biome = self.climates.choose(&mut rng, climate);

    biome.decorate(blocks, &mut rng, chunk_pos, &mut world);
  }
}
