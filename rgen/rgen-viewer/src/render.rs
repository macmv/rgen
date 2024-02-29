use sdl2::pixels::Color;

pub struct RenderBuffer {
  // Width and height in pixels.
  width:  u32,
  height: u32,
  buffer: Vec<u8>,
}

impl RenderBuffer {
  pub fn new(width: u32, height: u32) -> RenderBuffer {
    let buffer = vec![0; (width * height * 4) as usize];

    RenderBuffer { width, height, buffer }
  }

  pub fn set(&mut self, x: i32, y: i32, color: Color) {
    if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
      return;
    }

    let index = (y * self.width as i32 + x) as usize * 4;

    self.buffer[index + 0] = color.b;
    self.buffer[index + 1] = color.g;
    self.buffer[index + 2] = color.r;
    self.buffer[index + 3] = color.a;
  }

  pub fn copy_to_sdl2(&self, texture: &mut sdl2::render::Texture) {
    texture.update(None, &self.buffer, (self.width * 4) as usize).unwrap();
  }
}
