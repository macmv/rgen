use super::{NoiseGenerator, NoiseGenerator3D};

#[derive(Debug, Copy, Clone)]
pub struct OctavedNoise<Noise> {
  pub octaves: usize,
  pub freq:    f64,
  pub pers:    f64,
  pub lacu:    f64,
  pub noise:   Noise,
}

impl<N: Default> Default for OctavedNoise<N> {
  fn default() -> Self { Self { octaves: 4, freq: 1.0, pers: 0.5, lacu: 2.0, noise: N::default() } }
}

impl<Noise: NoiseGenerator> NoiseGenerator for OctavedNoise<Noise> {
  fn generate(&self, x: f64, y: f64, seed: u64) -> f64 {
    let mut x = x * self.freq;
    let mut y = y * self.freq;

    let mut res = self.noise.generate(x, y, seed);

    for octave in 1..self.octaves {
      x *= self.lacu;
      y *= self.lacu;

      let seed = seed + octave as u64;
      res += self.noise.generate(x, y, seed) * self.pers.powi(octave as i32) * res;
    }

    // Make sure the noise is in the range [-1.0, 1.0).
    res.clamp(-1.0, 1.0 - 1e-6)
  }
}

impl<Noise: NoiseGenerator3D> NoiseGenerator3D for OctavedNoise<Noise> {
  fn generate_3d(&self, x: f64, y: f64, z: f64, seed: u64) -> f64 {
    let mut x = x * self.freq;
    let mut y = y * self.freq;
    let mut z = z * self.freq;
    let mut pers = 1.0f64;

    (0..self.octaves)
      .fold(0.0, |value, octave| {
        let seed = seed + octave as u64;
        let value = value + self.noise.generate_3d(x, y, z, seed) * pers;

        x *= self.lacu;
        y *= self.lacu;
        z *= self.lacu;
        pers *= self.pers;

        value
      })
      // FIXME: Don't clamp this here.
      .clamp(-1.0, 1.0 - 1e-6)
  }
}
