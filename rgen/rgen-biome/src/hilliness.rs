//! Inland geography from local shape, independent of absolute altitude.
//!
//! Sample the pre-cave, pre-decoration density surface on an eight-block
//! lattice. Local slope decides Hills; wider relief can promote rugged Hills
//! to Mountains, but cannot turn a flat plateau into either type by itself.
use std::{cell::RefCell, num::NonZero};

use lru::LruCache;
use rgen_base::Pos;

use crate::{GeographicType, WorldBiomes};

const STEP: i32 = 8;
const HILL_SLOPE: f64 = 0.28;
const MOUNTAIN_SLOPE: f64 = 0.45;
const MOUNTAIN_RELIEF: f64 = 32.0;

#[derive(Clone, Copy)]
struct Shape {
  slope_squared: f64,
  relief: f64,
}

impl Shape {
  fn mix(self, other: Self, t: f64) -> Self {
    Self {
      slope_squared: self.slope_squared * (1.0 - t) + other.slope_squared * t,
      relief: self.relief * (1.0 - t) + other.relief * t,
    }
  }

  fn geography(self) -> GeographicType {
    if self.slope_squared >= MOUNTAIN_SLOPE * MOUNTAIN_SLOPE
      && self.relief >= MOUNTAIN_RELIEF
    {
      GeographicType::Mountains
    } else if self.slope_squared >= HILL_SLOPE * HILL_SLOPE {
      GeographicType::Hills
    } else {
      GeographicType::Standard
    }
  }
}

thread_local! {
  // Worker-local bounded caches: no contention between generator threads.
  // The seed is essential when a worker is reused for another world.
  static SURFACES: RefCell<LruCache<(u64, i32, i32), f64>> =
    RefCell::new(LruCache::new(NonZero::new(8192).unwrap()));
  static SHAPES: RefCell<LruCache<(u64, i32, i32), Shape>> =
    RefCell::new(LruCache::new(NonZero::new(2048).unwrap()));
}

impl WorldBiomes {
  pub(crate) fn inland_hilliness(&self, pos: Pos) -> GeographicType {
    profile_function!();
    // Euclidean division keeps the lattice continuous across negative coords.
    let x = pos.x.div_euclid(STEP) * STEP;
    let z = pos.z.div_euclid(STEP) * STEP;
    let tx = pos.x.rem_euclid(STEP) as f64 / STEP as f64;
    let tz = pos.z.rem_euclid(STEP) as f64 / STEP as f64;
    let north = self.shape_at(x, z).mix(self.shape_at(x + STEP, z), tx);
    let south = self.shape_at(x, z + STEP).mix(self.shape_at(x + STEP, z + STEP), tx);
    // Interpolate measurements, not biome IDs: no square cells or voting that
    // spreads a distant hillside's label over a whole neighborhood.
    north.mix(south, tz).geography()
  }

  fn shape_at(&self, x: i32, z: i32) -> Shape {
    let calculate = || measure_shape(|dx, dz| self.geography_surface(x + dx, z + dz));
    // Inspector edits can change terrain without changing the seed.
    if cfg!(feature = "inspector") { return calculate(); }
    SHAPES.with(|cache| {
      let mut cache = cache.borrow_mut();
      let key = (self.seed, x, z);
      if let Some(shape) = cache.get(&key) { return *shape; }
      let shape = calculate();
      cache.put(key, shape);
      shape
    })
  }

  fn geography_surface(&self, x: i32, z: i32) -> f64 {
    let calculate = || {
      profile_scope!("geography surface cache miss");
      let mut info = self.height_info(Pos::new(x, 0, z));
      // Match generate_base's exclusive upper limit, including its integer
      // truncation. Do not treat submerged ocean/river floors as land relief.
      let top = (info.max_height() as i32 - 1).clamp(0, 255);
      for y in (64..=top).rev() {
        info.move_to(Pos::new(x, y, z));
        if info.underground() { return y as f64; }
      }
      63.0
    };
    if cfg!(feature = "inspector") { return calculate(); }
    SURFACES.with(|cache| {
      let mut cache = cache.borrow_mut();
      let key = (self.seed, x, z);
      if let Some(height) = cache.get(&key) { return *height; }
      let height = calculate();
      cache.put(key, height);
      height
    })
  }
}

fn measure_shape(mut surface: impl FnMut(i32, i32) -> f64) -> Shape {
  let center = surface(0, 0);
  let mut energy = 0.0;
  // One-sided differences retain hilltops and valley bottoms, where a central
  // derivative would cancel opposite slopes. Diagonals prevent axis bias.
  for dz in -1_i32..=1 {
    for dx in -1_i32..=1 {
      if dx == 0 && dz == 0 { continue; }
      let rise = surface(dx * STEP, dz * STEP) - center;
      let distance_squared = ((dx * dx + dz * dz) * STEP * STEP) as f64;
      energy += rise * rise / distance_squared;
    }
  }
  // For a plane this normalization gives its actual squared gradient.
  let slope_squared = energy / 4.0;
  let mut low = center;
  let mut high = center;
  // Broad relief is only relevant for mountain candidates. Flat ground near
  // a mountain must remain Standard regardless of the wider height range.
  if slope_squared >= HILL_SLOPE * HILL_SLOPE {
    for dz in -1..=1 {
      for dx in -1..=1 {
        if dx == 0 && dz == 0 { continue; }
        let height = surface(dx * 32, dz * 32);
        low = low.min(height);
        high = high.max(height);
      }
    }
  }
  Shape { slope_squared, relief: high - low }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn altitude_does_not_change_geography() {
    for altitude in [70.0, 150.0, 220.0] {
      assert_eq!(measure_shape(|_, _| altitude).geography(), GeographicType::Standard);
      assert_eq!(measure_shape(|x, _| altitude + x as f64 * 0.35).geography(), GeographicType::Hills);
      assert_eq!(measure_shape(|x, _| altitude + x as f64 * 0.75).geography(), GeographicType::Mountains);
    }
  }

  #[test]
  fn flat_pocket_is_not_promoted_by_distant_cliff() {
    let shape = measure_shape(|x, _| if x > 16 { 130.0 } else { 70.0 });
    assert_eq!(shape.geography(), GeographicType::Standard);
  }

  #[test]
  fn crests_and_diagonal_slopes_are_detected() {
    assert_eq!(measure_shape(|x, _| 90.0 - (x as f64).abs() * 0.35).geography(), GeographicType::Hills);
    let straight = measure_shape(|x, _| 90.0 + x as f64 * 0.35);
    let diagonal = measure_shape(|x, z| 90.0 + (x + z) as f64 * 0.35 / 2.0_f64.sqrt());
    assert!((straight.slope_squared - diagonal.slope_squared).abs() < 1e-10);
  }
}
