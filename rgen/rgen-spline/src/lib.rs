pub struct Spline<T> {
  pub storage: T,
}

impl<T> Spline<T> {
  pub fn new(storage: T) -> Self { Spline { storage } }
}

impl Spline<Vec<(f64, f64)>> {
  pub fn from_vec(storage: Vec<(f64, f64)>) -> Self { Spline { storage } }
}
impl<'a> Spline<&'a [(f64, f64)]> {
  pub fn from_slice(storage: &'a [(f64, f64)]) -> Self { Spline { storage } }
}

pub trait SplineStorage {
  fn len(&self) -> usize;
  fn get(&self, index: usize) -> (f64, f64);
}

impl SplineStorage for Vec<(f64, f64)> {
  fn len(&self) -> usize { self.len() }
  fn get(&self, index: usize) -> (f64, f64) { self[index] }
}

impl SplineStorage for [(f64, f64)] {
  fn len(&self) -> usize { self.as_ref().len() }
  fn get(&self, index: usize) -> (f64, f64) { self[index] }
}

impl<T: SplineStorage> Spline<T> {
  fn key(&self, index: usize) -> f64 { self.storage.get(index).0 }
  fn value(&self, index: usize) -> f64 { self.storage.get(index).1 }

  pub fn sample(&self, pos: f64) -> f64 {
    if pos < 0.0 || pos > 1.0 || self.storage.len() == 0 {
      return 0.0;
    }

    let len = self.storage.len();
    if len == 0 {
      return 0.0;
    }
    if len == 1 {
      return self.value(0);
    }
    if len == 2 {
      return self.value(0) + (self.value(1) - self.value(0)) * pos;
    }
    let mut i = 0;
    while i < len - 1 {
      let t0 = self.value(i);
      let t1 = self.value(i + 1);
      if pos >= t0 && pos <= t1 {
        let t = (pos - t0) / (t1 - t0);
        return t0 + (t1 - t0) * t;
      }
      i += 1;
    }
    self.value(len - 1)
  }
}
