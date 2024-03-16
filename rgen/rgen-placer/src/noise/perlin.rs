use super::{NoiseGenerator, NoiseGenerator3D, SeededNoise};

#[derive(Default, Debug, Copy, Clone)]
pub struct PerlinNoise {
  pub seed: u64,
}

impl SeededNoise for PerlinNoise {
  fn new(seed: u64) -> Self { PerlinNoise { seed } }
}

impl NoiseGenerator for PerlinNoise {
  fn generate(&self, x: f64, y: f64) -> f64 {
    let x0 = if x > 0.0 { x as i32 } else { (x - 1.0) as i32 };
    let x1 = x0 + 1;

    let y0 = if y > 0.0 { y as i32 } else { (y - 1.0) as i32 };
    let y1 = y0 + 1;

    let xd = s_curve(x - x0 as f64);
    let yd = s_curve(y - y0 as f64);

    let x0y0 = generate_random_value(x0, y0, self.seed as i32);
    let x1y0 = generate_random_value(x1, y0, self.seed as i32);
    let x0y1 = generate_random_value(x0, y1, self.seed as i32);
    let x1y1 = generate_random_value(x1, y1, self.seed as i32);

    let v1 = interpolate(x0y0, x1y0, xd);
    let v2 = interpolate(x0y1, x1y1, xd);

    interpolate(v1, v2, yd)
  }
}

impl NoiseGenerator3D for PerlinNoise {
  fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64 {
    let x0 = if x > 0.0 { x as i32 } else { (x - 1.0) as i32 };
    let x1 = x0 + 1;

    let y0 = if y > 0.0 { y as i32 } else { (y - 1.0) as i32 };
    let y1 = y0 + 1;

    let z0 = if z > 0.0 { z as i32 } else { (z - 1.0) as i32 };
    let z1 = z0 + 1;

    let xd = s_curve(x - x0 as f64);
    let yd = s_curve(y - y0 as f64);
    let zd = s_curve(z - z0 as f64);

    let x0y0z0 = generate_random_value_3d(x0, y0, z0, self.seed as i32);
    let x1y0z0 = generate_random_value_3d(x1, y0, z0, self.seed as i32);
    let x0y1z0 = generate_random_value_3d(x0, y1, z0, self.seed as i32);
    let x1y1z0 = generate_random_value_3d(x1, y1, z0, self.seed as i32);
    let x0y0z1 = generate_random_value_3d(x0, y0, z1, self.seed as i32);
    let x1y0z1 = generate_random_value_3d(x1, y0, z1, self.seed as i32);
    let x0y1z1 = generate_random_value_3d(x0, y1, z1, self.seed as i32);
    let x1y1z1 = generate_random_value_3d(x1, y1, z1, self.seed as i32);

    let v1z0 = interpolate(x0y0z0, x1y0z0, xd);
    let v2z0 = interpolate(x0y1z0, x1y1z0, xd);

    let v1 = interpolate(v1z0, v2z0, yd);

    let v1z1 = interpolate(x0y0z1, x1y0z1, xd);
    let v2z1 = interpolate(x0y1z1, x1y1z1, xd);

    let v2 = interpolate(v1z1, v2z1, yd);

    interpolate(v1, v2, zd)
  }
}

fn generate_random_value(x: i32, y: i32, seed: i32) -> f64 {
  let m = {
    let n = ((x.wrapping_mul(157))
      .wrapping_add(y.wrapping_mul(31337))
      .wrapping_add(seed.wrapping_mul(2633)))
      & 0x7fffffff;
    (n << 13) ^ n
  };

  1.0
    - ((((m
      .wrapping_mul(m.wrapping_mul(m).wrapping_mul(15731).wrapping_add(789221))
      .wrapping_add(1376312579))
      & 0x7fffffff) as f64)
      / 1073741824.0)
}

fn generate_random_value_3d(x: i32, y: i32, z: i32, seed: i32) -> f64 {
  let m = {
    let n = ((x.wrapping_mul(157))
      .wrapping_add(y.wrapping_mul(31337))
      .wrapping_add(z.wrapping_mul(42821))
      .wrapping_add(seed.wrapping_mul(2633)))
      & 0x7fffffff;
    (n << 13) ^ n
  };

  1.0
    - ((((m
      .wrapping_mul(m.wrapping_mul(m).wrapping_mul(15731).wrapping_add(789221))
      .wrapping_add(1376312579))
      & 0x7fffffff) as f64)
      / 1073741824.0)
}

fn s_curve(a: f64) -> f64 { a * a * (3.0 - 2.0 * a) }

fn interpolate(v1: f64, v2: f64, a: f64) -> f64 { ((1.0 - a) * v1) + (a * v2) }
