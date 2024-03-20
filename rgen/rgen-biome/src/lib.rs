use biome::IdContext;
use cave::CaveCarver;
use rgen_base::{Block, Blocks, Chunk, ChunkPos, Pos};
use rgen_placer::{
  noise::{NoiseGenerator, NoiseGenerator3D, OctavedNoise, OpenSimplexNoise, PerlinNoise},
  Rng,
};
use rgen_spline::{Cosine, Spline};
use rgen_world::{Context, PartialWorld};
use table::Tables;

mod biome;
mod builder;
mod cave;
mod lookup;
mod table;

pub use builder::BiomeBuilder;

pub struct WorldBiomes {
  seed: u64,

  tables:         Tables,
  biome_override: bool,

  cave: CaveCarver,

  temperature_map: OctavedNoise<PerlinNoise, 8>,
  humidity_map:    OctavedNoise<PerlinNoise, 8>,

  /// Defines how far inland or how far into the sea any given block is.
  ///
  /// In order:
  /// - Sea (ocean, deep ocean)
  /// - Coast (beach)
  /// - Near Inland (plains)
  /// - Mid Inland (forest, small mountains)
  /// - Far Inland (mountains)
  continentalness_map: OctavedNoise<OpenSimplexNoise, 8>,

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
  peaks_valleys_map: OctavedNoise<OpenSimplexNoise, 6>,

  /// Defines how erroded the land is.
  ///
  /// Note that this is heavily affected by the peaks and valleys map.
  ///
  /// In order:
  /// - Not eroded (mountains)
  /// - Somewhat eroded (forests, plains)
  /// - most eroded (swamps, deserts)
  erosion_map: OctavedNoise<OpenSimplexNoise, 8>,

  /// Variance determines which biome to pick out of a list. Its basically
  /// random.
  variance_map: OctavedNoise<OpenSimplexNoise, 8>,

  density_map: OctavedNoise<PerlinNoise, 5>,

  /// Controlls the depth of the sub layer (usually dirt).
  sub_layer_map: OctavedNoise<OpenSimplexNoise, 3>,
}

lazy_static::lazy_static! {
  pub static ref CONTINENTALNESS: Spline<&'static [(f64, f64)]> = Spline::new(&[
    (0.00, 88.0),
    (0.01, 35.0),
    (0.15, 38.0),
    (0.26, 52.0),
    (0.40, 65.0),
    (0.81, 85.0),
    (0.91, 103.0),
    (1.00, 128.0),
  ]);

  pub static ref EROSION: Spline<&'static [(f64, f64)]> = Spline::new(&[
    (0.00, 1.00),
    (0.10, 0.90),
    (0.20, 0.70),
    (0.30, 0.60),
    (0.40, 0.20),
    (0.81, 0.10),
    (0.91, 0.05),
    (1.00, 0.00),
  ]);

  pub static ref PEAKS_VALLEYS: Spline<&'static [(f64, f64)]> = Spline::new(&[
    (0.00, 256.0),
    (0.30, 128.0),
    (0.40, 0.0),
    (0.47, 0.0),
    (0.50, -16.0),
    (0.53, 0.0),
    (0.60, 0.0),
    (0.70, 128.0),
    (1.00, 256.0),
  ]);
}

impl WorldBiomes {
  pub fn new(blocks: &Blocks, biome_ids: &rgen_base::Biomes, seed: u64) -> Self {
    let ctx = IdContext { biomes: biome_ids, blocks };

    WorldBiomes {
      // this is dumb but it makes rustfmt look nicer.
      seed: seed + 0,

      tables:         Tables::new(&ctx),
      biome_override: false,

      cave: CaveCarver::new(&ctx, seed),

      temperature_map: OctavedNoise::new(seed, 1.0 / 2048.0),
      humidity_map:    OctavedNoise::new(seed, 1.0 / 4096.0),

      continentalness_map: OctavedNoise::new(seed, 1.0 / 1024.0),
      peaks_valleys_map:   OctavedNoise::new(seed, 1.0 / 256.0),
      erosion_map:         OctavedNoise::new(seed, 1.0 / 2048.0),
      variance_map:        OctavedNoise::new(seed, 1.0 / 512.0),

      density_map: OctavedNoise::new(seed, 1.0 / 64.0),

      sub_layer_map: OctavedNoise::new(seed, 1.0 / 20.0),
    }
  }

  pub fn sample_continentalness(&self, pos: Pos) -> f64 {
    (self.continentalness_map.generate(pos.x as f64, pos.z as f64) * 0.5 + 0.5).clamp(0.0, 1.0)
  }

  pub fn sample_peaks_valleys(&self, pos: Pos) -> f64 {
    (self.peaks_valleys_map.generate(pos.x as f64, pos.z as f64) * 0.5 + 0.5).clamp(0.0, 1.0)
  }

  pub fn sample_erosion(&self, pos: Pos) -> f64 {
    (self.erosion_map.generate(pos.x as f64, pos.z as f64) * 0.5 + 0.5).clamp(0.0, 1.0)
  }

  pub fn sample_height(&self, pos: Pos) -> f64 {
    let c = CONTINENTALNESS.sample::<Cosine>(self.sample_continentalness(pos));
    let p = PEAKS_VALLEYS.sample::<Cosine>(self.sample_peaks_valleys(pos));
    let e = EROSION.sample::<Cosine>(self.sample_erosion(pos));

    // FIXME: Remove this, and figure out how to keep oceans
    if c < 64.0 {
      c
    } else {
      ((((c - 64.0) * 4.0) + 64.0) + p - 64.0) * e + 64.0
    }
  }

  pub fn generate_base(&self, ctx: &Context, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    for rel_x in 0..16_u8 {
      for rel_z in 0..16_u8 {
        let pos = chunk_pos.min_block_pos() + Pos::new(rel_x.into(), 0, rel_z.into());

        // let height = self.height_at(pos) as i32;
        // let biome = self.choose_biome(seed, pos);

        // The "max height" here is the maximum Y level for a single block. We then
        // linearly interpolate between min_height and max_height, and compare the
        // interpolated value to a 3D noise map, to choose if there will be a block
        // placed or not.
        //
        // So, the height isn't really "height," its more the hilliness of the terrain.
        let max_height = self.sample_height(pos);
        let min_height = 64.0 - max_height / 128.0;

        if max_height < 64.0 {
          for y in 0..=63 {
            if y < max_height as i32 {
              chunk.set(pos.with_y(y).chunk_rel(), ctx.blocks.stone.block);
            } else {
              chunk.set(pos.with_y(y).chunk_rel(), ctx.blocks.water.block);
            }
          }
        } else {
          for y in 0..=255 {
            let pos = pos.with_y(y);

            let noise =
              self.density_map.generate_3d(pos.x as f64, pos.y as f64, pos.z as f64) * 0.5 + 0.5;

            let limit = (y as f64 - min_height) / (max_height - min_height);

            if noise > limit {
              chunk.set(pos.chunk_rel(), ctx.blocks.stone.block);
            }
          }
        }

        /*
        for y in 0..height as u8 {
          chunk.set(ChunkRelPos::new(rel_x, y, rel_z), ctx.blocks.stone.block);
        }
        for y in height as u8..64 {
          chunk.set(ChunkRelPos::new(rel_x, y, rel_z), ctx.blocks.water.block);
        }
        */
      }
    }

    self.cave.carve(self, chunk, chunk_pos);

    self.generate_top_layer(&ctx.blocks, chunk, chunk_pos);
  }

  pub fn generate_top_layer(&self, blocks: &Blocks, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    // For each column in the chunk, fill in the top layers.
    for x in 0..16 {
      for z in 0..16 {
        let pos = chunk_pos.min_block_pos() + Pos::new(x, 0, z);
        let sub_layer_depth = self.sample_sub_layer_depth(pos);

        // FIXME: Sample 3D noise here and don't just use the min height.
        let max_height = self.sample_height(pos);
        let min_height = 64.0 - max_height / 128.0;

        let mut depth = 0;
        for y in (min_height as i32..=255).rev() {
          let pos = pos.with_y(y);
          let rel_pos = pos.chunk_rel();

          let block = chunk.get(pos.chunk_rel());
          if block != Block::AIR && ![blocks.leaves.block].contains(&block) {
            depth += 1;
          } else {
            depth = 0;
          }

          let biome = self.choose_biome(pos);
          if depth == 1 {
            if chunk.get(rel_pos) == blocks.stone.block {
              chunk.set_state(rel_pos, biome.top_block);
            }
          } else if depth <= sub_layer_depth {
            if chunk.get(rel_pos) == blocks.stone.block {
              chunk.set_state(rel_pos, biome.sub_layer);
            }
          }
        }
      }
    }
  }

  fn sample_sub_layer_depth(&self, pos: Pos) -> i32 {
    let noise = self.sub_layer_map.generate(pos.x as f64, pos.z as f64);
    let depth = (noise * 2.0 + 3.0).round() as i32;
    depth
  }

  pub fn decorate(&self, blocks: &Blocks, world: &mut PartialWorld, chunk_pos: ChunkPos) {
    let mut biome_names = [[""; 16]; 16];
    // The length of this list is how many total biomes we support in a single
    // chunk. If there are more biomes than this, the extra ones will not be
    // decorated. This is an optimization to avoid allocating here.
    let mut biome_index = 0;
    let mut biome_set = [Option::<&BiomeBuilder>::None; 16];

    for x in 0..16 {
      for z in 0..16 {
        let pos = chunk_pos.min_block_pos() + Pos::new(x, 0, z);
        let biome = self.choose_biome(pos);
        biome_names[x as usize][z as usize] = biome.name;

        // `biome_set` acts like a set, so we need to check if this is a new biome or
        // not. Note that this means every biome name _must_ be unique.
        if !biome_set[..biome_index].iter().any(|b| b.is_some_and(|b| b.name == biome.name))
          && biome_index < biome_set.len()
        {
          biome_set[biome_index] = Some(biome);
          biome_index += 1;
        }
      }
    }

    for biome in biome_set.into_iter().flatten() {
      let mut rng = Rng::new(self.seed);
      biome.decorate(blocks, &mut rng, chunk_pos, world, |pos| {
        let rel_x = pos.x - chunk_pos.min_block_pos().x;
        let rel_z = pos.z - chunk_pos.min_block_pos().z;
        biome_names[rel_x as usize][rel_z as usize] == biome.name
      });
    }
  }
}
