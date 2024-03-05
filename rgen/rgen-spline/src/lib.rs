mod bezier_storage;
mod interpolation;
mod storage;

pub use bezier_storage::BezierStorage;
pub use interpolation::{Cosine, Interpolation, Linear};
pub use storage::SplineStorage;

pub struct Spline<T: ?Sized> {
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

impl<T: SplineStorage + ?Sized> Spline<T> {
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

impl<T: BezierStorage + ?Sized> Spline<T> {
  pub fn sample_bezier(&self, pos: f64) -> f64 {
    if pos < 0.0 || pos > 1.0 || self.storage.len() == 0 {
      return 0.0;
    }

    let i = self.storage.binary_search(pos);

    if i == 0 {
      return self.storage.get(0).1;
    }

    let (left_k, left_v, left_t) = self.storage.get(i - 1);
    let (right_k, right_v, _) = self.storage.get(i);

    assert!(pos <= right_k);
    assert!(pos >= left_k);

    let t = (pos - left_k) / (right_k - left_k);

    let one_t = 1.0 - t;

    left_t + (left_v - left_t) * one_t * one_t + (right_v - left_t) * t * t
  }
}
