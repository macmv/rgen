use std::ops::Add;

use crate::{Random, Rng};

/// This is an infinitely expanding voronoi map. It returns a unique id for
/// every region that is retrieved. It should be used to choose which biome to
/// generate at each block.
pub struct VoronoiNoise {
  scale: u32,
  grid:  PointGrid,
}

impl VoronoiNoise {
  pub fn new(seed: u64, scale: u32) -> Self {
    VoronoiNoise { scale, grid: PointGrid::new(seed, 256, scale) }
  }
}

impl VoronoiNoise {
  pub fn generate(&self, x: f64, y: f64) -> u32 {
    let point = Point::new(x as i32 * self.scale as i32, y as i32 * self.scale as i32);

    let p = self.grid.closest_point(point);
    (p.x as u32) ^ ((p.y as u32) << 16)
  }
}

/// This is a randomized point grid. It is built in such a way that the points
/// inside should be scattered in a random-looking fassion. This should be used
/// to spawn trees in the world.
#[derive(Debug)]
struct PointGrid {
  square_size: u32,
  points:      Vec<Vec<(u32, u32)>>,
}

impl PointGrid {
  /// Creates a new random point grid. `size` is the width and height of the
  /// grid of points. `square_size` is the size of each square in the point
  /// grid. So the total size of the grid is `size` * `square_size`.
  pub fn new(seed: u64, size: u32, square_size: u32) -> Self {
    let mut points = vec![vec![(0, 0); size as usize]; size as usize];
    let mut rng = Rng::new(seed);
    for row in points.iter_mut() {
      for p in row.iter_mut() {
        let num = rng.next();
        p.0 = num as u32 % square_size;
        p.1 = (num >> 32) as u32 % square_size;
      }
    }
    Self { square_size, points }
  }

  /// Returns the closest point to the given point.
  pub fn closest_point(&self, p: Point) -> Point { self.neighbors(p, 2)[0] }

  pub fn neighbors(&self, p: Point, radius: i32) -> Vec<Point> {
    let s = self.square_size as i32;
    let mut points = vec![];
    for x in -radius..=radius {
      for y in -radius..=radius {
        points.push(self.get(p + Point::new(s * x, s * y)));
      }
    }
    points.sort_by(|a, b| {
      let dist_a = p.dist(*a);
      let dist_b = p.dist(*b);
      dist_a.partial_cmp(&dist_b).unwrap()
    });
    points
  }

  // Takes two absolute coordinates for a point, and retrieves the point in
  // that square in absolute coordinate form.
  fn get(&self, p: Point) -> Point {
    let (_, lookup) = self.normalize(p);
    let inner = self.points[lookup.y as usize][lookup.x as usize];
    let p = p.pos_div(self.square_size as i32);
    Point::new(
      inner.0 as i32 + p.x * self.square_size as i32,
      inner.1 as i32 + p.y * self.square_size as i32,
    )
  }

  /// Takes a user-passed coordinate, and returns the relative point, along
  /// with the x and y indicies to use to lookup the point.
  ///
  /// Both points will always have positive x and y values.
  fn normalize(&self, p: Point) -> (Point, Point) {
    let rel = p.pos_mod(self.square_size as i32);
    let len = self.points.len() as i32;
    let lookup = p.pos_div(self.square_size as i32).pos_mod(len);
    (rel, lookup)
  }
}

#[derive(Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Self { Point { x, y } }

  pub fn dist(&self, other: Point) -> f64 {
    ((self.x - other.x).pow(2) as f64 + (self.y - other.y).pow(2) as f64).sqrt()
  }

  pub fn pos_mod(&self, rem: i32) -> Point {
    let x = ((self.x % rem) + rem) % rem;
    let y = ((self.y % rem) + rem) % rem;
    Point::new(x, y)
  }

  pub fn pos_div(&self, rem: i32) -> Point {
    // This should work, but causes things to break horribly
    let x = if self.x < 0 { (self.x + 1) / rem - 1 } else { self.x / rem };
    let y = if self.y < 0 { (self.y + 1) / rem - 1 } else { self.y / rem };
    // Point::new(self.x / rem, self.y / rem)
    Point::new(x, y)
  }
}

impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point { Point::new(self.x + other.x, self.y + other.y) }
}
