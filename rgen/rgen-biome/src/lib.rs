use biome::IdContext;
use rgen_base::{Block, Blocks, Chunk, ChunkPos, ChunkRelPos, Pos};
use rgen_placer::{
  grid::PointGrid,
  noise::{NoiseGenerator, NoiseGenerator3D, OctavedNoise, PerlinNoise},
  Rng,
};
use rgen_spline::{Cosine, Spline};
use rgen_world::{Context, PartialWorld};
use table::Tables;

mod biome;
mod builder;
mod lookup;
mod table;

pub use builder::BiomeBuilder;

pub struct WorldBiomes {
  tables:         Tables,
  biome_override: bool,

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

  /// Variance determines which biome to pick out of a list. Its basically
  /// random.
  variance_map: OctavedNoise<PerlinNoise>,

  density_map: OctavedNoise<PerlinNoise>,

  /// Controlls the depth of the sub layer (usually dirt).
  sub_layer_map: OctavedNoise<PerlinNoise>,

  noodle_cave_grid: PointGrid,

  /// Noodle caves are the long thin tunnels, the "normal" caves.
  noodle_cave_map:    OctavedNoise<PerlinNoise>,
  noodle_density_map: OctavedNoise<PerlinNoise>,
  /// Cheese caves are the big caverns.
  cheese_cave_map:    OctavedNoise<PerlinNoise>,
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
    (0.00, 1.0),
    (0.01, 0.8),
    (0.15, 0.7),
    (0.26, 0.5),
    (0.40, 0.3),
    (0.81, 0.2),
    (0.91, 0.1),
    (1.00, 0.0),
  ]);

  pub static ref PEAKS_VALLEYS: Spline<&'static [(f64, f64)]> = Spline::new(&[
    (0.00, 16.0),
    (0.40, 8.0),
    (0.47, 2.0),
    (0.50, 0.0),
    (0.53, 2.0),
    (0.60, 8.0),
    (1.00, 16.0),
  ]);
}

impl WorldBiomes {
  pub fn new(blocks: &Blocks, biome_ids: &rgen_base::Biomes) -> Self {
    let ctx = IdContext { biomes: biome_ids, blocks };

    WorldBiomes {
      tables:         Tables::new(&ctx),
      biome_override: false,

      temperature_map: OctavedNoise { octaves: 8, freq: 1.0 / 2048.0, ..Default::default() },
      humidity_map:    OctavedNoise { octaves: 8, freq: 1.0 / 4096.0, ..Default::default() },

      continentalness_map: OctavedNoise { octaves: 8, freq: 1.0 / 1024.0, ..Default::default() },
      peaks_valleys_map:   OctavedNoise { octaves: 8, freq: 1.0 / 256.0, ..Default::default() },
      erosion_map:         OctavedNoise { octaves: 8, freq: 1.0 / 2048.0, ..Default::default() },
      variance_map:        OctavedNoise { octaves: 8, freq: 1.0 / 512.0, ..Default::default() },

      density_map: OctavedNoise { octaves: 5, freq: 1.0 / 64.0, ..Default::default() },

      sub_layer_map: OctavedNoise { octaves: 3, freq: 1.0 / 20.0, ..Default::default() },

      noodle_cave_grid:   PointGrid::new(),
      noodle_cave_map:    OctavedNoise { octaves: 2, freq: 1.0 / 64.0, ..Default::default() },
      noodle_density_map: OctavedNoise { octaves: 2, freq: 1.0 / 16.0, ..Default::default() },
      cheese_cave_map:    OctavedNoise { octaves: 4, freq: 1.0 / 128.0, ..Default::default() },
    }
  }

  pub fn sample_continentalness(&self, seed: u64, pos: Pos) -> f64 {
    (self.continentalness_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5)
      .clamp(0.0, 1.0)
  }

  pub fn sample_peaks_valleys(&self, seed: u64, pos: Pos) -> f64 {
    let seed = seed.wrapping_add(1);

    (self.peaks_valleys_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5).clamp(0.0, 1.0)
  }

  pub fn sample_erosion(&self, seed: u64, pos: Pos) -> f64 {
    let seed = seed.wrapping_add(2);

    (self.erosion_map.generate(pos.x as f64, pos.z as f64, seed) * 0.5 + 0.5).clamp(0.0, 1.0)
  }

  pub fn sample_height(&self, seed: u64, pos: Pos) -> f64 {
    let c = CONTINENTALNESS.sample::<Cosine>(self.sample_continentalness(seed, pos));
    let p = PEAKS_VALLEYS.sample::<Cosine>(self.sample_peaks_valleys(seed, pos));
    let e = EROSION.sample::<Cosine>(self.sample_erosion(seed, pos));

    (c + p - 64.0) * e + 64.0
  }

  pub fn generate_base(&self, seed: u64, ctx: &Context, chunk: &mut Chunk, chunk_pos: ChunkPos) {
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
        let max_height = self.sample_height(seed, pos);
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
              self.density_map.generate_3d(pos.x as f64, pos.y as f64, pos.z as f64, seed) * 0.5
                + 0.5;

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

    // TODO: Don't carve through water.
    // self.carve_cave(seed, chunk, chunk_pos);

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
          if block != Block::AIR && ![blocks.leaves.block].contains(&block) {
            break;
          }
          y -= 1;
        }
        let rel_pos = rel_pos.with_y(y);
        let pos =
          chunk_pos.min_block_pos() + Pos::new(rel_pos.x().into(), rel_pos.y(), rel_pos.z().into());

        let biome = self.choose_biome(seed, pos);

        if chunk.get(rel_pos) == blocks.stone.block {
          chunk.set_state(rel_pos, biome.top_block);
        }

        for depth in 1..self.sample_sub_layer_depth(seed, pos) {
          let rel_pos = rel_pos.with_y(rel_pos.y() - depth);
          if chunk.get(rel_pos) == blocks.stone.block {
            chunk.set_state(rel_pos, biome.sub_layer);
          }
        }
      }
    }
  }

  fn sample_sub_layer_depth(&self, seed: u64, pos: Pos) -> i32 {
    let seed = seed.wrapping_add(10);

    let noise = self.sub_layer_map.generate(pos.x as f64, pos.z as f64, seed);
    let depth = (noise * 2.0 + 3.0).round() as i32;
    depth
  }

  pub fn decorate(
    &self,
    blocks: &Blocks,
    seed: u64,
    world: &mut PartialWorld,
    chunk_pos: ChunkPos,
  ) {
    let mut biome_names = [[""; 16]; 16];
    // The length of this list is how many total biomes we support in a single
    // chunk. If there are more biomes than this, the extra ones will not be
    // decorated. This is an optimization to avoid allocating here.
    let mut biome_index = 0;
    let mut biome_set = [Option::<&BiomeBuilder>::None; 16];

    for x in 0..16 {
      for z in 0..16 {
        let pos = chunk_pos.min_block_pos() + Pos::new(x, 0, z);
        let biome = self.choose_biome(seed, pos);
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
      let mut rng = Rng::new(seed);
      biome.decorate(blocks, &mut rng, chunk_pos, world, |pos| {
        let rel_x = pos.x - chunk_pos.min_block_pos().x;
        let rel_z = pos.z - chunk_pos.min_block_pos().z;
        biome_names[rel_x as usize][rel_z as usize] == biome.name
      });
    }
  }

  fn carve_cave(&self, seed: u64, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    self.carve_noodle_cave(seed, chunk, chunk_pos);
    self.carve_cheese_cave(seed, chunk, chunk_pos);
  }

  fn carve_noodle_cave(&self, seed: u64, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    let radius = 100;
    let scale = 48.0;

    let min_pos = chunk_pos.min_block_pos();
    let cave_min_x = ((min_pos.x - radius) as f64) / scale;
    let cave_min_z = ((min_pos.z - radius) as f64) / scale;
    let cave_max_x = ((min_pos.x + 16 + radius) as f64) / scale;
    let cave_max_z = ((min_pos.z + 16 + radius) as f64) / scale;

    let points =
      self.noodle_cave_grid.points_in_area(seed, cave_min_x, cave_min_z, cave_max_x, cave_max_z);
    for point in points {
      let mut pos = ((point.0 * scale), 64.0, (point.1 * scale));

      // A seed unique to this cave.
      let cave_seed =
        seed ^ (((pos.0 * 2048.0).round() as u64) << 8) ^ (((pos.2 * 2048.0).round() as u64) << 16);

      for _ in 0..100 {
        let dx = self.noodle_cave_map.generate_3d(pos.0, pos.1, pos.2, cave_seed.wrapping_add(1));
        let dy = self.noodle_cave_map.generate_3d(pos.0, pos.1, pos.2, cave_seed.wrapping_add(2));
        let dz = self.noodle_cave_map.generate_3d(pos.0, pos.1, pos.2, cave_seed.wrapping_add(3));

        let dy = dy / 2.0;

        let radius =
          (self.noodle_cave_map.generate_3d(pos.0, pos.1, pos.2, cave_seed.wrapping_add(4)) * 0.5
            + 0.5)
            * 4.0
            + 1.0;
        let radius_squared = (radius * radius).round() as i32;
        let max_radius = radius.ceil() as i32;

        for _ in 0..5 {
          pos.0 += dx;
          pos.1 += dy;
          pos.2 += dz;

          let pos = Pos::new(pos.0 as i32, pos.1 as i32, pos.2 as i32);
          for y in -max_radius..=max_radius {
            for z in -max_radius..=max_radius {
              for x in -max_radius..=max_radius {
                let r = x * x + y * y + z * z;
                if r > radius_squared {
                  continue;
                }
                let dist_to_center = r as f64 / radius_squared as f64;

                let pos = Pos::new(pos.x + x, pos.y + y, pos.z + z);
                if pos.in_chunk(chunk_pos) {
                  let density = self.noodle_density_map.generate_3d(
                    pos.x as f64,
                    pos.y as f64,
                    pos.z as f64,
                    seed,
                  ) * 0.4
                    + 0.6;
                  if density > dist_to_center {
                    chunk.set(pos.chunk_rel(), Block::AIR);
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  fn carve_cheese_cave(&self, seed: u64, chunk: &mut Chunk, chunk_pos: ChunkPos) {
    let seed = seed.wrapping_add(200);

    for rel_x in 0..16_u8 {
      for rel_z in 0..16_u8 {
        let pos = chunk_pos.min_block_pos() + Pos::new(rel_x.into(), 0, rel_z.into());

        for y in 0..=255 {
          let pos = pos.with_y(y);
          let noise =
            self.cheese_cave_map.generate_3d(pos.x as f64, pos.y as f64 * 4.0, pos.z as f64, seed)
              * 0.5
              + 0.5;
          if noise < 0.1 {
            chunk.set(pos.chunk_rel(), Block::AIR);
          }
        }
      }
    }
  }
}
