use cave::CaveCarver;
use rgen_base::{block, Chunk, ChunkPos, Pos, StateId};
use rgen_placer::{
  chunk_placer,
  noise::{
    NoiseGenerator, NoiseGenerator3D, OctavedNoise, OpenSimplexNoise, PerlinNoise, SeededNoise,
    ShiftedNoise, VoronoiNoise,
  },
  BiomeCachedChunk, ChunkPlacer, Rng, TemporaryBiome,
};
use rgen_spline::{Cosine, Spline};
use rgen_world::{BlockInfoSupplier, Context, Generator, PartialWorld};
use structure::StructureGenerator;
use table::CompositionLookup;

mod biome;
mod builder;
mod cave;
mod lookup;
mod structure;
mod table;

pub use builder::BiomeBuilder;

pub struct WorldBiomes {
  seed: u64,

  composition_lookup: CompositionLookup,
  biome_override:     bool,

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
  variance_map: ShiftedNoise<VoronoiNoise, OpenSimplexNoise>,

  density_map: OctavedNoise<PerlinNoise, 5>,

  /// Controlls the depth of the sub layer (usually dirt).
  sub_layer_map: OctavedNoise<OpenSimplexNoise, 3>,

  global_chunk_placers: Vec<Box<dyn ChunkPlacer>>,
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
    (0.46, 0.0),
    (0.48, -16.0),
    (0.50, -22.0),
    (0.52, -16.0),
    (0.54, 0.0),
    (0.60, 0.0),
    (0.70, 128.0),
    (1.00, 256.0),
  ]);

  pub static ref HEIGHT_IMPACT: Spline<&'static [(f64, f64)]> = Spline::new(&[
    (0.00, 1.0),
    (0.01, 0.0),
    (0.45, 0.0),
    (0.55, 1.0),
    (1.00, 1.0),
  ]);
}

const VILLAGES: bool = false;

impl WorldBiomes {
  pub fn new(info: &BlockInfoSupplier, seed: u64) -> Self {
    WorldBiomes {
      // this is dumb but it makes rustfmt look nicer.
      seed,

      composition_lookup: CompositionLookup::new(),
      biome_override: false,

      cave: CaveCarver::new(info, seed),
      structure: StructureGenerator::new(seed),

      temperature_map: OctavedNoise::new(seed, 1.0 / 2048.0),
      humidity_map: OctavedNoise::new(seed, 1.0 / 4096.0),

      continentalness_map: OctavedNoise::new(seed, 1.0 / 1024.0),
      peaks_valleys_map: OctavedNoise::new(seed, 1.0 / 256.0),
      erosion_map: OctavedNoise::new(seed, 1.0 / 2048.0),
      variance_map: ShiftedNoise::new(
        VoronoiNoise::new(seed, 128),
        OpenSimplexNoise::new(seed),
        1.0,
        1.0,
      ),

      density_map: OctavedNoise::new(seed, 1.0 / 64.0),

      sub_layer_map: OctavedNoise::new(seed, 1.0 / 20.0),

      global_chunk_placers: vec![
        Box::new(chunk_placer::Ore {
          ore:           block![coal_ore],
          avg_per_chunk: 4.0,
          size:          4..=12,
          height:        0..=128,
          width:         1.5,
        }),
        Box::new(chunk_placer::Ore {
          ore:           block![iron_ore],
          avg_per_chunk: 3.0,
          size:          4..=8,
          height:        0..=64,
          width:         1.5,
        }),
        Box::new(chunk_placer::Ore {
          ore:           block![gold_ore],
          avg_per_chunk: 2.0,
          size:          4..=8,
          height:        0..=32,
          width:         1.0,
        }),
        Box::new(chunk_placer::Ore {
          ore:           block![redstone_ore],
          avg_per_chunk: 1.0,
          size:          4..=12,
          height:        0..=32,
          width:         1.0,
        }),
        Box::new(chunk_placer::Ore {
          ore:           block![lapis_ore],
          avg_per_chunk: 1.0,
          size:          2..=6,
          height:        0..=16,
          width:         0.5,
        }),
        Box::new(chunk_placer::Ore {
          ore:           block![diamond_ore],
          avg_per_chunk: 1.0,
          size:          2..=6,
          height:        0..=16,
          width:         0.5,
        }),
      ],
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
    let impact = HEIGHT_IMPACT.sample::<Cosine>(c / 128.0);
    let p = PEAKS_VALLEYS.sample::<Cosine>(self.sample_peaks_valleys(pos));
    let e = EROSION.sample::<Cosine>(self.sample_erosion(pos));

    fn lerp(a: f64, b: f64, t: f64) -> f64 { a * (1.0 - t) + b * t }
    let e = lerp(0.2, e, impact);
    let p = lerp(8.0, p, impact);

    ((((c - 64.0) * 4.0) + 64.0) + p - 64.0) * e + 64.0
  }
}

impl Generator for WorldBiomes {
  fn generate_base(&self, ctx: &Context, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    if (0..=8).contains(&chunk_pos.x()) {
      return;
    }

    for rel_x in 0..16_u8 {
      for rel_z in 0..16_u8 {
        let pos = chunk_pos.min_block_pos() + Pos::new(rel_x.into(), 0, rel_z.into());

        // let height = self.height_at(pos) as i32;
        // let biome = self.choose_biome(seed, pos);
        let mut info = self.height_info(pos);

        if info.max_height() < 64.0 {
          for y in 0..=63 {
            let pos = pos.with_y(y);

            info.move_to(pos);
            if info.underground() {
              chunk.set(pos.chunk_rel(), ctx.blocks.encode(block![stone]));
            } else {
              chunk.set(pos.chunk_rel(), ctx.blocks.encode(block![water]));
            }
          }
        } else {
          for y in 0..=255 {
            let pos = pos.with_y(y);

            info.move_to(pos);
            if info.underground() {
              chunk.set(pos.chunk_rel(), ctx.blocks.encode(block![stone]));
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
    self.generate_chunk_placers(&ctx.blocks, chunk, chunk_pos);

    if VILLAGES {
      self.structure.generate(&ctx.blocks, chunk, chunk_pos);
    }
  }

  fn decorate(&self, world: &mut PartialWorld, chunk_pos: ChunkPos) {
    if (-1..=9).contains(&chunk_pos.x()) {
      return;
    }

    // TODO: Maybe make this 3D as well? Not sure if we want underground trees or
    // anything.

    if VILLAGES {
      self.structure.decorate(world, chunk_pos);
    }

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
      biome.decorate(&mut rng, chunk_pos, world, |pos| {
        let rel_x = pos.x - chunk_pos.min_block_pos().x;
        let rel_z = pos.z - chunk_pos.min_block_pos().z;
        biome_names[rel_x as usize][rel_z as usize] == biome.name
      });
    }

    world.set(chunk_pos.min_block_pos() + Pos::new(0, 6, 0), block![dirt]);
  }
}

impl WorldBiomes {
  fn generate_top_layer(
    &self,
    block_info: &BlockInfoSupplier,
    chunk: &mut Chunk,
    chunk_pos: ChunkPos,
  ) {
    // FIXME: Remove this and use a chunk placer instead.

    const SEA_LEVEL: i32 = 64;

    // For each column in the chunk, fill in the top layers.
    for x in 0..16 {
      for z in 0..16 {
        let pos = chunk_pos.min_block_pos() + Pos::new(x, 0, z);
        let sub_layer_depth = self.sample_sub_layer_depth(pos);

        let mut info = self.height_info(pos);

        let mut depth = 0;
        let mut layer = 0;

        let mut underwater = false;

        let biome = self.choose_surface_biome(pos);

        let min_height = (info.min_height as i32).min(40);
        for y in (min_height..=info.max_height as i32).rev() {
          let pos = pos.with_y(y);
          let rel_pos = pos.chunk_rel();

          info.move_to(pos);
          if info.underground() {
            depth += 1;
          } else {
            depth = 0;
          }

          if y < SEA_LEVEL && layer == 0 && depth == 0 {
            underwater = true;
          }

          let layers = if underwater { &biome.underwater_layers } else { &biome.layers };
          let mut current_layer = &layers[layer];
          let current_layer_depth = current_layer.sample_depth(sub_layer_depth);

          if depth > current_layer_depth {
            layer += 1;
            depth = 0;

            if layer >= layers.len() {
              break;
            }

            current_layer = &layers[layer];
          }

          if block_info.decode(chunk.get(rel_pos)) == block![stone] {
            // Special case: if the top layer is grass, always place grass if there is air
            // above (this makes cave entrances look nice.)
            if biome.top_block().block == block![grass]
              && chunk.get(rel_pos.with_y(y + 1)) == StateId::AIR
              && !underwater
            {
              chunk.set(rel_pos, block_info.encode(block![grass]));
            } else {
              chunk.set(rel_pos, block_info.encode(current_layer.state));
            }
          }
        }
      }
    }
  }

  fn sample_sub_layer_depth(&self, pos: Pos) -> f64 {
    self.sub_layer_map.generate(pos.x as f64, pos.z as f64)
  }

  fn generate_chunk_placers(
    &self,
    info: &BlockInfoSupplier,
    chunk: &mut Chunk,
    chunk_pos: ChunkPos,
  ) {
    // The length of this list is how many total biomes we support in a single
    // chunk. If there are more biomes than this, the extra ones will not be
    // decorated. This is an optimization to avoid allocating here.
    let mut biome_index = 0;
    let mut biome_set = [Option::<(&BiomeBuilder, TemporaryBiome)>::None; 16];

    let mut chunk = BiomeCachedChunk::new(info, chunk);

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

    for placer in &self.global_chunk_placers {
      placer.place(&mut chunk, &mut Rng::new(self.seed), chunk_pos);
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
      if self.min_height > self.max_height {
        // Special case for oceans.
        self.pos.y < self.max_height as i32
      } else if self.pos.y < self.min_height as i32 {
        true
      } else if self.pos.y >= self.max_height as i32 {
        false
      } else {
        let noise = self.world.density_map.generate_3d(
          self.pos.x as f64,
          self.pos.y as f64,
          self.pos.z as f64,
        ) * 0.5
          + 0.5;
        let limit = (self.pos.y as f64 - self.min_height) / (self.max_height - self.min_height);

        noise > limit
      }
    })
  }
}
