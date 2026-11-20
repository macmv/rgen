use cave::CaveCarver;
use rgen_base::{Chunk, ChunkPos, ChunkRelPos, Pos, StateId, block};
use rgen_placer::{
  BiomeCachedChunk, BiomeColumn, ChunkPlacer, Rng, TemporaryBiome, chunk_placer,
  noise::{
    NoiseGenerator, NoiseGenerator3D, OctavedNoise, OpenSimplexNoise, PerlinNoise, SeededNoise,
    ShiftedNoise, VoronoiNoise,
  },
};
use rgen_spline::Spline;
use rgen_world::{BlockInfoSupplier, Context, Generator, PartialWorld};
use structure::StructureGenerator;
use table::CompositionLookup;

/// Whether a part of the generator is on. This is always `true` unless the
/// `inspector` feature is on, which lets tools turn parts off.
#[cfg(feature = "inspector")]
macro_rules! enabled {
  ($toggle:ident) => {
    $crate::toggles::enabled($crate::toggles::Toggle::$toggle)
  };
}
#[cfg(not(feature = "inspector"))]
macro_rules! enabled {
  ($toggle:ident) => {
    true
  };
}

#[cfg(feature = "inspector")]
macro_rules! placer_enabled {
  ($name:expr) => {
    $crate::toggles::placer_enabled($name)
  };
}
#[cfg(not(feature = "inspector"))]
macro_rules! placer_enabled {
  ($name:expr) => {
    true
  };
}

mod biome;
mod builder;
mod cave;
mod continents;
mod drainage;
pub use drainage::Drainage;
pub use continents::ELEVATION_SPACING;
mod feature;
mod lookup;
mod hilliness;
mod splines;
mod structure;
mod table;
mod terrain;
mod watershed;
pub use watershed::DrainageVector;
#[cfg(feature = "inspector")]
pub mod toggles;

pub use builder::BiomeBuilder;
pub use table::{ClimateType, GeographicType};
#[cfg(feature = "inspector")]
pub use splines::set_splines;
pub use splines::{Keys, Splines};
pub use terrain::{HeightParts, RawNoise, TerrainNoise, height_parts};

#[macro_use]
extern crate puffin;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

pub struct WorldBiomes {
  seed: u64,

  composition_lookup: CompositionLookup,

  cave:      CaveCarver,
  structure: StructureGenerator,

  temperature_map: OctavedNoise<PerlinNoise, 8>,
  humidity_map:    OctavedNoise<PerlinNoise, 8>,

  /// The continentalness, peaks and valleys, and erosion maps. See the docs on
  /// the sample functions below for what each one means.
  terrain: TerrainNoise,

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
    // Deepen the offshore basins by ~5.6 blocks after the ocean height blend.
    // The minimum base floor is around Y=36; retain the shallow coastal key.
    (0.01, 28.0),
    (0.15, 31.0),
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

impl WorldBiomes {
  /// The name of every decoration that biomes place, for toggling them. Each
  /// one is turned on and off with `toggles::set_placer`.
  #[cfg(feature = "inspector")]
  pub fn decoration_names(&self) -> Vec<&'static str> {
    let lookup = &self.composition_lookup;
    let mut names: Vec<&'static str> = lookup
      .blank
      .iter()
      .chain(lookup.lookup.values().flatten())
      .flat_map(|biome| biome.placer_names())
      .collect();
    names.sort_unstable();
    names.dedup();
    names
  }

  pub fn new(info: &BlockInfoSupplier, seed: u64) -> Self {
    WorldBiomes {
      // this is dumb but it makes rustfmt look nicer.
      seed,

      composition_lookup: CompositionLookup::new(seed),

      cave: CaveCarver::new(info, seed),
      structure: StructureGenerator::new(seed),

      temperature_map: OctavedNoise::new(seed, 1.0 / 2048.0),
      humidity_map: OctavedNoise::new(seed, 1.0 / 4096.0),

      terrain: TerrainNoise::new(seed),
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
    self.terrain.sample_continentalness(pos.x as f64, pos.z as f64)
  }

  pub fn sample_peaks_valleys(&self, pos: Pos) -> f64 {
    self.terrain.sample_peaks_valleys(pos.x as f64, pos.z as f64)
  }

  pub fn sample_erosion(&self, pos: Pos) -> f64 {
    self.terrain.sample_erosion(pos.x as f64, pos.z as f64)
  }

  pub fn sample_river_distance(&self, pos: Pos) -> f64 {
    // This will be between 0.0 and 0.02 when there is a river, and any value
    // higher will be outside of a river.
    let distance_to_river = (self.sample_peaks_valleys(pos) - 0.5).abs();

    // So, return a value from 0.0 to 1.0 for the range 0.0 to 0.16, so that
    // caves can smooth the transition over to rivers.
    if distance_to_river > 0.16 { 1.0 } else { distance_to_river / 0.16 }
  }

  /// Regional planning elevation in block Y coordinates.
  pub fn broad_elevation(&self, pos: Pos) -> f64 {
    if self.is_continent_land(pos) { continents::broad_land_height(&self.terrain,self.seed, pos.x, pos.z) } else { 48.0 }
  }

  pub fn is_continent_land(&self, pos: Pos) -> bool { continents::land(self.seed, pos.x, pos.z) }

  pub fn sample_height(&self, pos: Pos) -> f64 { self.height_parts(pos).height }

  /// Every intermediate value of the height function at `pos`.
  pub fn height_parts(&self, pos: Pos) -> HeightParts {
    if feature::FLAT_TERRAIN { return self.watershed_column(pos).0; }
    splines::current_height_parts(self.terrain.sample(pos.x as f64, pos.z as f64))
  }

  fn watershed_column(&self, pos: Pos) -> (HeightParts, GeographicType) {
    let land = self.is_continent_land(pos);
    let (distance, cliff) = continents::coastal_profile(self.seed,pos.x,pos.z,land);
    let width = if land && distance < 58.0 {
      32.0 + 26.0 * continents::field(self.seed ^ 0xaf63, pos.x, pos.z, 73)
    } else { 58.0 };
    let t = (distance / width).min(1.0);
    let inland = t*t*(3.0-2.0*t);
    // Even cliff coasts must meet the waterline. Applying the full cliff
    // weight at distance zero creates a sheer wall around an entire island.
    let cliff_rise = distance / (distance + 20.0);
    let shore = inland + (1.0-inland)*cliff*cliff_rise;
    let mut density = 0.0;
    let mut steep_coast = 0.0;
    let mut natural = if land {
      let belt = continents::mountain_belt(self.seed,pos.x,pos.z);
      let rolling = continents::broad_land_height(&self.terrain,self.seed,pos.x,pos.z)
        + continents::rolling_detail(self.seed,pos.x,pos.z) * continents::relief_strength(&self.terrain,pos.x,pos.z);
      let mut envelope = rolling;
      if belt > 0.0 {
        let mut raw = self.terrain.sample_unrouted(pos.x as f64,pos.z as f64);
        raw.continentalness = 0.38 + raw.continentalness*0.62;
        let old = terrain::unrouted_height(&CONTINENTALNESS,&EROSION,&PEAKS_VALLEYS,&HEIGHT_IMPACT,raw);
        envelope += belt*(old.height-64.0).max(0.0);
        density = belt*shore;
      }
      // Limit coastal relief by the space available to climb inland. Ease
      // out the limit before the distance search reaches its 58-block cap.
      let relief = (envelope.clamp(64.0,253.0)-64.0)*shore;
      // Estimate the rise across the coastal slope before the beach flattens it.
      let grade = (envelope.clamp(64.0,253.0)-64.0) / width;
      let steep = ((grade-0.45)/0.65).clamp(0.0,1.0);
      steep_coast = steep*steep*(3.0-2.0*steep);
      let slope = 0.55 + 0.85*cliff;
      let coast_limit = distance*slope;
      let transition = ((distance-32.0)/26.0).clamp(0.0,1.0);
      let release = transition*transition*(3.0-2.0*transition);
      65.0 + relief.min(coast_limit) + (relief-coast_limit).max(0.0)*release
    } else {
      // The continent mask owns the coastline. Sample the ocean portion of
      // the original height spline for offshore basins and submerged rises.
      let x = pos.x as f64;
      let z = pos.z as f64;
      let ocean_c = 0.01 + 0.32 * self.terrain.sample_continentalness(x, z);
      let basin = CONTINENTALNESS.sample::<rgen_spline::Cosine>(ocean_c);
      let ridges = 8.0 * (self.terrain.sample_peaks_valleys(x, z) - 0.5);
      let offshore = 64.0 + 0.8 * (basin - 64.0) + 1.6 + ridges;
      63.0 - 14.0 * shore + (offshore.min(62.0) - 49.0) * inland
    };
    let (carved, river_bank) = if land { self.carve_ground(pos,natural-1.0) } else { (natural-1.0, false) };
    let mut beach = false;
    // The coarse coast distance shapes the hills. Resolve the dry beach at
    // block resolution so even a diagonal shoreline has a level approach.
    if land && distance < 18.0 {
      let variation = continents::field(self.seed ^ 0xb34c, pos.x, pos.z, 80);
      let gentle_width = 4.0 + 2.0*variation;
      let steep_width = 1.0 + variation;
      let beach_width = (gentle_width + (steep_width-gentle_width)*steep_coast).round() as i32;
      let shore_distance = continents::beach_distance(self.seed, pos.x, pos.z, beach_width + 6, true);
      beach = shore_distance <= beach_width as f64;
      let beyond_beach = (shore_distance-beach_width as f64).max(0.0);
      let rise = beyond_beach/6.0;
      let blend = rise*rise*(3.0-2.0*rise);
      let edge = 64.0 + beyond_beach*steep_coast;
      natural = edge + (natural-edge)*blend;
      density *= blend;
    } else if !land && distance < 18.0 {
      // Keep the first submerged block one step below the beach, then
      // gradually rejoin the offshore profile across the shallows.
      let offshore = continents::beach_distance(self.seed, pos.x, pos.z, 12, false);
      let t = ((offshore-1.0)/11.0).clamp(0.0,1.0);
      natural = 63.0 + (natural-63.0)*t*t*(3.0-2.0*t);
    }
    let height = if land { carved.min(natural-1.0).floor()+1.0 } else { natural.floor() };
    let geo = if !land { GeographicType::Ocean }
      else if river_bank && natural - 1.0 >= 93.0 { GeographicType::Canyon }
      else if river_bank { GeographicType::River }
      else if beach { GeographicType::Beach }
      else if natural >= 112.0 { GeographicType::Mountains }
      else if natural >= 84.0 { GeographicType::Hills }
      else { GeographicType::Standard };
    (HeightParts { c:natural, impact:shore, p:natural-64.0, e:1.0, height,
      natural_height:natural, solid_weight:1.0-density }, geo)
  }
}

const ICING_SALT: u64 = 0x1c17_c0de_5eed_1ced;

impl Generator for WorldBiomes {
  fn generate_base(&self, ctx: &Context, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    profile_function!();

    if feature::SUPERFLAT {
      for x in 0..16 {
        for z in 0..16 {
          let rel_pos = ChunkRelPos::new(x, 25, z);

          chunk.set(rel_pos, ctx.blocks.encode(block![grass]));
          let ground_pos = rel_pos;
          for i in 1..62 {
            chunk
              .set(ground_pos.with_y(ground_pos.y() - (1 * i)), ctx.blocks.encode(block![stone]));
          }
          chunk.add_surface(rel_pos);
        }
      }

      return;
    }

    if feature::DEBUG_ORES && (0..=8).contains(&chunk_pos.x()) {
      return;
    }

    self.generate_stone(ctx, chunk, chunk_pos);

    if !feature::FLAT_TERRAIN {
      self.cave.carve(self, chunk, chunk_pos);
    }

    self.generate_top_layer(&ctx.blocks, chunk, chunk_pos);

    self.generate_chunk_placers(&ctx.blocks, chunk, chunk_pos);

    if feature::VILLAGES {
      self.structure.generate(&ctx.blocks, chunk, chunk_pos);
    }
  }

  fn ice(&self, ctx: &Context, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    profile_function!();

    if feature::SUPERFLAT {
      return;
    }

    if feature::DEBUG_ORES && (-1..=9).contains(&chunk_pos.x()) {
      return;
    }

    if !enabled!(Decorations) {
      return;
    }

    let mut biome_set = TemporaryBiomeSet::new();
    let mut chunk = BiomeCachedChunk::new(&ctx.blocks, chunk);

    {
      profile_scope!("icing biome setup");

      for x in 0..16 {
        for z in 0..16 {
          let biome = self.choose_surface_biome(chunk_pos.min_block_pos() + Pos::new(x, 0, z));

          // Icing only cares about the surface, so there is no need to work out
          // where the caves are. A `min_height` of 0 makes every
          // column act as a surface column.
          let id = biome_set.add(biome);
          let column = BiomeColumn { surface: id, cave: id, min_height: 0 };

          chunk.set_column(ChunkRelPos::new(x as u8, 0, z as u8), column);
        }
      }
    }

    for (biome, id) in biome_set.set.into_iter().flatten() {
      // Salted, so that icing doesn't share seeds with the chunk placers.
      let mut rng = Rng::new(self.seed ^ ICING_SALT);
      chunk.set_active(id);
      biome.ice(&mut rng, &mut chunk, chunk_pos);
    }
  }

  fn decorate(&self, world: &mut PartialWorld, chunk_pos: ChunkPos) {
    profile_function!();

    //if feature::SUPERFLAT {
    //  return;
    //}

    if feature::DEBUG_ORES && (-1..=9).contains(&chunk_pos.x()) {
      return;
    }

    // TODO: Maybe make this 3D as well? Not sure if we want underground trees
    // or anything.

    if feature::VILLAGES {
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

        // `biome_set` acts like a set, so we need to check if this is a new
        // biome or not. Note that this means every biome name _must_ be
        // unique.
        if !biome_set[..biome_index].iter().any(|b| b.is_some_and(|b| b.name == biome.name))
          && biome_index < biome_set.len()
        {
          biome_set[biome_index] = Some(biome);
          biome_index += 1;
        }
      }
    }

    for biome in biome_set.into_iter().flatten().filter(|_| enabled!(Decorations)) {
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
  fn generate_stone(&self, ctx: &Context, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    profile_function!();

    let stone = ctx.blocks.encode(block![stone]);
    let water = ctx.blocks.encode(block![water]);

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
              chunk.set(pos.chunk_rel(), stone);
            } else {
              chunk.set(pos.chunk_rel(), water);
            }
          }
        } else {
          for y in 0..info.min_height as i32 {
            chunk.set(pos.chunk_rel().with_y(y), stone);
          }

          for y in info.min_height as i32..info.max_height as i32 {
            let pos = pos.with_y(y);

            info.move_to(pos);
            if info.underground() {
              chunk.set(pos.chunk_rel(), stone);
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
  }

  fn generate_top_layer(
    &self,
    block_info: &BlockInfoSupplier,
    chunk: &mut Chunk,
    chunk_pos: ChunkPos,
  ) {
    profile_function!();

    // FIXME: Remove this and use a chunk placer instead.

    // For each column in the chunk, fill in the top layers.
    for x in 0..16 {
      for z in 0..16 {
        let pos = chunk_pos.min_block_pos() + Pos::new(x, 0, z);
        let sub_layer_depth = self.sample_sub_layer_depth(pos);

        let mut info = self.height_info(pos);

        let mut depth = 0;
        let mut layer = 0;
        let mut air_above: u8 = 255;

        // TODO: Fix.
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
            if depth > 0 {
              // On the boundry from stone to air, reset `air_above`.
              air_above = 0;
            }

            depth = 0;
            layer = 0;
            air_above = air_above.saturating_add(1);

            if y <= 63 {
              underwater = true;
            }
            continue;
          }

          let layers = if underwater { &biome.underwater_layers } else { &biome.layers };
          if layer >= layers.len() || air_above < 5 {
            continue;
          }
          let mut current_layer = &layers[layer];
          let current_layer_depth = current_layer.sample_depth(sub_layer_depth);

          if depth > current_layer_depth {
            layer += 1;
            depth = 0;

            if layer >= layers.len() {
              continue;
            }

            current_layer = &layers[layer];
          }

          if block_info.decode(chunk.get(rel_pos)) == block![stone] {
            if depth == 1 && layer == 0 {
              chunk.add_surface(rel_pos);
            }

            // Special case: if the top layer is grass, always place grass if
            // there is air above (this makes cave entrances look
            // nice.)
            if !enabled!(SurfaceLayers) {
              // Leave the stone.
            } else if biome.top_block().block == block![grass]
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
    profile_function!();

    // The length of this list is how many total biomes we support in a single
    // chunk. If there are more biomes than this, the extra ones will not be
    // decorated. This is an optimization to avoid allocating here.
    let mut biome_set = TemporaryBiomeSet::new();

    let mut chunk = BiomeCachedChunk::new(info, chunk);

    {
      profile_scope!("biome cached chunk setup");

      for x in 0..16 {
        for z in 0..16 {
          // This is kinda restrictive, but helps performance a _lot_. We also
          // don't really want biomes to change on the Y axis, as that
          // causes weirdness when building with grass and such. So we
          // can limit ourselves to a single surface biome and a
          // single cave biome per column.
          let surface_biome =
            self.choose_surface_biome(chunk_pos.min_block_pos() + Pos::new(x, 0, z));
          let cave_biome = self.choose_cave_biome(chunk_pos.min_block_pos() + Pos::new(x, 0, z));

          let info = self.height_info(chunk_pos.min_block_pos() + Pos::new(x, 0, z));

          let column = BiomeColumn {
            surface:    biome_set.add(surface_biome),
            cave:       biome_set.add(cave_biome),
            min_height: info.min_height as i32,
          };

          chunk.set_column(ChunkRelPos::new(x as u8, 0, z as u8), column);
        }
      }
    }

    {
      profile_scope!("biome chunk placers");
      for (biome, id) in biome_set.set.into_iter().flatten() {
        if !enabled!(BiomeChunkPlacers) {
          break;
        }
        let mut rng = Rng::new(self.seed);
        chunk.set_active(id);
        biome.generate(&mut rng, &mut chunk, chunk_pos);
      }
    }

    {
      profile_scope!("global chunk placers");
      for placer in self.global_chunk_placers.iter().filter(|_| enabled!(Ores)) {
        placer.place(&mut chunk, &mut Rng::new(self.seed), chunk_pos);
      }
    }
  }

  /// Diagnostic used by the preview tool: excavation must never create a
  /// solid block where the original density terrain had air. Deltas are
  /// excluded because their sediment deposition is intentional.
  pub fn audit_river_excavation(&self) -> usize {
    let mut checked = 0;
    let mut columns = 0;
    for x in -24..=24 {
      for z in -24..=24 {
        let pos = Pos::new(x*48,0,z*48);
        let parts = self.height_parts(pos);
        if parts.solid_weight > 0.0 || parts.height >= parts.natural_height { continue; }
        columns += 1;
        let natural_min = 64.0-parts.natural_height/128.0;
        let mut info = self.height_info(pos);
        for y in 60..=(parts.natural_height.ceil() as i32).min(255) {
          let p = pos.with_y(y);
          let natural_solid = if natural_min >= parts.natural_height {
            y < parts.natural_height as i32
          } else if y < natural_min as i32 { true }
          else if y >= parts.natural_height as i32 { false }
          else {
            let density = self.density_map.generate_3d(p.x as f64,y as f64,p.z as f64)*0.5+0.5;
            density > (y as f64-natural_min)/(parts.natural_height-natural_min)
          };
          info.move_to(p);
          assert!(!info.underground() || natural_solid,
            "river excavation added terrain at {:?}",p);
          checked += 1;
        }
      }
    }
    assert!(columns > 0 && checked > 0,"excavation audit found no carved columns");
    checked
  }

  pub fn height_info(&self, pos: Pos) -> HeightInfo<'_> {
    let mut info =
      HeightInfo { world: self, pos, max_height: 0.0, min_height: 0.0, density_height: 0.0, underground: None };
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
  density_height: f64,
  underground: Option<bool>,
}

impl HeightInfo<'_> {
  fn change_xz(&mut self) {
    let parts = self.world.height_parts(self.pos);
    self.max_height = parts.height;
    let natural = if feature::WATERSHEDS || feature::FLAT_TERRAIN { parts.natural_height } else { parts.height };
    let original_min = 64.0 - natural / 128.0;
    self.min_height = (original_min * (1.0 - parts.solid_weight)
      + self.max_height * parts.solid_weight).min(self.max_height);
    self.density_height = natural * (1.0 - parts.solid_weight)
      + self.max_height * parts.solid_weight;
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
      if self.min_height >= self.max_height {
        // Special case for oceans.
        self.pos.y < self.max_height as i32
      } else if self.pos.y < self.min_height as i32 {
        true
      } else if self.pos.y >= self.max_height as i32 {
        false
      } else if !enabled!(Noise3d) {
        // Everything under the height is solid.
        true
      } else {
        let noise = self.world.density_map.generate_3d(
          self.pos.x as f64,
          self.pos.y as f64,
          self.pos.z as f64,
        ) * 0.5
          + 0.5;
        let limit = (self.pos.y as f64 - self.min_height) / (self.density_height - self.min_height);

        noise > limit
      }
    })
  }
}

struct TemporaryBiomeSet<'a> {
  set:   [Option<(&'a BiomeBuilder, TemporaryBiome)>; 16],
  index: usize,
}

impl<'a> TemporaryBiomeSet<'a> {
  pub fn new() -> Self { TemporaryBiomeSet { set: [None; 16], index: 0 } }

  pub fn add(&mut self, biome: &'a BiomeBuilder) -> TemporaryBiome {
    match self.set[..self.index].iter().find(|b| b.is_some_and(|(b, _)| b.name == biome.name)) {
      Some(Some((_, id))) => *id,
      Some(None) => unreachable!(),
      None => {
        if self.index < 15 {
          let id = TemporaryBiome(self.index as u8);
          self.set[self.index] = Some((biome, id));
          self.index += 1;
          id
        } else {
          TemporaryBiome(15)
        }
      }
    }
  }
}
