use super::NoiseGenerator;

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
    let mut pers = 1.0f64;

    (0..self.octaves)
      .fold(0.0, |value, octave| {
        let seed = seed + octave as u64;
        let value = value + self.noise.generate(x, y, seed) * pers;

        x *= self.lacu;
        y *= self.lacu;
        pers *= self.pers;

        value
      })
      // FIXME: Don't clamp this here.
      .clamp(-1.0, 1.0)
  }
}
