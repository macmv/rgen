mod octaved;
mod open_simplex;
mod perlin;

pub use octaved::OctavedNoise;
pub use open_simplex::OpenSimplexNoise;
pub use perlin::PerlinNoise;

pub trait SeededNoise {
  fn new(seed: u64) -> Self;
}

pub trait NoiseGenerator {
  fn generate(&self, x: f64, y: f64) -> f64;
}

pub trait NoiseGenerator3D {
  fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64;
}
