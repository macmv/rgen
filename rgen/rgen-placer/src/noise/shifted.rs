use super::NoiseGenerator;

#[derive(Debug, Copy, Clone)]
pub struct ShiftedNoise<Noise, Shift> {
  pub base:  Noise,
  pub shift: Shift,

  pub shift_x: f64,
  pub shift_y: f64,
}

impl<N, S> ShiftedNoise<N, S> {
  pub fn new(base: N, shift: S, shift_x: f64, shift_y: f64) -> Self {
    Self { base, shift, shift_x, shift_y }
  }
}

impl<Noise: NoiseGenerator, Shift: NoiseGenerator<Output = f64>> NoiseGenerator
  for ShiftedNoise<Noise, Shift>
{
  type Output = Noise::Output;

  fn generate(&self, x: f64, y: f64) -> Noise::Output {
    let shift_x = self.shift.generate(x, y) * self.shift_x;
    let shift_y = self.shift.generate(y, x) * self.shift_y;

    self.base.generate(x + shift_x, y + shift_y)
  }
}
