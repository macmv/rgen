pub mod octaved;
pub mod perlin;

pub trait NoiseGenerator {
  fn generate(&self, x: f64, y: f64, seed: u64) -> f64;
}
