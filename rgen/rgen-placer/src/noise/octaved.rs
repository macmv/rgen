use std::f64::consts::E;

use super::{NoiseGenerator, NoiseGenerator3D, SeededNoise};

#[derive(Debug, Copy, Clone)]
pub struct OctavedNoise<Noise, const O: usize> {
  pub freq:   f64,
  pub pers:   f64,
  pub lacu:   f64,
  pub layers: [Noise; O],
}

impl<N: SeededNoise, const O: usize> OctavedNoise<N, O> {
  pub fn new(seed: u64, freq: f64) -> Self {
    Self {
      freq,
      pers: 0.5,
      lacu: 2.0,
      layers: match (0..O).map(|i| N::new(seed + i as u64)).collect::<Vec<_>>().try_into() {
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

    res.clamp(-1.0, 1.0 - 1e-6)
  }
}

impl<Noise: NoiseGenerator3D, const O: usize> NoiseGenerator3D for OctavedNoise<Noise, O> {
  fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64 {
    let mut x = x * self.freq;
    let mut y = y * self.freq;
    let mut z = z * self.freq;

    let mut res = self.layers[0].generate_3d(x, y, z);

    for octave in 1..O {
      x *= self.lacu;
      y *= self.lacu;
      z *= self.lacu;

      res += self.layers[octave].generate_3d(x, y, z) * self.pers.powi(octave as i32);
    }

    smooth(res)
  }
}

fn smooth(t: f64) -> f64 {
  // Pass the result through a sigmoid function, to smooth out the values beyond
  // [-1, 1].
  let res = 2.0 / (1.0 + E.powf(-3.0 * t)) - 1.0;

  // Clamp the result so that we _never_ return a value outside [-1, 1].
  res.clamp(-1.0, 1.0)
}

#[cfg(test)]
mod tests {
  use crate::noise::{OpenSimplexNoise, PerlinNoise};

  use super::*;

  #[test]
  fn octaved_noise_works() {
    let noise = OctavedNoise::<OpenSimplexNoise, 3>::new(0, 1.0);

    for x in 0..100 {
      for y in 0..100 {
        let v = noise.generate(x as f64 / 100.0, y as f64 / 100.0);
        assert!(v >= -1.0 && v < 1.0, "v = {}", v);
      }
    }
  }

  #[test]
  fn octaved_noise_3d_works() {
    let noise = OctavedNoise::<PerlinNoise, 3>::new(0, 1.0);

    for x in 0..100 {
      for y in 0..100 {
        for z in 0..100 {
          let v = noise.generate_3d(x as f64 / 100.0, y as f64 / 100.0, z as f64 / 100.0);
          assert!(v >= -1.0 && v < 1.0, "v = {}", v);
        }
      }
    }
  }
}
