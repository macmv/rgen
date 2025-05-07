// An insecure, fast random number generator.
#[derive(Debug, Copy, Clone)]
pub struct Rng {
  seed: u64,
}

impl Rng {
  pub fn new(seed: u64) -> Rng { Rng { seed } }
}

impl Random for Rng {
  // This is the wyhash generator. See
  // https://lemire.me/blog/2019/03/19/the-fastest-conventional-random-number-generator-that-can-pass-big-crush/
  fn next(&mut self) -> u64 {
    self.seed = self.seed.wrapping_add(0x60bee2bee120fc15);

    let t1 = (self.seed as u128).wrapping_mul(0xa3b195354a39b70d);
    let m1 = ((t1 >> 64) ^ t1) as u64;

    let t2 = (m1 as u128).wrapping_mul(0x1b03738712fad5c9);

    ((t2 >> 64) ^ t2) as u64
  }
}

pub trait Random {
  fn next(&mut self) -> u64;

  fn shuffle<T>(&mut self, slice: &mut [T]) {
    for i in 0..slice.len() {
      let j = self.range(0..slice.len() as i32) as usize;
      slice.swap(i, j);
    }
  }

  fn range<T: RandRange>(&mut self, range: impl RangeBounds<T>) -> T {
    let (min, max) = range.bounds();
    if range.is_inclusive() {
      assert!(min <= max, "min must be less than or equal to max");

      let v = T::from_bits(self.next());
      v.mod_range_inclusive(min, max)
    } else {
      assert!(min < max, "min must be less than max");

      let v = T::from_bits(self.next());
      v.mod_range_exclusive(min, max)
    }
  }

  #[track_caller]
  fn choose<'a, T>(&mut self, choices: &'a [T]) -> &'a T {
    if choices.is_empty() {
      panic!("Can't choose from an empty list");
    }

    let index = self.range(0..choices.len() as i32);
    &choices[index as usize]
  }
}

pub trait RandRange: PartialOrd {
  fn from_bits(bits: u64) -> Self;
  fn mod_range_inclusive(self, min: Self, max: Self) -> Self;
  fn mod_range_exclusive(self, min: Self, max: Self) -> Self;
}

pub trait RangeBounds<T> {
  fn bounds(&self) -> (T, T);
  fn is_inclusive(self) -> bool;
}

impl<T: Copy> RangeBounds<T> for std::ops::RangeInclusive<T> {
  fn bounds(&self) -> (T, T) { (*self.start(), *self.end()) }
  fn is_inclusive(self) -> bool { true }
}
impl<T: Copy> RangeBounds<T> for std::ops::Range<T> {
  fn bounds(&self) -> (T, T) { (self.start, self.end) }
  fn is_inclusive(self) -> bool { false }
}

impl RandRange for i32 {
  fn from_bits(bits: u64) -> i32 { bits as i32 }
  fn mod_range_inclusive(self, min: i32, max: i32) -> i32 { self.rem_euclid(max - min + 1) + min }
  fn mod_range_exclusive(self, min: i32, max: i32) -> i32 { self.rem_euclid(max - min) + min }
}

impl RandRange for f64 {
  fn from_bits(bits: u64) -> f64 { (bits as f64) / u64::MAX as f64 }
  fn mod_range_exclusive(self, min: f64, max: f64) -> f64 { self * (max - min) + min }
  fn mod_range_inclusive(self, min: f64, max: f64) -> f64 { self * (max - min) + min }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rng_basic() {
    let mut rng = Rng::new(1234);

    assert_eq!(rng.next(), 15262463794981090671);
    assert_eq!(rng.next(), 12585898128285935653);
    assert_eq!(rng.next(), 5637883660013241997);
  }

  #[test]
  fn rng_i32_range() {
    let mut rng = Rng::new(1234);

    assert_eq!(rng.range(0..=100), 3);
    assert_eq!(rng.range(0..=100), 12);
    assert_eq!(rng.range(0..=100), 0);
    assert_eq!(rng.range(0..=100), 91);
  }

  #[test]
  fn rng_f64_range() {
    let mut rng = Rng::new(1234);

    assert_eq!(rng.range(0.0..=100.0), 82.73798201999928);
    assert_eq!(rng.range(0.0..=100.0), 68.22829046684427);
    assert_eq!(rng.range(0.0..=100.0), 30.56302856203442);
    assert_eq!(rng.range(0.0..=100.0), 38.638413541668285);
  }

  #[test]
  fn i32_range() {
    assert_eq!(5.mod_range_exclusive(0, 10), 5);
    assert_eq!(15.mod_range_exclusive(0, 10), 5);
    assert_eq!((-5).mod_range_exclusive(0, 10), 5);
  }

  #[test]
  fn f64_range() {
    assert_eq!(0.5.mod_range_exclusive(0.0, 10.0), 5.0);
  }
}
