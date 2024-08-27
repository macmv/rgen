use biome::IdContext;
use cave::CaveCarver;
use rgen_base::{Block, Blocks, Chunk, ChunkPos, Pos};
use rgen_placer::{
  noise::{NoiseGenerator, NoiseGenerator3D, OctavedNoise, OpenSimplexNoise, PerlinNoise},
  BiomeCachedChunk, Rng, TemporaryBiome,
};
use rgen_spline::{Cosine, Spline};
use rgen_world::{Context, Generator, PartialWorld};
use structure::StructureGenerator;
use table::Tables;

mod biome;
mod builder;
mod cave;
mod lookup;
mod structure;
mod table;

pub use builder::BiomeBuilder;

pub struct WorldBiomes {
  seed: u64,

  tables:         Tables,
  biome_override: bool,

  cave:      CaveCarver,
  structure: StructureGenerator,

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

      cave:      CaveCarver::new(&ctx, seed),
      structure: StructureGenerator::new(&ctx, seed),

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

  pub fn sample_river_distance(&self, pos: Pos) -> f64 {
    // This will be between 0.0 and 0.02 when there is a river, and any value higher
    // will be outside of a river.
    let distance_to_river = (self.sample_peaks_valleys(pos) - 0.5).abs();

    // So, return a value from 0.0 to 1.0 for the range 0.0 to 0.16, so that
    // caves can smooth the transition over to rivers.
    if distance_to_river > 0.16 {
      1.0
    } else {
      distance_to_river / 0.16
    }
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
}

impl Generator for WorldBiomes {
  fn generate_base(&self, ctx: &Context, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    for rel_x in 0..16_u8 {
      for rel_z in 0..16_u8 {
        let pos = chunk_pos.min_block_pos() + Pos::new(rel_x.into(), 0, rel_z.into());

        // let height = self.height_at(pos) as i32;
        // let biome = self.choose_biome(seed, pos);
        let mut info = self.height_info(pos);

        if info.max_height() < 64.0 {
          for y in 0..=63 {
            if y < info.max_height() as i32 {
              chunk.set(pos.with_y(y).chunk_rel(), ctx.blocks.stone.block);
            } else {
              chunk.set(pos.with_y(y).chunk_rel(), ctx.blocks.water.block);
            }
          }
        } else {
          for y in 0..=255 {
            let pos = pos.with_y(y);

            info.move_to(pos);
            if info.underground() {
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

    self.generate_chunk_placers(chunk, chunk_pos);

    // TODO: Generate villages!
    if false {
      self.structure.generate(chunk, chunk_pos);
    }
  }

  fn decorate(&self, ctx: &Context, world: &mut PartialWorld, chunk_pos: ChunkPos) {
    // TODO: Maybe make this 3D as well? Not sure if we want underground trees or
    // anything.

    let mut biome_names = [[""; 16]; 16];
    // The length of this list is how many total biomes we support in a single
    // chunk. If there are more biomes than this, the extra ones will not be
    // decorated. This is an optimization to avoid allocating here.
    let mut biome_index = 0;
    let mut biome_set = [Option::<&BiomeBuilder>::None; 16];

    for x in 0..16 {
      for z in 0..16 {
        // Check at Y=255, to get all the surface biomes.
        let pos = chunk_pos.min_block_pos() + Pos::new(x, 255, z);
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
      biome.decorate(&ctx.blocks, &mut rng, chunk_pos, world, |pos| {
        let rel_x = pos.x - chunk_pos.min_block_pos().x;
        let rel_z = pos.z - chunk_pos.min_block_pos().z;
        biome_names[rel_x as usize][rel_z as usize] == biome.name
      });
    }

    world.set(chunk_pos.min_block_pos() + Pos::new(0, 6, 0), ctx.blocks.dirt.block);
  }
}

impl WorldBiomes {
  fn generate_top_layer(&self, blocks: &Blocks, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    // FIXME: Remove this and use a chunk placer instead.

    // For each column in the chunk, fill in the top layers.
    for x in 0..16 {
      for z in 0..16 {
        let pos = chunk_pos.min_block_pos() + Pos::new(x, 0, z);
        let sub_layer_depth = self.sample_sub_layer_depth(pos);

        let mut info = self.height_info(pos);

        let mut depth = 0;
        for y in (info.min_height as i32..=info.max_height as i32).rev() {
          let pos = pos.with_y(y);
          let rel_pos = pos.chunk_rel();

          info.move_to(pos);
          if info.underground() {
            depth += 1;
          } else {
            depth = 0;
          }

          // The addition of 255 prevents the underground biome from interfering with the
          // sublayer selection.
          let biome = self.choose_biome(pos.with_y(255));
          if depth <= sub_layer_depth {
            if chunk.get(rel_pos) == blocks.stone.block {
              // Don't use depth, use y + 1, so that we account for caves.
              if chunk.get(rel_pos.with_y(y + 1)) == Block::AIR {
                chunk.set(rel_pos, biome.top_block);
              } else {
                chunk.set(rel_pos, biome.sub_layer);
              }
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

  fn generate_chunk_placers(&self, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    // The length of this list is how many total biomes we support in a single
    // chunk. If there are more biomes than this, the extra ones will not be
    // decorated. This is an optimization to avoid allocating here.
    let mut biome_index = 0;
    let mut biome_set = [Option::<(&BiomeBuilder, TemporaryBiome)>::None; 16];

    let mut chunk = BiomeCachedChunk::new(chunk);

    for x in 0..16 {
      for z in 0..16 {
        // This is kinda restrictive, but helps performance a _lot_. We also don't
        // really want biomes to change on the Y axis, as that causes weirdness when
        // building with grass and such. So we can limit ourselves to a single surface
        // biome and a single cave biome per column.
        let surface_biome = self.choose_biome(chunk_pos.min_block_pos() + Pos::new(x, 255, z));
        let cave_biome = self.choose_biome(chunk_pos.min_block_pos() + Pos::new(x, 0, z));

        let mut info = self.height_info(chunk_pos.min_block_pos() + Pos::new(x, 0, z));

        for y in 0..256 {
          let pos = chunk_pos.min_block_pos() + Pos::new(x, y, z);
          info.move_to(pos);

          let biome = if info.underground() { cave_biome } else { surface_biome };

          match biome_set[..biome_index]
            .iter()
            .find(|b| b.is_some_and(|(b, _)| b.name == biome.name))
          {
            Some(Some((_, id))) => {
              chunk.set_biome(pos.chunk_rel(), *id);
            }
            Some(None) => unreachable!(),
            None => {
              if biome_index < 15 {
                let id = TemporaryBiome(biome_index as u8);
                chunk.set_biome(pos.chunk_rel(), id);
                biome_set[biome_index] = Some((biome, id));
                biome_index += 1;
              } else {
                // if there would be too many biomes, set it to the max ID, which won't be used.
                chunk.set_biome(pos.chunk_rel(), TemporaryBiome(15));
              }
            }
          }
        }
      }
    }

    for (biome, id) in biome_set.into_iter().flatten() {
      let mut rng = Rng::new(self.seed);
      chunk.set_active(id);
      biome.generate(&mut rng, &mut chunk, chunk_pos);
    }
  }

  pub fn height_info(&self, pos: Pos) -> HeightInfo {
    let mut info =
      HeightInfo { world: self, pos, max_height: 0.0, min_height: 0.0, underground: None };
    info.change_xz();
    info
  }
}

pub struct HeightInfo<'a> {
  world: &'a WorldBiomes,
  pos:   Pos,

  // The "max height" here is the maximum Y level for a single block. We then
  // linearly interpolate between min_height and max_height, and compare the
  // interpolated value to a 3D noise map, to choose if there will be a block
  // placed or not.
  //
  // So, the height isn't really "height," its more the hilliness of the terrain.
  max_height:  f64,
  min_height:  f64,
  underground: Option<bool>,
}

impl HeightInfo<'_> {
  fn change_xz(&mut self) {
    self.max_height = self.world.sample_height(self.pos);
    self.min_height = 64.0 - self.max_height / 128.0;
  }
  fn change_y(&mut self) { self.underground = None; }

  pub fn move_to(&mut self, pos: Pos) {
    let old_pos = self.pos;
    self.pos = pos;

    if self.pos.x != old_pos.x || self.pos.z != old_pos.z {
      self.change_xz();
    }
    if self.pos.y != old_pos.y {
      self.change_y();
    }
  }

  pub fn max_height(&self) -> f64 { self.max_height }
  pub fn min_height(&self) -> f64 { self.min_height }
  pub fn underground(&mut self) -> bool {
    *self.underground.get_or_insert_with(|| {
      let noise =
        self.world.density_map.generate_3d(self.pos.x as f64, self.pos.y as f64, self.pos.z as f64)
          * 0.5
          + 0.5;
      let limit = (self.pos.y as f64 - self.min_height) / (self.max_height - self.min_height);

      noise > limit
    })
  }
}
