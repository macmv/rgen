use rgen_placer::noise::{NoiseGenerator, OctavedNoise, OpenSimplexNoise};
use rgen_spline::{Cosine, Spline, SplineStorage};

/// The three raw noise maps that feed into the height function. This is split
/// out of `WorldBiomes` so that tools can sample terrain without building the
/// whole generator.
pub struct TerrainNoise {
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
}

impl TerrainNoise {
  pub fn new(seed: u64) -> Self {
    TerrainNoise {
      continentalness_map: OctavedNoise::new(seed, 1.0 / 1024.0),
      peaks_valleys_map:   OctavedNoise::new(seed, 1.0 / 256.0),
      erosion_map:         OctavedNoise::new(seed, 1.0 / 2048.0),
    }
  }

  pub fn sample_continentalness(&self, x: f64, z: f64) -> f64 {
    (self.continentalness_map.generate(x, z) * 0.5 + 0.5).clamp(0.0, 1.0)
  }

  pub fn sample_peaks_valleys(&self, x: f64, z: f64) -> f64 {
    (self.peaks_valleys_map.generate(x, z) * 0.5 + 0.5).clamp(0.0, 1.0)
  }

  pub fn sample_erosion(&self, x: f64, z: f64) -> f64 {
    (self.erosion_map.generate(x, z) * 0.5 + 0.5).clamp(0.0, 1.0)
  }

  pub fn sample(&self, x: f64, z: f64) -> RawNoise {
    RawNoise {
      continentalness: self.sample_continentalness(x, z),
      peaks_valleys:   self.sample_peaks_valleys(x, z),
      erosion:         self.sample_erosion(x, z),
    }
  }
}

/// Noise values, each in `0.0..=1.0`.
#[derive(Debug, Clone, Copy)]
pub struct RawNoise {
  pub continentalness: f64,
  pub peaks_valleys:   f64,
  pub erosion:         f64,
}

/// Continuous river-channel and bank mask shared by terrain and geography.
/// The inner half is the water channel; the outer half forms dry banks.
pub fn river_strength(noise: RawNoise) -> f64 {
  if !(0.37..0.60).contains(&noise.continentalness) {
    return 0.0;
  }

  let distance = (noise.peaks_valleys - 0.5).abs();
  const CHANNEL_RADIUS: f64 = 0.0125;
  // Keep the dry River-classified bank roughly 7-8 blocks wide beyond the
  // water channel. The noise gradient varies, so this is an approximate width.
  const BANK_RADIUS: f64 = 0.055;
  if distance <= CHANNEL_RADIUS {
    return 1.0;
  }
  if distance >= BANK_RADIUS {
    return 0.0;
  }

  let t = (distance - CHANNEL_RADIUS) / (BANK_RADIUS - CHANNEL_RADIUS);
  1.0 - t * t * (3.0 - 2.0 * t)
}

/// Every intermediate value of the height function.
#[derive(Debug, Clone, Copy)]
pub struct HeightParts {
  /// Continentalness after its spline, in blocks.
  pub c:      f64,
  /// How much peaks and erosion apply, from `0.0` (ocean) to `1.0` (inland).
  pub impact: f64,
  /// Peaks and valleys after its spline and the impact blend.
  pub p:      f64,
  /// Erosion after its spline and the impact blend.
  pub e:      f64,
  pub height: f64,
}

/// Combines the splines and noise into a height. This is the one definition of
/// the height function: `WorldBiomes::sample_height` and the spline editor both
/// go through here.
pub fn height_parts<S: SplineStorage + ?Sized>(
  continentalness: &Spline<S>,
  erosion: &Spline<S>,
  peaks_valleys: &Spline<S>,
  height_impact: &Spline<S>,
  noise: RawNoise,
) -> HeightParts {
  let c = continentalness.sample::<Cosine>(noise.continentalness);
  let impact = height_impact.sample::<Cosine>(c / 128.0);
  let p = peaks_valleys.sample::<Cosine>(noise.peaks_valleys);
  let e = erosion.sample::<Cosine>(noise.erosion);

  fn lerp(a: f64, b: f64, t: f64) -> f64 { a * (1.0 - t) + b * t }
  let e = lerp(0.2, e, impact);
  let p = lerp(8.0, p, impact);

  let mut height = ((((c - 64.0) * 4.0) + 64.0) + p - 64.0) * e + 64.0;

  let river = river_strength(noise);
  if river >= 0.5 {
    // Lower the central channel smoothly below sea level, reaching Y=60 at
    // its center so every geographically classified river has a water bed.
    height = 63.5 - (river - 0.5) * 7.0;
  } else if river > 0.0 {
    // Keep the outer channel margin dry and raise it into a gentle bank.
    height = height.max(64.0 + (0.5 - river) * 4.0);
  }

  HeightParts { c, impact, p, e, height }
}
