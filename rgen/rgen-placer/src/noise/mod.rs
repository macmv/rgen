mod octaved;
mod open_simplex;
mod perlin;
mod voronoi;

pub use octaved::OctavedNoise;
pub use open_simplex::OpenSimplexNoise;
pub use perlin::PerlinNoise;
pub use voronoi::VoronoiNoise;

pub trait SeededNoise {
  fn new(seed: u64) -> Self;
}

pub trait NoiseGenerator {
  type Output;

  fn generate(&self, x: f64, y: f64) -> Self::Output;
}

pub trait NoiseGenerator3D {
  fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64;
}
