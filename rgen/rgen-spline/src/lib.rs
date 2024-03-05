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
  fn binary_search(&self, key: f64) -> usize;
}

impl SplineStorage for Vec<(f64, f64)> {
  fn len(&self) -> usize { self.len() }
  fn get(&self, index: usize) -> (f64, f64) { self[index] }
  fn binary_search(&self, key: f64) -> usize {
    match self.binary_search_by(|(k, _)| k.partial_cmp(&key).unwrap()) {
      Ok(i) => i,
      Err(i) => i,
    }
  }
}

impl SplineStorage for [(f64, f64)] {
  fn len(&self) -> usize { self.as_ref().len() }
  fn get(&self, index: usize) -> (f64, f64) { self[index] }
  fn binary_search(&self, key: f64) -> usize {
    match self.binary_search_by(|(k, _)| k.partial_cmp(&key).unwrap()) {
      Ok(i) => i,
      Err(i) => i,
    }
  }
}

pub trait Interpolation {
  fn interpolate(t: f64, left: f64, right: f64) -> f64;
}

pub struct Linear;
impl Interpolation for Linear {
  fn interpolate(t: f64, left: f64, right: f64) -> f64 { left + (right - left) * t }
}

pub struct Cosine;
impl Interpolation for Cosine {
  fn interpolate(t: f64, left: f64, right: f64) -> f64 {
    let cos_t = (1.0 - (t * std::f64::consts::PI).cos()) * 0.5;
    Linear::interpolate(cos_t, left, right)
  }
}

impl<T: SplineStorage> Spline<T> {
  pub fn sample<I: Interpolation>(&self, pos: f64) -> f64 {
    if pos < 0.0 || pos > 1.0 || self.storage.len() == 0 {
      return 0.0;
    }

    let i = self.storage.binary_search(pos);

    if i == 0 {
      return self.storage.get(0).1;
    }

    let (left_k, left_v) = self.storage.get(i - 1);
    let (right_k, right_v) = self.storage.get(i);

    assert!(pos <= right_k);
    assert!(pos >= left_k);

    let t = (pos - left_k) / (right_k - left_k);
    I::interpolate(t, left_v, right_v)
  }
}
