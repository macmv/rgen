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
