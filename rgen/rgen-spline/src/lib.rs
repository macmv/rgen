pub struct Spline<T> {
  pub storage: T,
}

impl<T> Spline<T> {
  pub fn new(storage: T) -> Self { Spline { storage } }
}

impl Spline<Vec<f64>> {
  pub fn from_vec(storage: Vec<f64>) -> Self { Spline { storage } }
}
impl<'a> Spline<&'a [f64]> {
  pub fn from_slice(storage: &'a [f64]) -> Self { Spline { storage } }
}

pub trait SplineStorage {
  fn len(&self) -> usize;
  fn get(&self, index: usize) -> f64;
}

impl SplineStorage for Vec<f64> {
  fn len(&self) -> usize { self.len() }
  fn get(&self, index: usize) -> f64 { self[index] }
}

impl SplineStorage for [f64] {
  fn len(&self) -> usize { self.as_ref().len() }
  fn get(&self, index: usize) -> f64 { self[index] }
}
