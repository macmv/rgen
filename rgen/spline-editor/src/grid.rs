use eframe::egui::Color32;
use rgen_biome::{HeightParts, RawNoise};

use crate::Splines;

pub const SEA_LEVEL: f64 = 64.0;

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

  pub fn value(&self, raw: RawNoise, parts: HeightParts) -> f64 {
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
}

/// The sampled terrain. The noise is sampled in the background, and everything
/// else is derived from it whenever the splines or the layer change.
///
/// Each sample is a column of cubes, `step` blocks on each side.
pub struct Grid {
  /// Columns along each side.
  pub n:      usize,
  pub center: [f64; 2],
  pub step:   f64,

  pub raw:   Vec<RawNoise>,
  pub parts: Vec<HeightParts>,

  /// The top of each column, in blocks. This is a multiple of `step`, so that
  /// the columns are made of whole cubes.
  pub height: Vec<f32>,
  /// The color of each column.
  pub colors: Vec<Color32>,
}

impl Grid {
  pub fn new(raw: Vec<RawNoise>, n: usize, center: [f64; 2], step: f64) -> Self {
    Grid { n, center, step, raw, parts: vec![], height: vec![], colors: vec![] }
  }

  pub fn index(&self, i: usize, j: usize) -> usize { i * self.n + j }

  /// The width of the grid, in blocks.
  pub fn extent(&self) -> f32 { self.n as f32 * self.step as f32 }

  /// The tallest column, in blocks.
  pub fn max_height(&self) -> f32 { self.height.iter().copied().fold(0.0, f32::max) }

  /// The world position of a grid vertex.
  pub fn world(&self, i: usize, j: usize) -> [f64; 2] {
    [
      self.center[0] + (i as f64 - self.n as f64 / 2.0) * self.step,
      self.center[1] + (j as f64 - self.n as f64 / 2.0) * self.step,
    ]
  }

  pub fn recompute(&mut self, splines: &Splines, layer: Layer, flatten_water: bool) {
    self.parts = self.raw.iter().map(|&r| splines.parts(r)).collect();

    let step = self.step;
    let snap = |h: f64| ((h / step).floor() * step) as f32;
    let sea = snap(SEA_LEVEL);
    self.height = self
      .parts
      .iter()
      .map(|p| if flatten_water && p.height < SEA_LEVEL { sea } else { snap(p.height) })
      .collect();

    let values: Vec<f64> =
      self.raw.iter().zip(&self.parts).map(|(&r, &p)| layer.value(r, p)).collect();
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).max(1e-9);

    let lo = self.parts.iter().map(|p| p.height).fold(f64::INFINITY, f64::min);
    let hi = self.parts.iter().map(|p| p.height).fold(f64::NEG_INFINITY, f64::max);

    self.colors = (0..self.n * self.n)
      .map(|idx| {
        if layer == Layer::Height {
          height_color(self.parts[idx].height, lo, hi)
        } else {
          gradient(&VIRIDIS, ((values[idx] - min) / range) as f32)
        }
      })
      .collect();
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
