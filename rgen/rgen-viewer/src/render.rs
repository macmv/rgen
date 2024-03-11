use sdl2::{pixels::Color, rect::Rect};

pub struct RenderBuffer {
  // Width and height in pixels.
  width:  u32,
  height: u32,
  buffer: Vec<u8>,
}

pub struct Render {
  #[allow(unused)]
  sdl_context: sdl2::Sdl,

  pub ttf_context: Option<sdl2::ttf::Sdl2TtfContext>,
  pub events:      sdl2::EventPump,
  pub canvas:      sdl2::render::Canvas<sdl2::video::Window>,
}

pub struct FontRender<'a> {
  pub font:   &'a sdl2::ttf::Font<'a, 'a>,
  pub render: &'a mut Render,
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
    // NB: Segfaults if you screw up the buffer size.
    texture.update(None, &self.buffer, (self.width * 4) as usize).unwrap();
  }
}

impl Render {
  pub fn new() -> Result<Render, String> {
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video()?;

    let screen = video_subsystem.current_display_mode(0).unwrap();

    let window = video_subsystem
      .window("RGen Viewer", (screen.w / 2) as u32, (screen.h / 2) as u32)
      .position_centered()
      .resizable()
      .build()
      .map_err(|e| e.to_string())?;

    let events = sdl_context.event_pump()?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    Ok(Render { sdl_context, ttf_context: Some(ttf_context), events, canvas })
  }

  pub fn clear(&mut self) {
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.clear();
  }

  pub fn present(&mut self) { self.canvas.present(); }
}

impl FontRender<'_> {
  pub fn render(&mut self, x: i32, y: i32, text: impl AsRef<str>) {
    let texture_creator = self.render.canvas.texture_creator();

    let surface = self.font.render(text.as_ref()).blended(Color::RGB(255, 255, 255)).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    self
      .render
      .canvas
      .copy(&texture, None, Rect::new(x, y, surface.width(), surface.height()))
      .unwrap();
  }
}
