// An insecure, fast random number generator.
pub struct Rng {
  seed: u64,
}

impl Random for Rng {
  // This is the wyhash generator. See
  // https://lemire.me/blog/2019/03/19/the-fastest-conventional-random-number-generator-that-can-pass-big-crush/
  fn next(&mut self) -> u64 {
    self.seed += 0x60bee2bee120fc15;

    let t1 = (self.seed as u128) * 0xa3b195354a39b70d;
    let m1 = ((t1 >> 64) ^ t1) as u64;

    let t2 = (m1 as u128) * 0x1b03738712fad5c9;
    let m2 = ((t2 >> 64) ^ t2) as u64;

    m2
  }
}

pub trait Random {
  fn next(&mut self) -> u64;

  fn rand_inclusive(&mut self, min: i32, max: i32) -> i32 {
    let range = max - min;
    let rand = self.next() as i32;
    return min + (rand % (range + 1));
  }
}
