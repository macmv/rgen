use eframe::egui::Color32;
use rgen_biome::RawNoise;

use crate::Splines;

pub const SEA_LEVEL: f64 = 64.0;

/// The width of a chunk, in blocks. This is the same size as the regions in
/// `rgen-viewer`.
pub const TILE_BLOCKS: u32 = 128;
/// The terrain is a square of this many chunks on each side.
pub const TILES: usize = 21;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Layer {
  Height,
  RawContinentalness,
  RawErosion,
  RawPeaksValleys,
  Continentalness,
  Erosion,
  PeaksValleys,
  HeightImpact,
  RiverDistance,
}

impl Layer {
  pub const ALL: [Layer; 9] = [
    Layer::Height,
    Layer::RawContinentalness,
    Layer::RawErosion,
    Layer::RawPeaksValleys,
    Layer::Continentalness,
    Layer::Erosion,
    Layer::PeaksValleys,
    Layer::HeightImpact,
    Layer::RiverDistance,
  ];

  pub fn name(&self) -> &'static str {
    match self {
      Layer::Height => "Final height",
      Layer::RawContinentalness => "Continentalness (noise)",
      Layer::RawErosion => "Erosion (noise)",
      Layer::RawPeaksValleys => "Peaks and valleys (noise)",
      Layer::Continentalness => "Continentalness (spline)",
      Layer::Erosion => "Erosion (spline, blended)",
      Layer::PeaksValleys => "Peaks and valleys (spline, blended)",
      Layer::HeightImpact => "Height impact",
      Layer::RiverDistance => "River distance",
    }
  }

  pub fn value(&self, raw: RawNoise, parts: rgen_biome::HeightParts) -> f64 {
    match self {
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

/// The sampled terrain: `TILES` x `TILES` chunks, each of which is generated on
/// its own. Chunks that haven't been generated yet are empty.
///
/// Each sample is a column of cubes, `step` blocks on each side. All the arrays
/// are indexed with `index`, which matches how they're laid out in a texture.
pub struct Grid {
  /// Columns along each side of a chunk.
  pub cols:   usize,
  /// Columns along each side of the whole grid.
  pub n:      usize,
  pub step:   f64,
  /// The world position of the grid's minimum corner.
  pub origin: [f64; 2],

  pub raw:    Vec<RawNoise>,
  /// The top of each column, in blocks. This is a multiple of `step`, so that
  /// the columns are made of whole cubes. Columns with no terrain are -1.
  pub height: Vec<f32>,
  pub colors: Vec<Color32>,

  loaded:     Vec<bool>,
  pub loaded_count: usize,
  max_height: f32,
}

/// Terrain to copy into the GPU's textures.
pub enum Upload {
  /// Replaces both textures.
  Full { n: usize, heights: Vec<f32>, colors: Vec<u8> },
  /// Replaces one square of both textures.
  Tile { x: usize, y: usize, size: usize, heights: Vec<f32>, colors: Vec<u8> },
}

impl Grid {
  /// Makes a grid where nothing has been generated.
  pub fn new(origin: [f64; 2], step: u32) -> Self {
    let cols = (TILE_BLOCKS / step) as usize;
    let n = cols * TILES;
    Grid {
      cols,
      n,
      step: f64::from(step),
      origin,
      raw: vec![RawNoise { continentalness: 0.0, peaks_valleys: 0.0, erosion: 0.0 }; n * n],
      height: vec![-1.0; n * n],
      colors: vec![Color32::BLACK; n * n],
      loaded: vec![false; TILES * TILES],
      loaded_count: 0,
      max_height: 0.0,
    }
  }

  /// The array index of a column. `i` is along x, and `j` is along z.
  pub fn index(&self, i: usize, j: usize) -> usize { j * self.n + i }

  /// The inverse of `index`.
  pub fn coords(&self, idx: usize) -> (usize, usize) { (idx % self.n, idx / self.n) }

  /// The width of the grid, in blocks.
  pub fn extent(&self) -> f32 { self.n as f32 * self.step as f32 }

  /// The tallest column, in blocks.
  pub fn max_height(&self) -> f32 { self.max_height }

  /// The world position of a column's minimum corner.
  pub fn world(&self, i: usize, j: usize) -> [f64; 2] {
    [self.origin[0] + i as f64 * self.step, self.origin[1] + j as f64 * self.step]
  }

  /// The upload for the entire grid.
  pub fn full_upload(&self) -> Upload {
    Upload::Full {
      n:       self.n,
      heights: self.height.clone(),
      colors:  self.colors.iter().flat_map(|c| c.to_array()).collect(),
    }
  }

  /// Stores a freshly generated chunk, and returns the upload for it.
  /// `raw` is `cols * cols` samples, indexed like the grid.
  pub fn set_tile(
    &mut self,
    tx: usize,
    tz: usize,
    raw: &[RawNoise],
    splines: &Splines,
    layer: Layer,
    flatten_water: bool,
  ) -> Upload {
    let cols = self.cols;
    let (mut heights, mut colors) = (Vec::with_capacity(cols * cols), Vec::with_capacity(cols * 4 * cols));

    for tj in 0..cols {
      for ti in 0..cols {
        let idx = self.index(tx * cols + ti, tz * cols + tj);
        let sample = raw[tj * cols + ti];
        let (height, color) = Self::column(self.step, sample, splines, layer, flatten_water);

        self.raw[idx] = sample;
        self.height[idx] = height;
        self.colors[idx] = color;
        self.max_height = self.max_height.max(height);

        heights.push(height);
        colors.extend(color.to_array());
      }
    }

    if !self.loaded[tz * TILES + tx] {
      self.loaded[tz * TILES + tx] = true;
      self.loaded_count += 1;
    }

    Upload::Tile { x: tx * cols, y: tz * cols, size: cols, heights, colors }
  }

  /// Recalculates every loaded column, after the splines or layer change.
  pub fn recompute_all(&mut self, splines: &Splines, layer: Layer, flatten_water: bool) {
    let n = self.n;
    let threads = std::thread::available_parallelism().map_or(1, |t| t.get());
    let rows_per_thread = n.div_ceil(threads);

    let (raw, loaded) = (&self.raw, &self.loaded);
    let step = self.step;
    let cols = self.cols;

    let max_height = std::thread::scope(|scope| {
      let handles: Vec<_> = self
        .height
        .chunks_mut(rows_per_thread * n)
        .zip(self.colors.chunks_mut(rows_per_thread * n))
        .enumerate()
        .map(|(chunk, (heights, colors))| {
          scope.spawn(move || {
            let mut max = 0.0f32;
            for (r, (hrow, crow)) in heights.chunks_mut(n).zip(colors.chunks_mut(n)).enumerate() {
              let j = chunk * rows_per_thread + r;
              for i in 0..n {
                if !loaded[(j / cols) * TILES + i / cols] {
                  continue;
                }
                let (h, c) = Self::column(step, raw[j * n + i], splines, layer, flatten_water);
                hrow[i] = h;
                crow[i] = c;
                max = max.max(h);
              }
            }
            max
          })
        })
        .collect();

      handles.into_iter().map(|h| h.join().unwrap()).fold(0.0, f32::max)
    });

    self.max_height = max_height;
  }

  /// The height and color of one column.
  fn column(
    step: f64,
    raw: RawNoise,
    splines: &Splines,
    layer: Layer,
    flatten_water: bool,
  ) -> (f32, Color32) {
    let parts = splines.parts(raw);

    let snap = |h: f64| ((h / step).floor() * step) as f32;
    let height = if flatten_water && parts.height < SEA_LEVEL {
      snap(SEA_LEVEL)
    } else {
      snap(parts.height)
    };

    let color = if layer == Layer::Height {
      let (lo, hi) = layer.range();
      height_color(parts.height, lo, hi)
    } else {
      let (lo, hi) = layer.range();
      gradient(&VIRIDIS, ((layer.value(raw, parts) - lo) / (hi - lo)) as f32)
    };

    (height, color)
  }

  /// Casts a ray through the columns, and returns the first column it hits.
  /// The ray is in grid space: one unit is one block, and the grid covers
  /// `0..extent` on x and z. This is the same walk the shader does.
  pub fn raycast(&self, origin: [f32; 3], dir: [f32; 3]) -> Option<usize> {
    let step = self.step as f32;
    let extent = self.extent();
    let top = self.max_height();

    // Clip the ray to the box around the grid.
    let (mut t_in, mut t_out) = (0.0f32, f32::INFINITY);
    for (axis, max) in [(0, extent), (1, top), (2, extent)] {
      if dir[axis].abs() < 1e-9 {
        if origin[axis] < 0.0 || origin[axis] > max {
          return None;
        }
        continue;
      }
      let a = (0.0 - origin[axis]) / dir[axis];
      let b = (max - origin[axis]) / dir[axis];
      t_in = t_in.max(a.min(b));
      t_out = t_out.min(a.max(b));
    }
    if t_in >= t_out {
      return None;
    }

    let start = [origin[0] + dir[0] * (t_in + 1e-3), origin[2] + dir[2] * (t_in + 1e-3)];
    let mut cell = [
      ((start[0] / step).floor() as i32).clamp(0, self.n as i32 - 1),
      ((start[1] / step).floor() as i32).clamp(0, self.n as i32 - 1),
    ];
    let d = [dir[0], dir[2]];
    let o = [origin[0], origin[2]];

    let dir_sign = [if d[0] >= 0.0 { 1 } else { -1 }, if d[1] >= 0.0 { 1 } else { -1 }];
    let t_delta = [step / d[0].abs().max(1e-9), step / d[1].abs().max(1e-9)];
    let mut t_max = [0.0; 2];
    for a in 0..2 {
      let boundary = (cell[a] + (dir_sign[a] + 1) / 2) as f32 * step;
      t_max[a] = if d[a].abs() < 1e-9 { f32::INFINITY } else { (boundary - o[a]) / d[a] };
    }

    let mut t_cur = t_in;
    for _ in 0..self.n * 3 {
      let idx = self.index(cell[0] as usize, cell[1] as usize);
      let h = self.height[idx];
      let t_exit = t_max[0].min(t_max[1]);

      let y_in = origin[1] + dir[1] * t_cur;
      let y_out = origin[1] + dir[1] * t_exit;
      if y_in < h || y_out < h {
        return Some(idx);
      }
      if t_exit > t_out {
        return None;
      }

      let a = if t_max[0] < t_max[1] { 0 } else { 1 };
      cell[a] += dir_sign[a];
      t_cur = t_max[a];
      t_max[a] += t_delta[a];
      if cell[a] < 0 || cell[a] >= self.n as i32 {
        return None;
      }
    }
    None
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
