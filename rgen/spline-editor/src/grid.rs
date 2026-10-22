use eframe::egui::Color32;
use rgen_biome::{HeightParts, RawNoise};

use crate::Splines;

/// Number of samples along each side of the terrain grid.
pub const N: usize = 128;

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
pub struct Grid {
  pub center: [f64; 2],
  pub step:   f64,

  pub raw:   Vec<RawNoise>,
  pub parts: Vec<HeightParts>,

  /// The height each vertex is drawn at.
  pub height: Vec<f32>,
  /// Base color of each vertex, with lighting already applied.
  pub colors: Vec<Color32>,
}

impl Grid {
  pub fn new(raw: Vec<RawNoise>, center: [f64; 2], step: f64) -> Self {
    Grid { center, step, raw, parts: vec![], height: vec![], colors: vec![] }
  }

  pub fn index(i: usize, j: usize) -> usize { i * N + j }

  /// The world position of a grid vertex.
  pub fn world(&self, i: usize, j: usize) -> [f64; 2] {
    [
      self.center[0] + (i as f64 - N as f64 / 2.0) * self.step,
      self.center[1] + (j as f64 - N as f64 / 2.0) * self.step,
    ]
  }

  pub fn recompute(&mut self, splines: &Splines, layer: Layer, flatten_water: bool, vscale: f32) {
    self.parts = self.raw.iter().map(|&r| splines.parts(r)).collect();

    self.height = self
      .parts
      .iter()
      .map(
        |p| {
          if flatten_water && p.height < SEA_LEVEL { SEA_LEVEL as f32 } else { p.height as f32 }
        },
      )
      .collect();

    let values: Vec<f64> =
      self.raw.iter().zip(&self.parts).map(|(&r, &p)| layer.value(r, p)).collect();
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).max(1e-9);

    let light = normalize([0.5, 0.8, 0.3]);

    self.colors = (0..N * N)
      .map(|idx| {
        let (i, j) = (idx / N, idx % N);

        let base = if layer == Layer::Height {
          let lo = self.parts.iter().map(|p| p.height).fold(f64::INFINITY, f64::min);
          let hi = self.parts.iter().map(|p| p.height).fold(f64::NEG_INFINITY, f64::max);
          height_color(self.parts[idx].height, lo, hi)
        } else {
          gradient(&VIRIDIS, ((values[idx] - min) / range) as f32)
        };

        // Surface normal from central differences, in world units.
        let h = |i: usize, j: usize| self.height[Grid::index(i.min(N - 1), j.min(N - 1))];
        let dx = h(i + 1, j) - h(i.saturating_sub(1), j);
        let dz = h(i, j + 1) - h(i, j.saturating_sub(1));
        let scale = vscale / (2.0 * self.step as f32);
        let n = normalize([-dx * scale, 1.0, -dz * scale]);
        let shade = 0.35 + 0.65 * dot(n, light).max(0.0);

        let [r, g, b, _] = base.to_array();
        Color32::from_rgb(
          (r as f32 * shade) as u8,
          (g as f32 * shade) as u8,
          (b as f32 * shade) as u8,
        )
      })
      .collect();
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
