use std::collections::HashMap;

use eframe::egui::Color32;
use rgen_biome::RawNoise;

use crate::{
  Splines,
  chunks::{CHUNK, ChunkUpdate, Chunks},
  real::TileBlocks,
};

pub const SEA_LEVEL: f64 = 64.0;

/// The width of a chunk, in blocks.
pub const TILE_BLOCKS: usize = 128;

const _: () = assert!(TILE_BLOCKS % CHUNK == 0);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Layer {
  /// Not a layer: blocks are colored by their kind.
  Blocks,
  Height,
  RawContinentalness,
  RawErosion,
  RawPeaksValleys,
  Continentalness,
  Erosion,
  PeaksValleys,
  HeightImpact,
  RiverDistance,
  Biomes,
}

impl Layer {
  pub const ALL: [Layer; 11] = [
    Layer::Blocks,
    Layer::Height,
    Layer::RawContinentalness,
    Layer::RawErosion,
    Layer::RawPeaksValleys,
    Layer::Continentalness,
    Layer::Erosion,
    Layer::PeaksValleys,
    Layer::HeightImpact,
    Layer::RiverDistance,
    Layer::Biomes,
  ];

  pub fn name(&self) -> &'static str {
    match self {
      Layer::Blocks => "Blocks",
      Layer::Height => "Final height",
      Layer::RawContinentalness => "Continentalness (noise)",
      Layer::RawErosion => "Erosion (noise)",
      Layer::RawPeaksValleys => "Peaks and valleys (noise)",
      Layer::Continentalness => "Continentalness (spline)",
      Layer::Erosion => "Erosion (spline, blended)",
      Layer::PeaksValleys => "Peaks and valleys (spline, blended)",
      Layer::HeightImpact => "Height impact",
      Layer::RiverDistance => "River distance",
      Layer::Biomes => "Biomes",
    }
  }

  pub fn value(&self, raw: RawNoise, parts: rgen_biome::HeightParts) -> f64 {
    match self {
      Layer::Blocks => 0.0,
      Layer::Height => parts.height,
      Layer::RawContinentalness => raw.continentalness,
      Layer::RawErosion => raw.erosion,
      Layer::RawPeaksValleys => raw.peaks_valleys,
      Layer::Continentalness => parts.c,
      Layer::Erosion => parts.e,
      Layer::PeaksValleys => parts.p,
      Layer::HeightImpact => parts.impact,
      // Matches `WorldBiomes::sample_river_distance`.
      Layer::RiverDistance => ((raw.peaks_valleys - 0.5).abs() / 0.16).min(1.0),
      // Biomes are colored directly, in `Grid::color`.
      Layer::Biomes => 0.0,
    }
  }

  /// The values that map to the ends of the color ramp. These are fixed, so
  /// that colors stay put while chunks load and splines change.
  pub fn range(&self) -> (f64, f64) {
    match self {
      Layer::Height => (30.0, 220.0),
      Layer::Continentalness => (0.0, 128.0),
      Layer::PeaksValleys => (-64.0, 256.0),
      _ => (0.0, 1.0),
    }
  }
}

/// The largest view distance. Chunks are generated in a circle of this many
/// chunks around the camera.
pub const MAX_RADIUS: i32 = 32;

/// How many chunks past the view distance a generated chunk is kept for. This
/// keeps chunks from being thrown out while panning back and forth.
pub const UNLOAD_MARGIN: i32 = 3;

/// The number of chunks along each side of the grid. It's big enough for the
/// window around the camera at the largest view distance, plus the chunks that
/// are kept past it.
pub const SIDE: usize = 2 * (MAX_RADIUS + UNLOAD_MARGIN) as usize + 1;

/// How many atlas slots to budget for each column of chunks. A column needs a
/// slot for each chunk with a mix of blocks in it: the ones the surface passes
/// through, and the ones with caves, ores or trees in them.
const SLOTS_PER_COLUMN: usize = 6;

/// The atlas slots needed to hold the terrain at a view distance, including the
/// chunks that are kept past it.
pub fn slots_for(radius: i32) -> usize {
  let columns = (TILE_BLOCKS / CHUNK).pow(2);
  tiles_in_range([0, 0], radius + UNLOAD_MARGIN).count() * columns * SLOTS_PER_COLUMN
}

/// The squared distance between two chunks, in chunks.
pub fn distance_sq(a: [i32; 2], b: [i32; 2]) -> i32 {
  let (dx, dz) = (a[0] - b[0], a[1] - b[1]);
  dx * dx + dz * dz
}

/// Whether a chunk is within `radius` chunks of `center`.
pub fn in_range(center: [i32; 2], radius: i32, tile: [i32; 2]) -> bool {
  distance_sq(center, tile) <= radius * radius
}

/// Every chunk within `radius` chunks of `center`.
pub fn tiles_in_range(center: [i32; 2], radius: i32) -> impl Iterator<Item = [i32; 2]> {
  (-radius..=radius)
    .flat_map(move |dz| (-radius..=radius).map(move |dx| [center[0] + dx, center[1] + dz]))
    .filter(move |&t| in_range(center, radius, t))
}

/// The noise and colors of one generated chunk, with one sample per block
/// column and x changing fastest.
struct TileData {
  /// The noise at each column. This is `[continentalness, peaks_valleys,
  /// erosion]`, which is kept as `f32` to save memory.
  raw:    Vec<[f32; 3]>,
  /// The color of each column's biome, as `0xrrggbb`.
  biomes: Vec<u32>,
  /// The color of each column in the layer being shown.
  colors: Vec<Color32>,
}

/// The sampled terrain: the chunks around the camera, each of which is
/// generated on its own.
///
/// The chunks are stored on a torus. The chunk at `(tx, tz)` always lives at
/// `(tx mod SIDE, tz mod SIDE)`, in both the CPU's and the GPU's copy, so
/// moving the camera never moves any terrain. Only the chunks that leave the
/// view distance are cleared. The window is the square of `SIDE` chunks that is
/// centered on the camera, and is the space the camera and ray marcher work in.
pub struct Grid {
  /// The number of columns along x and z.
  pub size: [usize; 2],
  /// The chunk at the window's minimum corner.
  window:   [i32; 2],

  tiles: HashMap<[i32; 2], TileData>,

  /// The same terrain as 3D chunks, for the ray marcher.
  pub chunks: Chunks,
}

/// Terrain to copy into the GPU's textures.
pub enum Upload {
  /// Sizes the textures for a new grid, and empties the chunks. The colors
  /// start out undefined, as only generated columns are ever drawn.
  Reset { size: [usize; 2], dims: [usize; 3] },
  /// Replaces the colors of one square of columns.
  Tile { x: usize, y: usize, size: usize, colors: Vec<u8> },
  /// Changes some chunks.
  Chunks(Vec<ChunkUpdate>),
}

fn pack(raw: RawNoise) -> [f32; 3] {
  [raw.continentalness as f32, raw.peaks_valleys as f32, raw.erosion as f32]
}

fn unpack(raw: [f32; 3]) -> RawNoise {
  RawNoise {
    continentalness: f64::from(raw[0]),
    peaks_valleys:   f64::from(raw[1]),
    erosion:         f64::from(raw[2]),
  }
}

fn colors_bytes(colors: &[Color32]) -> Vec<u8> {
  colors.iter().flat_map(|c| c.to_array()).collect()
}

impl Grid {
  /// Makes a grid where nothing has been generated. `brick_capacity` is the
  /// number of chunks the GPU can hold blocks for.
  pub fn new(brick_capacity: usize) -> Self {
    let size = [SIDE * TILE_BLOCKS; 2];
    Grid { size, window: [0, 0], tiles: HashMap::new(), chunks: Chunks::new(size, brick_capacity) }
  }

  /// Where a chunk is on the torus, in chunks.
  fn torus(tile: [i32; 2]) -> [usize; 2] { tile.map(|t| t.rem_euclid(SIDE as i32) as usize) }

  /// Centers the window on a chunk.
  pub fn set_center(&mut self, center: [i32; 2]) {
    self.window = center.map(|c| c - SIDE as i32 / 2);
    let shift = Self::torus(self.window);
    self.chunks.set_shift(shift.map(|s| s * (TILE_BLOCKS / CHUNK)));
  }

  /// The world position of the window's minimum corner. Camera and ray
  /// marcher positions are relative to this.
  pub fn window_origin(&self) -> [f64; 2] { self.window.map(|w| f64::from(w) * TILE_BLOCKS as f64) }

  /// How far the torus is rotated to get the window, in blocks.
  pub fn shift(&self) -> [i32; 2] { Self::torus(self.window).map(|s| (s * TILE_BLOCKS) as i32) }

  /// A world block, relative to the window.
  pub fn to_window(&self, world: [i32; 2]) -> [i32; 2] {
    [0, 1].map(|a| world[a] - self.window[a] * TILE_BLOCKS as i32)
  }

  /// Whether a chunk is generated.
  pub fn contains(&self, tile: [i32; 2]) -> bool { self.tiles.contains_key(&tile) }

  /// The noise at a world block, if its chunk is generated.
  pub fn raw_at(&self, world: [i32; 2]) -> Option<RawNoise> {
    let tile = world.map(|w| w.div_euclid(TILE_BLOCKS as i32));
    let [i, j] = world.map(|w| w.rem_euclid(TILE_BLOCKS as i32) as usize);
    self.tiles.get(&tile).map(|t| unpack(t.raw[j * TILE_BLOCKS + i]))
  }

  /// The upload for a new grid, which is all it needs, as the columns are
  /// uploaded as they are generated.
  pub fn reset_upload(&self) -> Upload { Upload::Reset { size: self.size, dims: self.chunks.dims } }

  /// The color of every column of a tile, in a layer. `raw` is `TILE_BLOCKS *
  /// TILE_BLOCKS` samples. This is the slow part of `set_tile`, and doesn't
  /// need the grid, so the workers do it.
  pub fn tile_colors(
    raw: &[RawNoise],
    biomes: &[u32],
    splines: &Splines,
    layer: Layer,
  ) -> Vec<Color32> {
    raw
      .iter()
      .zip(biomes)
      .map(|(&sample, &biome)| Self::color(sample, biome, splines, layer))
      .collect()
  }

  /// Stores a freshly generated chunk, and returns the uploads for it. `raw`
  /// is the noise of each column of the tile.
  pub fn set_tile(
    &mut self,
    tile: [i32; 2],
    raw: &[RawNoise],
    biomes: &[u32],
    colors: Vec<Color32>,
    blocks: TileBlocks,
  ) -> Vec<Upload> {
    let data =
      TileData { raw: raw.iter().map(|&r| pack(r)).collect(), biomes: biomes.to_vec(), colors };
    let upload = self.color_upload(tile, &data);
    self.tiles.insert(tile, data);

    let [tx, tz] = Self::torus(tile);
    let per_tile = TILE_BLOCKS / CHUNK;
    let mut updates = vec![];
    for (i, column) in blocks.columns.into_iter().enumerate() {
      let (cx, cz) = (i % per_tile, i / per_tile);
      self.chunks.set_column(tx * per_tile + cx, tz * per_tile + cz, column, &mut updates);
    }
    vec![upload, Upload::Chunks(updates)]
  }

  fn color_upload(&self, tile: [i32; 2], data: &TileData) -> Upload {
    let [tx, tz] = Self::torus(tile);
    Upload::Tile {
      x:      tx * TILE_BLOCKS,
      y:      tz * TILE_BLOCKS,
      size:   TILE_BLOCKS,
      colors: colors_bytes(&data.colors),
    }
  }

  /// Recolors every loaded column, after the splines or layer change. This
  /// returns the uploads for them.
  pub fn recolor_all(&mut self, splines: &Splines, layer: Layer) -> Vec<Upload> {
    let threads = std::thread::available_parallelism().map_or(1, |t| t.get());
    let mut tiles: Vec<&mut TileData> = self.tiles.values_mut().collect();
    let per_thread = tiles.len().div_ceil(threads).max(1);

    std::thread::scope(|scope| {
      for group in tiles.chunks_mut(per_thread) {
        scope.spawn(move || {
          for data in group {
            for (i, &raw) in data.raw.iter().enumerate() {
              data.colors[i] = Self::color(unpack(raw), data.biomes[i], splines, layer);
            }
          }
        });
      }
    });

    self.tiles.iter().map(|(&tile, data)| self.color_upload(tile, data)).collect()
  }

  /// The color of one column in a layer.
  fn color(raw: RawNoise, biome: u32, splines: &Splines, layer: Layer) -> Color32 {
    let parts = splines.parts(raw);
    let (lo, hi) = layer.range();
    match layer {
      // These are colored by block, so this is never drawn.
      Layer::Blocks => Color32::BLACK,
      Layer::Height => height_color(parts.height, lo, hi),
      Layer::Biomes => Color32::from_rgb((biome >> 16) as u8, (biome >> 8) as u8, biome as u8),
      _ => gradient(&VIRIDIS, ((layer.value(raw, parts) - lo) / (hi - lo)) as f32),
    }
  }

  /// Clears every generated chunk that is further than `radius` chunks from
  /// `center`, and returns the chunks that changed.
  pub fn evict_outside(&mut self, center: [i32; 2], radius: i32) -> Vec<ChunkUpdate> {
    let gone: Vec<[i32; 2]> =
      self.tiles.keys().copied().filter(|&t| !in_range(center, radius, t)).collect();

    let per_tile = TILE_BLOCKS / CHUNK;
    let mut updates = vec![];
    for tile in gone {
      self.tiles.remove(&tile);
      let [tx, tz] = Self::torus(tile);
      for cz in 0..per_tile {
        for cx in 0..per_tile {
          self.chunks.clear_column(tx * per_tile + cx, tz * per_tile + cz, &mut updates);
        }
      }
    }
    updates
  }

  /// Casts a ray through the blocks, and returns the world position of the
  /// column of the first one it hits. The ray is in window space: one unit is
  /// one block, and the window covers `0..size` on x and z. This is the same
  /// walk the shader does.
  pub fn raycast(&self, origin: [f32; 3], dir: [f32; 3]) -> Option<[i32; 2]> {
    let [x, _, z] = self.chunks.raycast(origin, dir)?;
    let w = self.window_origin();
    Some([w[0] as i32 + x as i32, w[1] as i32 + z as i32])
  }
}

const VIRIDIS: [(f32, [u8; 3]); 5] = [
  (0.00, [68, 1, 84]),
  (0.25, [59, 82, 139]),
  (0.50, [33, 145, 140]),
  (0.75, [94, 201, 98]),
  (1.00, [253, 231, 37]),
];

fn height_color(h: f64, lo: f64, hi: f64) -> Color32 {
  if h < SEA_LEVEL {
    let t = ((SEA_LEVEL - h) / (SEA_LEVEL - lo).max(1.0)) as f32;
    gradient(&[(0.0, [70, 140, 210]), (1.0, [15, 40, 110])], t)
  } else {
    let t = ((h - SEA_LEVEL) / (hi - SEA_LEVEL).max(1.0)) as f32;
    gradient(
      &[
        (0.00, [194, 178, 128]),
        (0.04, [90, 150, 70]),
        (0.30, [50, 105, 50]),
        (0.55, [120, 100, 80]),
        (0.80, [150, 150, 150]),
        (1.00, [250, 250, 250]),
      ],
      t,
    )
  }
}

fn gradient(stops: &[(f32, [u8; 3])], t: f32) -> Color32 {
  let t = t.clamp(0.0, 1.0);
  for w in stops.windows(2) {
    let (t0, c0) = w[0];
    let (t1, c1) = w[1];
    if t <= t1 {
      let f = (t - t0) / (t1 - t0);
      let l = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * f) as u8;
      return Color32::from_rgb(l(c0[0], c1[0]), l(c0[1], c1[1]), l(c0[2], c1[2]));
    }
  }
  let [r, g, b] = stops.last().unwrap().1;
  Color32::from_rgb(r, g, b)
}

pub fn dot(a: [f32; 3], b: [f32; 3]) -> f32 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }

pub fn normalize(a: [f32; 3]) -> [f32; 3] {
  let l = dot(a, a).sqrt();
  [a[0] / l, a[1] / l, a[2] / l]
}
