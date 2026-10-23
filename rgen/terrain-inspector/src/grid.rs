use std::collections::HashMap;

use rgen_biome::RawNoise;

use crate::{
  chunks::{CHUNK, ChunkUpdate, Chunks},
  real::TileBlocks,
};

pub const SEA_LEVEL: f64 = 64.0;

/// The width of a chunk, in blocks.
pub const TILE_BLOCKS: usize = 128;

const _: () = assert!(TILE_BLOCKS % CHUNK == 0);

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

/// The noise of one generated chunk, with one sample per block column and x
/// changing fastest.
struct TileData {
  /// The noise at each column. This is `[continentalness, peaks_valleys,
  /// erosion]`, which is kept as `f32` to save memory.
  raw: Vec<[f32; 3]>,
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
  /// Sizes the textures for a new grid, and empties the chunks.
  Reset { size: [usize; 2], dims: [usize; 3] },
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

  /// Stores a freshly generated chunk, and returns the uploads for it. `raw`
  /// is the noise of each column of the tile.
  pub fn set_tile(&mut self, tile: [i32; 2], raw: &[RawNoise], blocks: TileBlocks) -> Vec<Upload> {
    self.tiles.insert(tile, TileData { raw: raw.iter().map(|&r| pack(r)).collect() });

    let [tx, tz] = Self::torus(tile);
    let per_tile = TILE_BLOCKS / CHUNK;
    let mut updates = vec![];
    for (i, column) in blocks.columns.into_iter().enumerate() {
      let (cx, cz) = (i % per_tile, i / per_tile);
      self.chunks.set_column(tx * per_tile + cx, tz * per_tile + cz, column, &mut updates);
    }
    vec![Upload::Chunks(updates)]
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

pub fn dot(a: [f32; 3], b: [f32; 3]) -> f32 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }

pub fn normalize(a: [f32; 3]) -> [f32; 3] {
  let l = dot(a, a).sqrt();
  [a[0] / l, a[1] / l, a[2] / l]
}
