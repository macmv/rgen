use crate::{Random, Rng};

/// A determinstic, randomly spread out grid of points.
pub struct PointGrid {
  seed: u64,
}

impl PointGrid {
  pub fn new(seed: u64) -> Self { Self { seed } }

  pub fn closest_point(&self, x: f64, y: f64) -> (f64, f64) {
    let candidates = [
      self.point_in_square(x as i32, y as i32),
      self.point_in_square(x as i32 + 1, y as i32),
      self.point_in_square(x as i32, y as i32 + 1),
      self.point_in_square(x as i32 + 1, y as i32 + 1),
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

  pub fn points_in_area<'a>(
    &'a self,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
  ) -> impl Iterator<Item = (f64, f64)> + 'a {
    let min_x = x;
    let min_y = y;
    let max_x = x + width;
    let max_y = y + height;

    (min_x as i32..=max_x as i32).flat_map(move |x| {
      (min_y as i32..=max_y as i32).filter_map(move |y| {
        let p = self.point_in_square(x, y);
        if p.0 >= min_x && p.0 <= max_x && p.1 >= min_y && p.1 <= max_y {
          Some(p)
        } else {
          None
        }
      })
    })
  }

  /// Returns the location of a point in the given square of the grid.
  fn point_in_square(&self, x: i32, y: i32) -> (f64, f64) {
    let seed = self.seed.wrapping_add((x as u64) << 32).wrapping_add(y as u64);

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
    let grid = PointGrid::new(0);

    let points: Vec<_> = grid.points_in_area(0.0, 0.0, 1.0, 1.0).collect();
    assert_eq!(points.len(), 1);

    let points: Vec<_> = grid.points_in_area(0.0, 0.0, 1.5, 1.5).collect();
    assert!((1..=3).contains(&points.len()));

    for p in points {
      assert!(p.0 >= 0.0 && p.0 <= 1.5);
      assert!(p.1 >= 0.0 && p.1 <= 1.5);
    }
  }
}
