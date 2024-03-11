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
    let m2 = ((t2 >> 64) ^ t2) as u64;

    m2
  }
}

pub trait Random {
  fn next(&mut self) -> u64;

  fn shuffle<T>(&mut self, slice: &mut [T]) {
    for i in 0..slice.len() {
      let j = self.rand_exclusive(0, slice.len() as i32) as usize;
      slice.swap(i, j);
    }
  }

  #[track_caller]
  fn rand_inclusive(&mut self, min: i32, max: i32) -> i32 {
    assert!(min <= max, "min must be less than or equal to max");

    let range = max - min;
    let rand = (self.next() & 0x7fffffff) as i32;
    return min + (rand % (range + 1));
  }

  #[track_caller]
  fn rand_exclusive(&mut self, min: i32, max: i32) -> i32 {
    assert!(min < max, "min must be less than max");

    let range = max - min;
    let rand = (self.next() & 0x7fffffff) as i32;
    return min + (rand % range);
  }

  #[track_caller]
  fn choose<'a, T>(&mut self, choices: &'a [T]) -> &'a T {
    if choices.is_empty() {
      panic!("Can't choose from an empty list");
    }

    let index = self.rand_exclusive(0, choices.len() as i32);
    &choices[index as usize]
  }
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
}
