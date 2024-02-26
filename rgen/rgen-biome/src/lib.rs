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

  pub fn generate(&self, blocks: &Blocks, chunk_pos: ChunkPos, chunk: &mut Chunk) {
    let pos = chunk_pos.min_block_pos();

    let climate = climate::from_temperature_and_rainfall(
      self.temperature_map.generate(pos.x as f64, pos.z as f64, 1234),
      self.rainfall_map.generate(pos.x as f64, pos.z as f64, 1234),
    );
    dbg!(&climate);

    let mut rng = Rng::new(1234);

    let biome = self.climates.choose(&mut rng, climate);
    biome.generate(blocks, &mut rng, chunk_pos, chunk);
  }
}
