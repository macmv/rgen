#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl Color {
  pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };

  pub fn from_hex(hex: u32) -> Color {
    let r = ((hex >> 16) & 0xff) as f32 / 255.0;
    let g = ((hex >> 8) & 0xff) as f32 / 255.0;
    let b = (hex & 0xff) as f32 / 255.0;

    Color { r, g, b }
  }

  pub fn from_gray(value: f32) -> Color { Color { r: value, g: value, b: value } }

  pub fn to_sdl2(self) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGB((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8)
  }

  pub fn fade(self, to: Color, alpha: f32) -> Self {
    Color {
      r: self.r * alpha + to.r * (1.0 - alpha),
      g: self.g * alpha + to.g * (1.0 - alpha),
      b: self.b * alpha + to.b * (1.0 - alpha),
    }
  }
}
