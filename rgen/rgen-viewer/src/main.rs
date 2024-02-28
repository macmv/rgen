use rgen_base::{Biome, ChunkPos, Pos};
use rgen_placer::noise::NoiseGenerator;
use rgen_world::Context;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

mod render;
mod terrain;
mod world;

use terrain::TerrainGenerator;
use world::World;

use crate::render::RenderGrid;

enum RenderMode {
  /// Number 1
  Height,
  /// Number 2
  Slope,
  /// Number 3
  Aspect,
  /// Number 4
  Brightness,
  /// Number 5
  BiomeColors,
}

pub fn main() -> Result<(), String> {
  let arg = std::env::args().nth(1).unwrap_or("".to_string());

  let seed = if arg.is_empty() {
    rand::random::<u64>()
  } else {
    match arg.parse() {
      Ok(seed) => seed,
      Err(e) => {
        println!("Invalid seed: {}", e);
        std::process::exit(1);
      }
    }
  };
  println!("Using seed {}", seed);

  let mut render = Render::new()?;

  let context = Context::new_test(seed);
  let terrain = TerrainGenerator::new(&context.blocks, &context.biomes, context.seed);
  let world = World::new(context, terrain);

  render.clear();
  render.present();

  let ttf_context = render.ttf_context.take().unwrap();

  let path = "/usr/share/fonts/TTF/DejaVuSans.ttf";
  let font = match ttf_context.load_font(path, 24) {
    Ok(font) => Some(font),
    Err(e) => {
      println!("Failed to load font at {path}: {}", e);
      None
    }
  };

  let mut mode = RenderMode::Height;
  let mut hover_pos = Pos::new(0, 0, 0);

  let screen_width = 1920;
  let screen_height = 1080;

  let mut grid = RenderGrid::new(screen_width, screen_height, 4);

  let texture_creator = render.canvas.texture_creator();
  let mut screen_texture = texture_creator
    .create_texture_streaming(
      Some(sdl2::pixels::PixelFormatEnum::ARGB8888),
      screen_width as u32,
      screen_height as u32,
    )
    .unwrap();

  'main: loop {
    render.clear();

    for event in render.events.poll_iter() {
      match event {
        Event::Quit { .. } => break 'main,

        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,

        Event::KeyDown { keycode: Some(Keycode::Num1), .. } => mode = RenderMode::Height,
        Event::KeyDown { keycode: Some(Keycode::Num2), .. } => mode = RenderMode::Slope,
        Event::KeyDown { keycode: Some(Keycode::Num3), .. } => mode = RenderMode::Aspect,
        Event::KeyDown { keycode: Some(Keycode::Num4), .. } => mode = RenderMode::Brightness,
        Event::KeyDown { keycode: Some(Keycode::Num5), .. } => mode = RenderMode::BiomeColors,

        Event::MouseMotion { x, y, .. } => {
          hover_pos = Pos::new(x / 4, 0, y / 4);
        }

        _ => {}
      }
    }

    let max_pos = Pos::new(screen_width as i32 / 4, 0, screen_height as i32 / 4);
    let max_chunk = ChunkPos::new(max_pos.x / 16, max_pos.z / 16);

    for chunk_x in 0..=max_chunk.x + 1 {
      for chunk_z in 0..=max_chunk.z + 1 {
        let chunk_pos = ChunkPos::new(chunk_x, chunk_z);

        let mut biomes = [0; 256];
        world.generator.generate_biomes(chunk_pos, &mut biomes);

        for rel_x in 0..16 {
          for rel_z in 0..16 {
            let pos = chunk_pos.min_block_pos() + Pos::new(rel_x, 0, rel_z);
            let i = (rel_x * 16 + rel_z) as usize;
            let biome_id = biomes[i];
            let meter_height = world.meter_height(pos);

            let block_distance = -1;
            let cross_bottom = world.meter_height(pos + Pos::new(0, 0, -block_distance));
            let cross_top = world.meter_height(pos + Pos::new(0, 0, block_distance));
            let dz_dx = (cross_bottom - cross_top) / (2.0 * 1.0);

            let cross_right = world.meter_height(pos + Pos::new(block_distance, 0, 0));
            let cross_left = world.meter_height(pos + Pos::new(-block_distance, 0, 0));
            let dz_dy = (cross_right - cross_left) / (2.0 * 1.0);

            //claculates cell slope at that location

            let cell_slope = ((dz_dx).powi(2) + (dz_dy).powi(2)).sqrt().atan();
            //dbg!(cell_slope);
            //Slope = arctan(sqrt((dz/dx)^2 + (dz/dy)^2))

            //calulates cell aspect this is the direction the cell is facing as a slope
            let cell_aspect = dz_dx.atan2(-dz_dy);
            //arctan2(dz/dx, -dz/dy)

            let azimuth = 315.0 / 180.0 * std::f64::consts::PI;
            let altidue = 45.0 / 180.0 * std::f64::consts::PI;

            let solar_incidence_angle = (altidue.sin() * cell_slope.sin()
              + altidue.cos() * cell_slope.cos() * (azimuth - cell_aspect).cos())
            .acos();

            let brightness = ((((solar_incidence_angle).cos() + 1.0) / 2.0) * 255.0) as u8;

            let brightness = match mode {
              RenderMode::Height => (meter_height * 2.0) as u8,
              RenderMode::Slope => (cell_slope * 255.0 / std::f64::consts::PI) as u8,
              RenderMode::Aspect => (cell_aspect * 255.0 / std::f64::consts::PI) as u8,
              RenderMode::Brightness => (brightness as f64 * 0.2 + meter_height as f64 * 2.0) as u8,
              RenderMode::BiomeColors => 0,
            };

            let height_color = Color::RGB(brightness, brightness, brightness);
            let biome_color = world.color_for_biome(Biome::from_raw_id(biome_id.into()));

            grid.set(
              pos.x,
              pos.z,
              Color::RGB(
                biome_color.r + height_color.r,
                biome_color.g + height_color.g,
                biome_color.b + height_color.b,
              ),
            );
          }
        }
      }
    }

    // NB: Segfaults if you screw up the buffer size.
    grid.buffer.copy_to_sdl2(&mut screen_texture);
    render.canvas.copy(&screen_texture, None, None)?;

    let meter_height = world.meter_height(hover_pos);

    if let Some(f) = &font {
      let mut f = FontRender { font: f, render: &mut render };

      f.render(0, 0, format!("X: {x:0.2} Z: {z:0.2}", x = hover_pos.x, z = hover_pos.z));
      f.render(0, 24, format!("Height: {meter_height:0.2}"));
    }

    render.canvas.set_draw_color(Color::RGB(0, 0, 255));
    render.canvas.draw_rect(Rect::new(hover_pos.x() * 4, hover_pos.z() * 4, 4, 4))?;

    render.present();
  }

  Ok(())
}

impl World<TerrainGenerator> {
  pub fn height(&self, pos: Pos) -> f64 {
    let height =
      self.generator.height_map.generate(pos.x as f64, pos.z as f64, self.generator.seed) + 1.0;
    height
  }
  pub fn meter_height(&self, pos: Pos) -> f64 {
    let meter_height = self.height(pos) * 64.0;
    meter_height
  }

  pub fn color_for_biome(&self, biome: Biome) -> Color {
    let biome_hex = match biome {
      b if b == self.context.biomes.ice_plains => 0x518ded,
      b if b == self.context.biomes.cold_taiga => 0x3265db,
      b if b == self.context.biomes.extreme_hills => 0x4f6aab,
      b if b == self.context.biomes.plains => 0x61b086,
      b if b == self.context.biomes.savanna => 0xa19d55,
      b => {
        println!("unknown biome {b:?}");
        0x000000
      }
    };

    Color::RGB((biome_hex >> 16) as u8 / 8, (biome_hex >> 8) as u8 / 8, biome_hex as u8 / 8)
  }
}

struct Render {
  #[allow(unused)]
  sdl_context: sdl2::Sdl,
  ttf_context: Option<sdl2::ttf::Sdl2TtfContext>,
  events:      sdl2::EventPump,
  canvas:      sdl2::render::Canvas<sdl2::video::Window>,
}

impl Render {
  pub fn new() -> Result<Render, String> {
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
      .window("RGen Viewer", 1920, 1080)
      .position_centered()
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

struct FontRender<'a> {
  font:   &'a sdl2::ttf::Font<'a, 'a>,
  render: &'a mut Render,
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
