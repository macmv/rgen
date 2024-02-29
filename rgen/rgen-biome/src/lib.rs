use biome::{ClimateMap, IdContext};
use rgen_base::{Block, Blocks, Chunk, ChunkPos, ChunkRelPos, Pos};
use rgen_placer::{
  noise::{NoiseGenerator, OctavedNoise, PerlinNoise},
  Rng,
};
use rgen_world::{Context, PartialWorld};
use splines::Key;
use table::Tables;

mod biome;
mod builder;
mod climate;
mod lookup;
mod table;

pub struct WorldBiomes {
  climates: ClimateMap,

  tables: Tables,

  height_map:      OctavedNoise<PerlinNoise>,
  temperature_map: OctavedNoise<PerlinNoise>,
  humidity_map:    OctavedNoise<PerlinNoise>,

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

impl WorldBiomes {
  pub fn new(blocks: &Blocks, biome_ids: &rgen_base::Biomes) -> Self {
    let ctx = IdContext { biomes: biome_ids, blocks };

    WorldBiomes {
      climates: ClimateMap::new(blocks, biome_ids),

      tables: Tables::new(&ctx),

      height_map:      OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },
      temperature_map: OctavedNoise { octaves: 8, freq: 1.0 / 2048.0, ..Default::default() },
      humidity_map:    OctavedNoise { octaves: 8, freq: 1.0 / 4096.0, ..Default::default() },

      continentalness_map: OctavedNoise { octaves: 8, freq: 1.0 / 1024.0, ..Default::default() },
      peaks_valleys_map:   OctavedNoise { octaves: 8, freq: 1.0 / 256.0, ..Default::default() },
      erosion_map:         OctavedNoise { octaves: 8, freq: 1.0 / 2048.0, ..Default::default() },
    }
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

        let biome = self.choose_biome(seed, pos);

        if chunk.get(rel_pos) == blocks.stone {
          chunk.set_data(rel_pos, biome.top_block, biome.top_block_data);
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
      (self.humidity_map.generate(pos.x as f64, pos.z as f64, rainfall_seed) + 1.0) / 2.0,
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
          (self.humidity_map.generate(pos.x as f64, pos.z as f64, rainfall_seed) + 1.0) / 2.0,
        );

        let mut rng = Rng::new(seed);
        let biome = self.climates.choose(&mut rng, climate);

        biomes[i] = biome.id.raw_id();
      }
    }
  }
}
