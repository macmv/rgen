use biome::ClimateMap;
use rgen_base::{Biome, Block, Blocks, Chunk, ChunkPos, ChunkRelPos, Pos};
use rgen_placer::{
  grid::PointGrid,
  noise::{NoiseGenerator, OctavedNoise, PerlinNoise},
  Placer, Random, Rng,
};
use rgen_world::{Context, PartialWorld};
use splines::Key;

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
  pub fn decorate(
    &self,
    blocks: &Blocks,
    rng: &mut Rng,
    chunk_pos: ChunkPos,
    world: &mut PartialWorld,
  ) {
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
        let seed = rng.next() ^ (pos.x as u64) << 32 ^ pos.z as u64;
        placer.placer.place(world, &mut Rng::new(seed), pos);
      }
    }
  }
}

pub struct WorldBiomes {
  climates: ClimateMap,

  height_map:      OctavedNoise<PerlinNoise>,
  temperature_map: OctavedNoise<PerlinNoise>,
  rainfall_map:    OctavedNoise<PerlinNoise>,

  /// Defines how far inland or how far into the sea any given block is.
  ///
  /// In order:
  /// - Sea (ocean, deep ocean)
  /// - Coast (beach)
  /// - Near Inland (plains)
  /// - Mid Inland (forest, small mountains)
  /// - Far Inland (mountains)
  continentalness_map: OctavedNoise<PerlinNoise>,

  /// Defines the approximate height of the type of biome. Note that this isn't
  /// the height map, its almost the height goal of the biome that is chosen.
  ///
  /// Note that the low/mid/high slices can also change based on the
  /// continalness.
  ///
  /// In order:
  /// - Valley (rivers, swamps)
  /// - Low Slice (plains?)
  /// - Mid Slice (forest, small mountains)
  /// - High Slice (mountains)
  /// - Peak (extreme hills)
  peaks_valleys_map: OctavedNoise<PerlinNoise>,

  /// Defines how erroded the land is.
  ///
  /// Note that this is heavily affected by the peaks and valleys map.
  ///
  /// In order:
  /// - Not eroded (mountains)
  /// - Somewhat eroded (forests, plains)
  /// - most eroded (swamps, deserts)
  erosion_map: OctavedNoise<PerlinNoise>,
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

lazy_static::lazy_static! {
  pub static ref CONTINENTALNESS_TO_HEIGHT: splines::Spline<f64, f64> = splines::Spline::from_vec(vec![
    Key::new(0.0, 120.0, splines::Interpolation::Cosine),
    Key::new(0.1, 40.0, splines::Interpolation::Cosine),
    Key::new(0.3, 40.0, splines::Interpolation::Cosine),
    Key::new(0.4, 70.0, splines::Interpolation::Cosine),
    Key::new(0.5, 80.0, splines::Interpolation::Cosine),
    Key::new(0.8, 140.0, splines::Interpolation::Cosine),
    Key::new(1.0, 150.0, splines::Interpolation::Cosine),
  ]);
}

enum ContinentalnessCategory {
  Sea,
  Coast,
  NearInland,
  MidInland,
  FarInland,
}

enum PeaksValleysCategory {
  Valley,
  LowSlice,
  MidSlice,
  HighSlice,
  Peak,
}

struct ErosionCategory(u8);

impl WorldBiomes {
  pub fn new(blocks: &Blocks, biome_ids: &rgen_base::Biomes) -> Self {
    WorldBiomes {
      climates:        ClimateMap::new(blocks, biome_ids),
      height_map:      OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },
      temperature_map: OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },
      rainfall_map:    OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },

      continentalness_map: OctavedNoise { octaves: 8, freq: 1.0 / 1024.0, ..Default::default() },
      peaks_valleys_map:   OctavedNoise { octaves: 8, freq: 1.0 / 256.0, ..Default::default() },
      erosion_map:         OctavedNoise { octaves: 8, freq: 1.0 / 2048.0, ..Default::default() },
    }
  }

  fn continentalness_category(&self, seed: u64, pos: Pos) -> ContinentalnessCategory {
    let continentalness =
      self.continentalness_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5;

    match continentalness {
      x if x < 0.1 => ContinentalnessCategory::Sea,
      x if x < 0.3 => ContinentalnessCategory::Coast,
      x if x < 0.6 => ContinentalnessCategory::NearInland,
      x if x < 0.8 => ContinentalnessCategory::MidInland,
      _ => ContinentalnessCategory::FarInland,
    }
  }

  fn peaks_valleys_category(&self, seed: u64, pos: Pos) -> PeaksValleysCategory {
    let seed = seed.wrapping_add(1);

    let peaks_valleys =
      self.peaks_valleys_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5;

    match peaks_valleys {
      x if x < 0.075 => PeaksValleysCategory::Valley,
      x if x < 0.2 => PeaksValleysCategory::LowSlice,
      x if x < 0.6 => PeaksValleysCategory::MidSlice,
      x if x < 0.85 => PeaksValleysCategory::HighSlice,
      _ => PeaksValleysCategory::Peak,
    }
  }

  fn erosion_category(&self, seed: u64, pos: Pos) -> ErosionCategory {
    let seed = seed.wrapping_add(2);

    let erosion = self.peaks_valleys_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5;

    // FIXME: This is dumb
    ErosionCategory((erosion * 6.9999) as u8)
  }

  fn sample_height(&self, seed: u64, pos: Pos) -> f64 {
    let continentalness =
      ((self.continentalness_map.generate(pos.x as f64, pos.z as f64, seed) + 1.0) / 2.0)
        .clamp(0.0, 1.0);

    let height = CONTINENTALNESS_TO_HEIGHT.sample(continentalness).unwrap_or_default();

    height
  }

  pub fn height_at(&self, pos: Pos) -> f64 {
    // let noise_height = self.height_map.generate(pos.x as f64, pos.z as f64, 0) +
    // 1.0; noise_height * 64.0

    self.sample_height(0, pos)
  }

  pub fn generate_base(&self, seed: u64, ctx: &Context, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    for rel_x in 0..16_u8 {
      for rel_z in 0..16_u8 {
        let pos = chunk_pos.min_block_pos() + Pos::new(rel_x.into(), 0, rel_z.into());

        let height = self.height_at(pos) as i32;

        for y in 0..height as u8 {
          chunk.set(ChunkRelPos::new(rel_x, y, rel_z), ctx.blocks.stone);
        }
        for y in height as u8..64 {
          chunk.set(ChunkRelPos::new(rel_x, y, rel_z), ctx.blocks.water);
        }
      }
    }

    self.generate_top_layer(seed, &ctx.blocks, chunk, chunk_pos);
  }

  pub fn generate_top_layer(
    &self,
    seed: u64,
    blocks: &Blocks,
    chunk: &mut Chunk,
    chunk_pos: ChunkPos,
  ) {
    let temperature_seed = seed.wrapping_add(1);
    let rainfall_seed = seed.wrapping_add(2);

    // For each column in the chunk, fill in the top layers.
    for x in 0..16 {
      for z in 0..16 {
        let rel_pos = ChunkRelPos::new(x, 0, z);

        let mut y = 255;
        while y > 0 {
          let block = chunk.get(rel_pos.with_y(y));
          if block != Block::AIR && ![blocks.leaves].contains(&block) {
            break;
          }
          y -= 1;
        }
        let rel_pos = rel_pos.with_y(y);
        let pos =
          chunk_pos.min_block_pos() + Pos::new(rel_pos.x().into(), rel_pos.y(), rel_pos.z().into());

        let climate = climate::from_temperature_and_rainfall(
          (self.temperature_map.generate(pos.x as f64, pos.z as f64, temperature_seed) + 1.0) / 2.0,
          (self.rainfall_map.generate(pos.x as f64, pos.z as f64, rainfall_seed) + 1.0) / 2.0,
        );

        let mut rng = Rng::new(seed);
        let biome = self.climates.choose(&mut rng, climate);

        if chunk.get(rel_pos) == blocks.stone {
          chunk.set(rel_pos, biome.top_block);
        }
      }
    }
  }

  pub fn decorate(
    &self,
    blocks: &Blocks,
    seed: u64,
    world: &mut PartialWorld,
    chunk_pos: ChunkPos,
  ) {
    let temperature_seed = seed.wrapping_add(1);
    let rainfall_seed = seed.wrapping_add(2);

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

    biome.decorate(blocks, &mut rng, chunk_pos, world);
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
