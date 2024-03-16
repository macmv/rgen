use super::{NoiseGenerator, NoiseGenerator3D};

#[derive(Debug, Copy, Clone)]
pub struct OctavedNoise<Noise, const O: usize> {
  pub freq:   f64,
  pub pers:   f64,
  pub lacu:   f64,
  pub layers: [Noise; O],
}

impl<N, const O: usize> OctavedNoise<N, O> {
  pub fn new(freq: f64, f: impl Fn(u64) -> N) -> Self {
    Self {
      freq,
      pers: 0.5,
      lacu: 2.0,
      layers: match (0..O).map(|i| f(i as u64)).collect::<Vec<_>>().try_into() {
        Ok(layers) => layers,
        Err(_) => unreachable!(),
      },
    }
  }

  pub fn with_freq(mut self, freq: f64) -> Self {
    self.freq = freq;
    self
  }
  pub fn with_pers(mut self, pers: f64) -> Self {
    self.pers = pers;
    self
  }
  pub fn with_lacu(mut self, lacu: f64) -> Self {
    self.lacu = lacu;
    self
  }
}

impl<Noise: NoiseGenerator, const O: usize> NoiseGenerator for OctavedNoise<Noise, O> {
  fn generate(&self, x: f64, y: f64) -> f64 {
    let mut x = x * self.freq;
    let mut y = y * self.freq;

    let mut res = self.layers[0].generate(x, y);

    for octave in 1..O {
      x *= self.lacu;
      y *= self.lacu;

      res += self.layers[octave].generate(x, y) * self.pers.powi(octave as i32);
    }

    // Make sure the noise is in the range [-1.0, 1.0).
    res.clamp(-1.0, 1.0 - 1e-6)
  }
}

impl<Noise: NoiseGenerator3D, const O: usize> NoiseGenerator3D for OctavedNoise<Noise, O> {
  fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64 {
    let mut x = x * self.freq;
    let mut y = y * self.freq;
    let mut z = z * self.freq;
    let mut pers = 1.0f64;

    (0..O)
      .fold(0.0, |value, octave| {
        let value = value + self.layers[octave].generate_3d(x, y, z) * pers;

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
