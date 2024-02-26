use biome::ClimateMap;
use rgen_base::{Biome, Block, Blocks, Chunk, ChunkPos, Pos};
use rgen_placer::{
  grid::PointGrid,
  noise::{self, NoiseGenerator},
  Placer, Random, Rng, World,
};

mod biome;
mod climate;

pub struct BiomeBuilder {
  pub name: &'static str,
  pub id:   rgen_base::Biome,

  pub top_block: Block,

  placers: Vec<PlacerBuilder>,
}

struct PlacerBuilder {
  placer: Box<dyn Placer>,
  grid:   PointGrid,
}

impl PlacerBuilder {
  fn new(placer: Box<dyn Placer>) -> Self { Self { placer, grid: PointGrid::new() } }
}

pub enum PlacerStage {
  Sand,
  Sand2,
  Tree,
  Ore,
}

impl BiomeBuilder {
  pub fn new(name: &'static str, blocks: &Blocks) -> Self {
    Self { name, id: Biome::VOID, top_block: blocks.grass, placers: vec![] }
  }

  pub fn place(&mut self, name: &str, stage: PlacerStage, placer: impl Placer + 'static) {
    // TODO: Do we even need name? Its a pain to add them later, so I'm keeping them
    // for now.
    let _ = name;

    self.place0(stage, Box::new(placer));
  }

  // Don't monomorphise this.
  fn place0(&mut self, _stage: PlacerStage, placer: Box<dyn Placer>) {
    // TODO: Using the stage, insert this at the right spot.
    self.placers.push(PlacerBuilder::new(placer));
  }

  /// Decorates the given chunk. The `rng` passed in should only be seeded with
  /// the world seed.
  pub fn decorate(&self, blocks: &Blocks, rng: &mut Rng, chunk_pos: ChunkPos, world: &mut World) {
    for placer in self.placers.iter() {
      let seed = rng.next();

      const SCALE: f64 = 1.0 / 4.0;

      let min_x = chunk_pos.min_block_pos().x as f64 * SCALE;
      let min_y = chunk_pos.min_block_pos().z as f64 * SCALE;
      let max_x = (chunk_pos.min_block_pos().x + 15) as f64 * SCALE;
      let max_y = (chunk_pos.min_block_pos().z + 15) as f64 * SCALE;

      for point in placer.grid.points_in_area(seed, min_x, min_y, max_x, max_y) {
        let pos = world.top_block_excluding(
          Pos::new((point.0 / SCALE) as i32, 0, (point.1 / SCALE) as i32),
          &[blocks.leaves],
        );

        // This builds a unique seed for each placer. This gives the placer the same
        // seed if it crosses chunk boundaries.
        let seed = rng.next() ^ (pos.x as u64).wrapping_add(pos.z as u64) << 32;
        placer.placer.place(world, &mut Rng::new(seed), pos);
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
  fn build(
    name: &'static str,
    blocks: &Blocks,
    biomes: &rgen_base::Biomes,
    build: impl FnOnce(&Blocks, &rgen_base::Biomes, &mut Self),
  ) -> Self {
    let mut builder = BiomeBuilder::new(name, blocks);
    build(blocks, biomes, &mut builder);
    builder
  }
}

impl Biomes {
  pub fn new(blocks: &Blocks, biome_ids: &rgen_base::Biomes) -> Self {
    Biomes {
      climates:        ClimateMap::new(blocks, biome_ids),
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

    println!("biome: {:?}", biome.name);

    biome.decorate(blocks, &mut rng, chunk_pos, &mut world);
  }

  pub fn generate_ids(&self, seed: u64, chunk_pos: ChunkPos, biomes: &mut [u8; 256]) {
    let temperature_seed = seed.wrapping_add(1);
    let rainfall_seed = seed.wrapping_add(2);

    for x in 0..16 {
      for z in 0..16 {
        let i = (x * 16 + z) as usize;
        let pos = chunk_pos.min_block_pos() + Pos::new(x, 0, z);

        let climate = climate::from_temperature_and_rainfall(
          (self.temperature_map.generate(pos.x as f64, pos.z as f64, temperature_seed) + 1.0) / 2.0,
          (self.rainfall_map.generate(pos.x as f64, pos.z as f64, rainfall_seed) + 1.0) / 2.0,
        );

        let mut rng = Rng::new(seed);
        let biome = self.climates.choose(&mut rng, climate);

        biomes[i] = biome.id.raw_id();
      }
    }
  }
}
