use crate::{Random, Rng};

/// A determinstic, randomly spread out grid of points.
pub struct PointGrid;

impl Default for PointGrid {
  fn default() -> Self { Self::new() }
}

impl PointGrid {
  pub fn new() -> Self { Self }

  pub fn closest_point(&self, seed: u64, x: f64, y: f64) -> (f64, f64) {
    let candidates = [
      self.point_in_square(seed, x as i32, y as i32),
      self.point_in_square(seed, x as i32 + 1, y as i32),
      self.point_in_square(seed, x as i32, y as i32 + 1),
      self.point_in_square(seed, x as i32 + 1, y as i32 + 1),
    ];

    candidates
      .into_iter()
      .min_by(|(c_x_a, c_y_a), (c_x_b, c_y_b)| {
        let dist_a = (x - c_x_a).powi(2) + (y - c_y_a).powi(2);
        let dist_b = (x - c_x_b).powi(2) + (y - c_y_b).powi(2);

        // There are no NaNs, because I said so.
        dist_a.partial_cmp(&dist_b).unwrap()
      })
      .unwrap()
  }

  pub fn points_in_area(
    &self,
    seed: u64,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
  ) -> impl Iterator<Item = (f64, f64)> + '_ {
    (min_x.floor() as i32..=max_x.ceil() as i32).flat_map(move |x| {
      (min_y.floor() as i32..=max_y.ceil() as i32).filter_map(move |y| {
        let p = self.point_in_square(seed, x, y);
        if p.0 >= min_x && p.0 <= max_x && p.1 >= min_y && p.1 <= max_y {
          Some(p)
        } else {
          None
        }
      })
    })
  }

  /// Returns the location of a point in the given square of the grid.
  fn point_in_square(&self, seed: u64, x: i32, y: i32) -> (f64, f64) {
    let seed = seed.wrapping_add((x as u64) << 32).wrapping_add(y as u64);

    let number = Rng::new(seed).next();

    let p_x = number as u32;
    let p_y = (number >> 32) as u32;

    (
      f64::from(p_x) / f64::from(u32::MAX) + f64::from(x),
      f64::from(p_y) / f64::from(u32::MAX) + f64::from(y),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn points_in_area() {
    const SEED: u64 = 1234;
    let grid = PointGrid::new();

    let points: Vec<_> = grid.points_in_area(SEED, 0.0, 0.0, 1.0, 1.0).collect();
    assert_eq!(points.len(), 1);

    let points: Vec<_> = grid.points_in_area(SEED, 0.0, 0.0, 1.5, 1.5).collect();
    assert!((1..=3).contains(&points.len()));

    for p in points {
      assert!(p.0 >= 0.0 && p.0 <= 1.5);
      assert!(p.1 >= 0.0 && p.1 <= 1.5);
    }

    let points: Vec<_> = grid.points_in_area(SEED, 0.0, 0.0, 2.0, 2.0).collect();
    assert_eq!(points.len(), 4);

    let points: Vec<_> = grid.points_in_area(SEED, -1.0, -1.0, 0.0, 0.0).collect();
    assert_eq!(points.len(), 1);

    for p in points {
      assert!(p.0 >= -1.0 && p.0 <= 0.0);
      assert!(p.1 >= -1.0 && p.1 <= 0.0);
    }
  }

  #[test]
  fn negative_works() {
    const SEED: u64 = 1234;
    let grid = PointGrid::new();

    let points: Vec<_> = grid.points_in_area(SEED, -1.0, -1.0, 0.0, 0.0).collect();
    assert_eq!(points.len(), 1);

    // FIXME: It'd be nice to test this without relying on the seed. Ah well.
    assert_eq!(points[0], (-0.3343546994343295, -0.5086268462493613));

    let points_2: Vec<_> = grid.points_in_area(SEED, -0.4, -1.0, -0.3, 0.0).collect();
    assert_eq!(points_2.len(), 1);

    assert_eq!(points_2[0], points[0]);
  }
}
