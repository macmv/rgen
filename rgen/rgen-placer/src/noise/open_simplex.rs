use super::{NoiseGenerator, NoiseGenerator3D, SeededNoise};

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

#[derive(Clone)]
pub struct OpenSimplexNoise {
  perm: PermutationTable,
}

impl SeededNoise for OpenSimplexNoise {
  fn new(seed: u64) -> Self { OpenSimplexNoise { perm: PermutationTable::init(seed) } }
}

impl NoiseGenerator for OpenSimplexNoise {
  type Output = f64;

  fn generate(&self, x: f64, y: f64) -> f64 { noise_2(Vector2::new(x, y), &self.perm) }
}

impl NoiseGenerator3D for OpenSimplexNoise {
  fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64 {
    noise_3(Vector3::new(x, y, z), &self.perm)
  }
}

impl PermutationTable {
  fn init(mut seed: u64) -> PermutationTable {
    let mut perm = [0_u8; 256];

    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);

    let mut source = [0_u8; 256];

    for i in 0..=255 {
      source[i as usize] = i;
    }

    for i in (0..256_u64).rev() {
      seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
      let r = (seed + 31) % i.wrapping_add(1);
      perm[i as usize] = source[r as usize];
      source[r as usize] = source[i as usize];
    }

    PermutationTable { values: perm }
  }
}

#[derive(Clone)]
struct PermutationTable {
  values: [u8; 256],
}

impl PermutationTable {
  fn hash(&self, to_hash: &[isize]) -> usize {
    let index = to_hash
      .iter()
      .map(|&a| (a & 0xff) as usize)
      .reduce(|a, b| self.values[a] as usize ^ b)
      .unwrap();
    self.values[index] as usize
  }
}

// See the `noise` crate: https://crates.io/crates/noise
fn noise_2(point: Vector2, perm: &PermutationTable) -> f64 {
  const STRETCH_CONSTANT: f64 = -0.211_324_865_405_187; //(1/sqrt(2+1)-1)/2;
  const SQUISH_CONSTANT: f64 = 0.366_025_403_784_439; //(sqrt(2+1)-1)/2;
  const NORM_CONSTANT: f64 = 1.0 / 14.0;

  fn surflet(index: usize, point: Vector2) -> f64 {
    let t = 2.0 - point.magnitude_squared();

    if t > 0.0 {
      let gradient = grad2(index);
      t.powi(4) * point.dot(gradient)
    } else {
      0.0
    }
  }

  // Place input coordinates onto grid.
  let stretch_offset = point.sum() * STRETCH_CONSTANT;
  let stretched = Vector2::new(point.x + stretch_offset, point.y + stretch_offset);

  // Floor to get grid coordinates of rhombus (stretched square) cell origin.
  let stretched_floor = stretched.floor();

  // Skew out to get actual coordinates of rhombus origin. We'll need these later.
  let squish_offset = stretched_floor.sum() * SQUISH_CONSTANT;
  let origin = Vector2::new(stretched_floor.x + squish_offset, stretched_floor.y + squish_offset);

  // Compute grid coordinates relative to rhombus origin.
  let rel_coords = stretched - stretched_floor;

  // Sum those together to get a value that determines which region we're in.
  let region_sum = rel_coords.sum();

  // Positions relative to origin point (0, 0).
  let rel_pos = point - origin;

  macro_rules! contribute (
    ($x:expr, $y:expr) => {
      {
        let offset = Vector2::new($x, $y);
        let vertex = stretched_floor + offset;
        let index = perm.hash(&[vertex.x as isize, vertex.y as isize]);
        let dpos = rel_pos - (Vector2::broadcast(SQUISH_CONSTANT) * offset.sum()) - offset;

        surflet(index, dpos)
      }
    }
  );

  let mut value = 0.0;

  // (0, 0) --- (1, 0)
  // |   A     /     |
  // |       /       |
  // |     /     B   |
  // (0, 1) --- (1, 1)

  // Contribution (1, 0)
  value += contribute!(1.0, 0.0);

  // Contribution (0, 1)
  value += contribute!(0.0, 1.0);

  // See the graph for an intuitive explanation; the sum of `x` and `y` is
  // only greater than `1` if we're on Region B.
  if region_sum > 1.0 {
    // Contribution (1, 1)
    value += contribute!(1.0, 1.0);
  } else {
    // Contribution (1, 1)
    value += contribute!(0.0, 0.0);
  }

  value * NORM_CONSTANT
}

fn noise_3(point: Vector3, perm: &PermutationTable) -> f64 {
  const STRETCH_CONSTANT: f64 = -1.0 / 6.0; //(1/sqrt(3+1)-1)/3;
  const SQUISH_CONSTANT: f64 = 1.0 / 3.0; //(sqrt(3+1)-1)/3;
  const NORM_CONSTANT: f64 = 1.0 / 14.0;

  fn surflet(index: usize, point: Vector3) -> f64 {
    let t = 2.0 - point.magnitude_squared();

    if t > 0.0 {
      let gradient = grad3(index);
      t.powi(4) * point.dot(gradient)
    } else {
      0.0
    }
  }

  // Place input coordinates on simplectic honeycomb.
  let stretch_offset = point.sum() * STRETCH_CONSTANT;
  let stretched =
    Vector3::new(point.x + stretch_offset, point.y + stretch_offset, point.z + stretch_offset);

  // Floor to get simplectic honeycomb coordinates of rhombohedron
  // (stretched cube) super-cell origin.
  let stretched_floor = stretched.floor();

  // Skew out to get actual coordinates of rhombohedron origin. We'll need
  // these later.
  let squish_offset = stretched_floor.sum() * SQUISH_CONSTANT;
  let origin = Vector3::new(
    stretched_floor.x + squish_offset,
    stretched_floor.y + squish_offset,
    stretched_floor.z + squish_offset,
  );

  // Compute simplectic honeycomb coordinates relative to rhombohedral origin.
  let rel_coords = stretched - stretched_floor;

  // Sum those together to get a value that determines which region we're in.
  let region_sum = rel_coords.sum();

  // Positions relative to origin point.
  let rel_pos = point - origin;

  macro_rules! contribute (
    ($x:literal, $y:literal, $z:literal) => {
      {
        let offset = Vector3::new($x, $y, $z);
        let vertex = stretched_floor + offset;
        let index = perm.hash(&[vertex.x as isize, vertex.y as isize, vertex.z as isize]);
        let dpos = rel_pos - (Vector3::broadcast(SQUISH_CONSTANT) * offset.sum()) - offset;

        surflet(index, dpos)
      }
    }
  );

  let mut value = 0.0;

  if region_sum <= 1.0 {
    // We're inside the tetrahedron (3-Simplex) at (0, 0, 0)

    // Contribution at (0, 0, 0)
    value += contribute!(0.0, 0.0, 0.0);

    // Contribution at (1, 0, 0)
    value += contribute!(1.0, 0.0, 0.0);

    // Contribution at (0, 1, 0)
    value += contribute!(0.0, 1.0, 0.0);

    // Contribution at (0, 0, 1)
    value += contribute!(0.0, 0.0, 1.0);
  } else if region_sum >= 2.0 {
    // We're inside the tetrahedron (3-Simplex) at (1, 1, 1)

    // Contribution at (1, 1, 0)
    value += contribute!(1.0, 1.0, 0.0);

    // Contribution at (1, 0, 1)
    value += contribute!(1.0, 0.0, 1.0);

    // Contribution at (0, 1, 1)
    value += contribute!(0.0, 1.0, 1.0);

    // Contribution at (1, 1, 1)
    value += contribute!(1.0, 1.0, 1.0);
  } else {
    // We're inside the octahedron (Rectified 3-Simplex) inbetween.

    // Contribution at (1, 0, 0)
    value += contribute!(1.0, 0.0, 0.0);

    // Contribution at (0, 1, 0)
    value += contribute!(0.0, 1.0, 0.0);

    // Contribution at (0, 0, 1)
    value += contribute!(0.0, 0.0, 1.0);

    // Contribution at (1, 1, 0)
    value += contribute!(1.0, 1.0, 0.0);

    // Contribution at (1, 0, 1)
    value += contribute!(1.0, 0.0, 1.0);

    // Contribution at (0, 1, 1)
    value += contribute!(0.0, 1.0, 1.0);
  }

  value * NORM_CONSTANT
}

fn grad2(index: usize) -> Vector2 {
  // Vectors are combinations of -1, 0, and 1
  // Precompute the normalized element
  const DIAG: f64 = core::f64::consts::FRAC_1_SQRT_2;

  match index % 8 {
    0 => Vector2::new(1.0, 0.0),
    1 => Vector2::new(-1.0, 0.0),
    2 => Vector2::new(0.0, 1.0),
    3 => Vector2::new(0.0, -1.0),
    4 => Vector2::new(DIAG, DIAG),
    5 => Vector2::new(-DIAG, DIAG),
    6 => Vector2::new(DIAG, -DIAG),
    7 => Vector2::new(-DIAG, -DIAG),
    _ => panic!("Attempt to access gradient {} of 8", index % 8),
  }
}

fn grad3(index: usize) -> Vector3 {
  // Vectors are combinations of -1, 0, and 1
  // Precompute the normalized elements
  const DIAG: f64 = core::f64::consts::FRAC_1_SQRT_2;
  const DIAG2: f64 = 0.577_350_269_189_625_8;

  match index % 32 {
    // 12 edges repeated twice then 8 corners
    0 | 12 => Vector3::new(DIAG, DIAG, 0.0),
    1 | 13 => Vector3::new(-DIAG, DIAG, 0.0),
    2 | 14 => Vector3::new(DIAG, -DIAG, 0.0),
    3 | 15 => Vector3::new(-DIAG, -DIAG, 0.0),
    4 | 16 => Vector3::new(DIAG, 0.0, DIAG),
    5 | 17 => Vector3::new(-DIAG, 0.0, DIAG),
    6 | 18 => Vector3::new(DIAG, 0.0, -DIAG),
    7 | 19 => Vector3::new(-DIAG, 0.0, -DIAG),
    8 | 20 => Vector3::new(0.0, DIAG, DIAG),
    9 | 21 => Vector3::new(0.0, -DIAG, DIAG),
    10 | 22 => Vector3::new(0.0, DIAG, -DIAG),
    11 | 23 => Vector3::new(0.0, -DIAG, -DIAG),
    24 => Vector3::new(DIAG2, DIAG2, DIAG2),
    25 => Vector3::new(-DIAG2, DIAG2, DIAG2),
    26 => Vector3::new(DIAG2, -DIAG2, DIAG2),
    27 => Vector3::new(-DIAG2, -DIAG2, DIAG2),
    28 => Vector3::new(DIAG2, DIAG2, -DIAG2),
    29 => Vector3::new(-DIAG2, DIAG2, -DIAG2),
    30 => Vector3::new(DIAG2, -DIAG2, -DIAG2),
    31 => Vector3::new(-DIAG2, -DIAG2, -DIAG2),
    _ => panic!("Attempt to access gradient {} of 32", index % 32),
  }
}

// TODO: Expose this in a more useful way.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector2 {
  pub x: f64,
  pub y: f64,
}

impl Vector2 {
  pub const fn new(x: f64, y: f64) -> Self { Vector2 { x, y } }
  pub const fn broadcast(value: f64) -> Self { Vector2 { x: value, y: value } }

  pub fn floor(self) -> Vector2 { Vector2 { x: self.x.floor(), y: self.y.floor() } }

  pub fn sum(self) -> f64 { self.x + self.y }
  pub fn dot(self, other: Vector2) -> f64 { self.x * other.x + self.y * other.y }

  pub fn magnitude_squared(self) -> f64 { self.dot(self) }
}

impl Add<Vector2> for Vector2 {
  type Output = Vector2;

  fn add(self, other: Vector2) -> Vector2 { Vector2 { x: self.x + other.x, y: self.y + other.y } }
}

impl Sub<Vector2> for Vector2 {
  type Output = Vector2;

  fn sub(self, other: Vector2) -> Vector2 { Vector2 { x: self.x - other.x, y: self.y - other.y } }
}

impl Mul<f64> for Vector2 {
  type Output = Vector2;

  fn mul(self, scalar: f64) -> Vector2 { Vector2 { x: self.x * scalar, y: self.y * scalar } }
}

impl AddAssign for Vector2 {
  fn add_assign(&mut self, other: Vector2) { *self = *self + other; }
}

impl MulAssign<f64> for Vector2 {
  fn mul_assign(&mut self, scalar: f64) { *self = *self * scalar; }
}

// TODO: Expose this in a more useful way.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vector3 {
  pub const fn new(x: f64, y: f64, z: f64) -> Self { Vector3 { x, y, z } }
  pub const fn broadcast(value: f64) -> Self { Vector3 { x: value, y: value, z: value } }

  pub fn floor(self) -> Vector3 {
    Vector3 { x: self.x.floor(), y: self.y.floor(), z: self.z.floor() }
  }

  pub fn sum(self) -> f64 { self.x + self.y + self.z }
  pub fn dot(self, other: Vector3) -> f64 { self.x * other.x + self.y * other.y + self.z * other.z }

  pub fn magnitude_squared(self) -> f64 { self.dot(self) }
}

impl Add<Vector3> for Vector3 {
  type Output = Vector3;

  fn add(self, other: Vector3) -> Vector3 {
    Vector3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
  }
}

impl Sub<Vector3> for Vector3 {
  type Output = Vector3;

  fn sub(self, other: Vector3) -> Vector3 {
    Vector3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
  }
}

impl Mul<f64> for Vector3 {
  type Output = Vector3;

  fn mul(self, scalar: f64) -> Vector3 {
    Vector3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
  }
}

impl AddAssign for Vector3 {
  fn add_assign(&mut self, other: Vector3) { *self = *self + other; }
}

impl MulAssign<f64> for Vector3 {
  fn mul_assign(&mut self, scalar: f64) { *self = *self * scalar; }
}
