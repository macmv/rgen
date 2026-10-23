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

  let height = ((((c - 64.0) * 4.0) + 64.0) + p - 64.0) * e + 64.0;

  HeightParts { c, impact, p, e, height }
}
