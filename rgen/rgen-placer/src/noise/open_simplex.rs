use super::NoiseGenerator;

use std::{
  cell::RefCell,
  ops::{Add, AddAssign, Mul, MulAssign, Sub},
};

#[derive(Default, Debug, Copy, Clone)]
pub struct OpenSimplexNoise;

impl NoiseGenerator for OpenSimplexNoise {
  fn generate(&self, x: f64, y: f64, seed: u64) -> f64 {
    thread_local! {
      static PERM: RefCell<PermutationTable> = RefCell::new(PermutationTable::init(0));
    }

    PERM.with(|p| noise_2(Vector2::new(x, y), &p.borrow()))
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

  let point = Vector2::from(point);

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
