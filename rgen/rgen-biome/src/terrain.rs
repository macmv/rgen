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

  /// Selects a sparse subset of suitable coastal river mouths.
  delta_rarity_map: OctavedNoise<OpenSimplexNoise, 2>,

  /// Breaks selected delta flats into raised islands and shallow channels.
  delta_islands_map: OctavedNoise<OpenSimplexNoise, 4>,
  /// Adds small-scale height variation to the floor of the main river channel.
  river_bed_map: OctavedNoise<OpenSimplexNoise, 2>,
  /// Selects longer river stretches where the channel walls become rounded.
  river_rounding_map: OctavedNoise<OpenSimplexNoise, 2>,
}

impl TerrainNoise {
  pub fn new(seed: u64) -> Self {
    TerrainNoise {
      continentalness_map: OctavedNoise::new(seed, 1.0 / 1024.0),
      peaks_valleys_map:   OctavedNoise::new(seed, 1.0 / 256.0),
      erosion_map:         OctavedNoise::new(seed, 1.0 / 2048.0),
      delta_rarity_map:    OctavedNoise::new(seed.wrapping_add(0xD317_A001), 1.0 / 512.0),
      // Longer wavelengths create broader, smoother delta islands.
      delta_islands_map:   OctavedNoise::new(seed.wrapping_add(0xD317_A002), 1.0 / 5.0),
      river_bed_map:       OctavedNoise::new(seed.wrapping_add(0xD317_A003), 1.0 / 16.0),
      river_rounding_map:  OctavedNoise::new(seed.wrapping_add(0xD317_A004), 1.0 / 64.0),
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
    let mut noise = RawNoise {
      continentalness: self.sample_continentalness(x, z),
      peaks_valleys:   self.sample_peaks_valleys(x, z),
      erosion:         self.sample_erosion(x, z),
      delta_rarity:    0.0,
      delta_islands:   0.0,
      river_bed:       0.0,
      river_rounding:  0.0,
    };

    if river_distance(noise) <= RIVER_WATERLINE_RADIUS {
      noise.river_bed = self.river_bed_map.generate(x, z);
      noise.river_rounding = self.river_rounding_map.generate(x, z);
    }

    if is_delta_mouth_candidate(noise) {
      noise.delta_rarity = self.delta_rarity_map.generate(x, z) * 0.5 + 0.5;
      if noise.delta_rarity >= DELTA_RARITY_THRESHOLD {
        noise.delta_islands = self.delta_islands_map.generate(x, z) * 0.5 + 0.5;
      }
    }

    noise
  }
}

/// Noise values, each in `0.0..=1.0`.
#[derive(Debug, Clone, Copy)]
pub struct RawNoise {
  pub continentalness: f64,
  pub peaks_valleys:   f64,
  pub erosion:         f64,
  pub delta_rarity:    f64,
  pub delta_islands:   f64,
  /// Small-scale river floor variation in `-1.0..=1.0`.
  pub river_bed:       f64,
  /// Long-wave variation that selects rounded river sections.
  pub river_rounding:  f64,
}

// Lower gate and wider coastal/valley bands make suitable river mouths more
// likely to become deltas, while the seeded rarity field keeps occurrences
// spatially clustered rather than turning every coast into one.
const DELTA_RARITY_THRESHOLD: f64 = 0.52;
const DELTA_GEOGRAPHY_THRESHOLD: f64 = 0.60;
const DELTA_TERRAIN_BLEND_START: f64 = 0.35;

/// Potential delta mouths are coastal, valley-shaped, and highly eroded
/// (flatter) terrain. A separate seeded noise gate makes only some candidates
/// become deltas.
pub fn is_delta_mouth_candidate(noise: RawNoise) -> bool {
  (0.22..0.54).contains(&noise.continentalness)
    && (noise.peaks_valleys - 0.5).abs() < 0.22
    && noise.erosion >= 0.45
}

fn smoothstep(edge0: f64, edge1: f64, value: f64) -> f64 {
  let t = ((value - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
  t * t * (3.0 - 2.0 * t)
}

fn band_strength(value: f64, min: f64, max: f64, fade: f64) -> f64 {
  smoothstep(min, min + fade, value) * (1.0 - smoothstep(max - fade, max, value))
}

pub fn delta_strength(noise: RawNoise) -> f64 {
  if noise.delta_rarity < DELTA_RARITY_THRESHOLD {
    return 0.0;
  }

  let coastal_band = band_strength(noise.continentalness, 0.22, 0.54, 0.05);
  let valley = 1.0 - smoothstep(0.09, 0.22, (noise.peaks_valleys - 0.5).abs());
  let flatness = smoothstep(0.45, 0.70, noise.erosion);
  let rarity = smoothstep(DELTA_RARITY_THRESHOLD, 0.78, noise.delta_rarity);

  coastal_band * valley * flatness * rarity
}

pub fn is_delta(noise: RawNoise) -> bool {
  delta_strength(noise) >= DELTA_GEOGRAPHY_THRESHOLD
}

/// Continuous river-channel and bank mask shared by terrain and geography.
/// The inner half is the water channel; the outer half forms dry banks.
pub fn river_strength(noise: RawNoise) -> f64 {
  let distance = river_distance(noise);
  const CHANNEL_RADIUS: f64 = 0.0125;
  // Keep the dry River-classified bank roughly 7-8 blocks wide beyond the
  // water channel. The noise gradient varies, so this is an approximate width.
  const BANK_RADIUS: f64 = 0.055;
  let rounding = smoothstep(0.0, 1.0, noise.river_rounding);
  let channel_radius = CHANNEL_RADIUS * (1.0 - rounding);
  let bank_radius = BANK_RADIUS + (RIVER_WATERLINE_RADIUS - BANK_RADIUS) * rounding;
  if distance <= channel_radius {
    return 1.0;
  }
  if distance >= bank_radius {
    return 0.0;
  }

  let t = (distance - channel_radius) / (bank_radius - channel_radius);
  1.0 - t * t * (3.0 - 2.0 * t)
}

// Round the ends of the river corridor instead of cutting both the channel
// and its banks off at the continentalness boundaries. This distance is in
// noise space; the resulting width in world blocks varies with the gradient.
fn river_distance(noise: RawNoise) -> f64 {
  let across = (noise.peaks_valleys - 0.5).abs();
  // Carry the full channel across the beach (0.32..0.37), tapering only
  // offshore where the natural seabed is already below the river floor.
  let beyond_end = (0.26 - noise.continentalness)
    .max(noise.continentalness - 0.60)
    .max(0.0) * 2.0;
  across.hypot(beyond_end)
}

/// Include the flat shelf and rounded inland end. Offshore carving continues
/// into the sea, but keeps Ocean geography instead of extending River offshore.
pub fn is_river(noise: RawNoise) -> bool {
  noise.continentalness >= 0.32 && river_distance(noise) <= RIVER_WATERLINE_RADIUS
}

const RIVER_WATERLINE_RADIUS: f64 = 0.06;
const RIVER_SLOPE_RADIUS: f64 = 0.16;

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
  let terrain_height = height;

  let river = river_strength(noise);
  if river >= 0.5 {
    // Lower the central channel smoothly below sea level, reaching Y=60 at
    // its center so every geographically classified river has a water bed.
    let channel_depth = ((river - 0.5) * 2.0).clamp(0.0, 1.0);
    let wall_rounding = smoothstep(0.0, 1.0, noise.river_rounding);
    let depth_curve = channel_depth.powf(1.0 - wall_rounding * 0.75);
    height = height.min(64.0 - depth_curve * 4.0);
    if river >= 0.75 {
      // Roughen the submerged channel floor by up to about 1.5 blocks while
      // keeping it below the waterline.
      height = (height + noise.river_bed * 1.5).min(63.0);
    }
  } else {
    let river_distance = river_distance(noise);
    if river_distance <= RIVER_WATERLINE_RADIUS {
      // Keep a short, flat shelf at the waterline before the bank rises.
      height = height.min(64.0);
    } else if river_distance < RIVER_SLOPE_RADIUS {
      // Blend the shelf into the natural terrain over a broad, smooth bank.
      let t = (river_distance - RIVER_WATERLINE_RADIUS)
        / (RIVER_SLOPE_RADIUS - RIVER_WATERLINE_RADIUS);
      let slope = smoothstep(0.0, 1.0, t);
      let bank_height = 64.0 + (height - 64.0).max(0.0) * slope;
      height = height.min(bank_height);
    }
  }

  // Start from the river-shaped height so the delta's outer transition is
  // continuous. Build the target from uncarved terrain: at full influence the
  // delta's islands and water pockets replace the river channel completely.
  let delta = delta_strength(noise);
  let blend = smoothstep(DELTA_TERRAIN_BLEND_START, 0.85, delta);
  if blend > 0.0 {
    let basin_height = lerp(terrain_height, 64.0, delta);
    let delta_height = if noise.delta_islands >= 0.68 {
      64.5 + (noise.delta_islands - 0.68) * 2.0
    } else {
      60.5 + noise.delta_islands * (4.0 / 0.68)
    };
    let island_detail = smoothstep(0.25, 0.85, delta);
    let target = lerp(basin_height, delta_height, island_detail);
    height = lerp(height, target, blend);
  }

  HeightParts { c, impact, p, e, height }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn height(noise: RawNoise) -> f64 {
    height_parts(
      &crate::CONTINENTALNESS,
      &crate::EROSION,
      &crate::PEAKS_VALLEYS,
      &crate::HEIGHT_IMPACT,
      noise,
    ).height
  }

  fn sample(peaks_valleys: f64, rarity: f64) -> RawNoise {
    RawNoise {
      continentalness: 0.42,
      peaks_valleys,
      erosion: 0.8,
      delta_rarity: rarity,
      delta_islands: 0.5,
      river_bed: 0.0,
      river_rounding: 0.0,
    }
  }

  #[test]
  fn river_endpoints_and_slopes_are_continuous() {
    for boundary in [0.18, 0.23, 0.26, 0.32, 0.37, 0.60, 0.63, 0.68] {
      for peaks in [0.5, 0.54, 0.58] {
        let mut before = sample(peaks, 0.0);
        before.continentalness = boundary - 0.0000001;
        let mut after = before;
        after.continentalness = boundary + 0.0000001;
        assert!((height(before) - height(after)).abs() < 0.001,
          "height jump at continentalness {boundary}, peaks {peaks}");
      }
    }
  }

  #[test]
  fn river_channel_stays_submerged_across_beach_into_ocean() {
    for step in 0..=120 {
      let continentalness = 0.26 + step as f64 * 0.001;
      for rounding in [0.0, 0.5, 1.0] {
        let mut noise = sample(0.5, 0.0);
        noise.continentalness = continentalness;
        noise.river_rounding = rounding;
        noise.river_bed = 1.0; // Highest possible river-bed bump.
        assert_eq!(river_strength(noise), 1.0);
        assert!(height(noise) <= 61.5, "mouth rises at {continentalness}");
        assert_eq!(is_river(noise), continentalness >= 0.32);
      }
    }
  }

  #[test]
  fn river_end_has_classified_flat_bank_beyond_water() {
    let mut bank = sample(0.5, 0.0);
    bank.continentalness = 0.629;
    assert_eq!(river_strength(bank), 0.0);
    assert!(is_river(bank));
    assert!((height(bank) - 64.0).abs() < 0.0001);
    bank.continentalness = 0.65;
    assert!(!is_river(bank));
    assert!(height(bank) > 64.0);
  }

  #[test]
  fn weak_delta_does_not_change_river_or_bank() {
    for peaks in [0.5, 0.54, 0.58] {
      let weak = sample(peaks, 0.60);
      assert!(delta_strength(weak) > 0.0);
      assert!(!is_delta(weak));
      assert_eq!(height(weak), height(sample(peaks, 0.0)));
    }
  }

  #[test]
  fn delta_blend_start_preserves_channel_and_bank_continuity() {
    // Test the terrain activation boundary, not just the orange biome boundary.
    for peaks in [0.5, 0.54, 0.58, 0.60] {
      let (mut low, mut high) = (DELTA_RARITY_THRESHOLD, 0.78);
      assert!(delta_strength(sample(peaks, high)) > DELTA_TERRAIN_BLEND_START);
      for _ in 0..50 {
        let mid = (low + high) * 0.5;
        if delta_strength(sample(peaks, mid)) > DELTA_TERRAIN_BLEND_START {
          high = mid;
        } else {
          low = mid;
        }
      }
      let before = sample(peaks, low - 0.000001);
      let after = sample(peaks, high + 0.000001);
      assert_eq!(height(before), height(sample(peaks, 0.0)));
      assert!((height(before) - height(after)).abs() < 0.0001,
        "delta activation jumps at peaks {peaks}");
    }
  }

  #[test]
  fn full_delta_keeps_its_independent_island_height() {
    let mut noise = sample(0.5, 0.78);
    for islands in [0.2, 0.75] {
      noise.delta_islands = islands;
      let target = if islands >= 0.68 {
        64.5 + (islands - 0.68) * 2.0
      } else {
        60.5 + islands * (4.0 / 0.68)
      };
      assert!((height(noise) - target).abs() < 0.0001);
    }
  }

  #[test]
  fn delta_boundary_has_no_height_jump() {
    // Locate the classification boundary without assuming a linear noise mask.
    let (mut low, mut high) = (DELTA_RARITY_THRESHOLD, 0.78);
    for _ in 0..50 {
      let mid = (low + high) * 0.5;
      if is_delta(sample(0.5, mid)) { high = mid; } else { low = mid; }
    }
    let outside = sample(0.5, low - 0.000001);
    let inside = sample(0.5, high + 0.000001);
    assert!(!is_delta(outside));
    assert!(is_delta(inside));
    assert!((height(outside) - height(inside)).abs() < 0.0001);
    assert!(height(sample(0.5, 0.78)) > height(sample(0.5, 0.0)));
  }
}
