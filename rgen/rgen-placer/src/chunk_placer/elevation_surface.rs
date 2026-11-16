use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use super::util::mix;
use crate::noise::{NoiseGenerator, OpenSimplexNoise, SeededNoise};
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

/// Smooth world-coordinate upper limit, shared across chunk boundaries.
pub struct ElevationCutoff {
  noise: OpenSimplexNoise,
  range: RangeInclusive<i32>,
  scale: f64,
}

impl ElevationCutoff {
  pub fn new(seed: u64, range: RangeInclusive<i32>, scale: f64) -> Self {
    assert!(!range.is_empty() && scale.is_finite() && scale > 0.0);
    Self { noise: OpenSimplexNoise::new(seed), range, scale }
  }

  fn height(&self, x: i32, z: i32) -> i32 {
    let noise = self.noise.generate(x as f64 / self.scale, z as f64 / self.scale);
    let t = (noise * 0.5 + 0.5).clamp(0.0, 1.0);
    (*self.range.start() as f64 + t * (*self.range.end() - *self.range.start()) as f64).round() as i32
  }
}

/// Treat exposed surface columns only within an inclusive elevation range.
pub struct ElevationSurface {
  pub elevation: RangeInclusive<i32>,
  pub upper_cutoff: Option<ElevationCutoff>,
  pub replace: BlockFilter,
  pub surface: BlockState,
  pub subsurface: BlockState,
  /// Number of subsurface blocks, excluding the surface block.
  pub depth: RangeInclusive<i32>,
}

impl ChunkPlacer for ElevationSurface {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    let seed = rng.next();
    let min = chunk_pos.min_block_pos();
    for x in 0..16 {
      for z in 0..16 {
        let column = ChunkRelPos::new(x, 0, z);
        let Some(y) = chunk.chunk.surfaces(column).iter().copied().max().map(i32::from)
        else { continue; };
        if !self.elevation.contains(&y) || !(1..255).contains(&y) {
          continue;
        }
        let surface = column.with_y(y);
        if !chunk.is_active(surface)
          || !self.replace.contains(chunk.get(surface))
          || chunk.get(surface.with_y(y + 1)) != block![air]
        {
          continue;
        }
        let world_x = min.x + i32::from(x);
        let world_z = min.z + i32::from(z);
        if let Some(cutoff) = &self.upper_cutoff {
          if y > cutoff.height(world_x, world_z) {
            continue;
          }
        }
        let mut rng = Rng::new(mix(seed, world_x, world_z));
        let depth = rng.range(self.depth.clone());
        chunk.set(surface, self.surface);
        for offset in 1..=depth.min(y - 1) {
          let pos = surface.with_y(y - offset);
          // Preserve cavities and anything outside the natural substrate.
          if !self.replace.contains(chunk.get(pos)) {
            break;
          }
          chunk.set(pos, self.subsurface);
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::ground_tests::{registry, terrain};

  #[test]
  fn noisy_cutoff_uses_world_coordinates_and_keeps_lower_limit() {
    let info = registry();
    let cutoff = ElevationCutoff::new(42, 69..=75, 96.0);
    let mut min = 75;
    let mut max = 69;
    for x in (-1024..1024).step_by(16) {
      for z in (-1024..1024).step_by(16) {
        let height = cutoff.height(x, z);
        assert!((69..=75).contains(&height));
        min = min.min(height);
        max = max.max(height);
      }
    }
    assert!(max > min, "cutoff must vary spatially");
    let placer = ElevationSurface {
      elevation: 63..=75,
      upper_cutoff: Some(cutoff),
      replace: [block![grass], block![dirt], block![stone]].into(),
      surface: block![sand],
      subsurface: block![clay],
      depth: 2..=3,
    };
    for cx in [-1, 0, 1] {
      for y in [62, 63, 69, 70, 71, 72, 73, 74, 75, 76] {
        let mut terrain = terrain(&info, y);
        let mut chunk = BiomeCachedChunk::new(&info, &mut terrain);
        placer.place(&mut chunk, &mut Rng::new(42), ChunkPos::new(cx, -1));
        for x in 0..16 {
          for z in 0..16 {
            let cap = placer.upper_cutoff.as_ref().unwrap().height(cx * 16 + i32::from(x), -16 + i32::from(z));
            let expected = if y >= 63 && y <= cap { block![sand] } else { block![grass] };
            assert_eq!(chunk.get(ChunkRelPos::new(x, y, z)), expected);
          }
        }
      }
    }
  }

  #[test]
  fn elevation_limits_depth_and_repeatability() {
    let info = registry();
    let placer = ElevationSurface {
      elevation: 64..=80,
      upper_cutoff: None,
      replace: [block![grass], block![dirt], block![stone]].into(),
      surface: block![sand],
      subsurface: block![clay],
      depth: 8..=12,
    };
    for height in [63, 64, 80, 81] {
      let original = terrain(&info, height);
      let mut a = original.clone();
      let mut b = original.clone();
      for chunk in [&mut a, &mut b] {
        placer.place(&mut BiomeCachedChunk::new(&info, chunk), &mut Rng::new(42), ChunkPos::new(-1, 2));
      }
      assert_eq!(a.data(), b.data());
      if height == 63 || height == 81 {
        assert_eq!(a.data(), original.data());
        continue;
      }
      let chunk = BiomeCachedChunk::new(&info, &mut a);
      for x in 0..16 {
        for z in 0..16 {
          let pos = ChunkRelPos::new(x, height, z);
          assert_eq!(chunk.get(pos), block![sand]);
          let depth = (1..=13).take_while(|d| chunk.get(pos.with_y(height - d)) == block![clay]).count();
          assert!((8..=12).contains(&depth));
          assert_eq!(chunk.get(pos.with_y(height + 1)), block![air]);
        }
      }
    }
  }
}
