use rgen_base::{Biome, ChunkPos, Pos};
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
  let mut world = World::new(context, terrain);

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
        world.generate_chunk(chunk_pos);

        for rel_x in 0..16 {
          for rel_z in 0..16 {
            let pos = chunk_pos.min_block_pos() + Pos::new(rel_x, 0, rel_z);
            let column = world.column_at(pos);

            let biome = column.biome;
            let meter_height = column.height as f64;

            let block_distance = 1;
            // ╔═╦═╦═╗
            // ║a║b║c║
            // ╠═╬═╬═╣     MINECRAFT
            // ║d║é║f║     - X & Z is flat plane
            // ╠═╬═╬═╣     - Y is up
            // ║g║h║i║
            // ╚═╩═╩═╝ <- var table  || block_distance

            let a = world.height_at(pos + Pos::new(-block_distance, 0, block_distance));
            let b = world.height_at(pos + Pos::new(0, 0, block_distance));
            let c = world.height_at(pos + Pos::new(block_distance, 0, block_distance));

            let d = world.height_at(pos + Pos::new(-block_distance, 0, 0));
            let f = world.height_at(pos + Pos::new(block_distance, 0, 0));

            let g = world.height_at(pos + Pos::new(-block_distance, 0, -block_distance));
            let h = world.height_at(pos + Pos::new(0, 0, -block_distance));
            let i = world.height_at(pos + Pos::new(block_distance, 0, -block_distance));

            let dz_dx = ((c + (2.0 * f) + i) * 4.0 - (a + (2.0 * d) + g) * 4.0) / (8.0 * 1.0);
            //[dz/dx] = ((c + 2f + i)*4/wght1 - (a + 2d + g)*4/wght2) / (8 * x_cellsize)

            let dz_dy = ((g + (2.0 * h) + i) * 4.0 - (a + (2.0 * b) + c) * 4.0) / (8.0 * 1.0);
            //[dz/dy] = ((g + 2h + i)*4/wght3 - (a + 2b + c)*4/wght4) / (8 * y_cellsize)

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
              RenderMode::Aspect => {
                //let asp = (cell_aspect * 255.0 / std::f64::consts::PI) as u8;
                //println!("Aspect: {asp}");
                (cell_aspect * 255.0 / std::f64::consts::PI) as u8
              }
              RenderMode::Brightness => {
                //let bright = (brightness as f64 * 0.2 + meter_height as f64 * 2.0) as u8;
                //println!("Brightness: {bright}");
                (brightness as f64 * 0.2 + meter_height as f64 * 2.0) as u8
              }
              RenderMode::BiomeColors => 0,
              //
              //HSV
              //Hue:0-360         - This is the color of the terrain
              //Saturation:0-100  - This is the terrain height
              //Value:0-100       - Keep locked too set darkness to max-light
            };

            let height_color = Color::RGB(brightness, brightness, brightness);
            let biome_color = world.color_for_biome(biome);

            grid.set(
              pos.x,
              pos.z,
              //ERROR THAT I DON'T FEE LIKE FIXING TRACKED DOWN
              Color::RGB(
                height_color.r, //+ biome_color.r,
                height_color.g, //+ biome_color.g,
                height_color.b, //+ biome_color.b,
              ),
            );
          }
        }
      }
    }

    // NB: Segfaults if you screw up the buffer size.
    grid.buffer.copy_to_sdl2(&mut screen_texture);
    render.canvas.copy(&screen_texture, None, None)?;

    let meter_height = world.height_at(hover_pos);

    if let Some(f) = &font {
      let mut f = FontRender { font: f, render: &mut render };

      f.render(0, 0, format!("X: {x:0.2} Z: {z:0.2}", x = hover_pos.x, z = hover_pos.z));
      f.render(0, 24, format!("Height: {meter_height:0.2}"));

      //let biome = world.biome_at(hover_pos);
      //f.render(0, 48, format!("Biome: {}",
      // world.context.biomes.name_of(biome)));
    }

    render.canvas.set_draw_color(Color::RGB(0, 0, 255));
    render.canvas.draw_rect(Rect::new(hover_pos.x() * 4, hover_pos.z() * 4, 4, 4))?;

    render.present();
  }

  Ok(())
}

impl World<TerrainGenerator> {
  pub fn color_for_biome(&self, biome: Biome) -> Color {
    let biome_hex = match biome {
      b if b == self.context.biomes.ice_plains => 0x518ded,
      b if b == self.context.biomes.cold_taiga => 0x3265db,
      b if b == self.context.biomes.extreme_hills => 0x4f6aab,
      b if b == self.context.biomes.plains => 0x61b086,
      b if b == self.context.biomes.savanna => 0xa19d55,
      _ => {
        //println!("unknown biome {b:?}");
        0x000000
      }
    };

    Color::RGB((biome_hex >> 16) as u8 / 4, (biome_hex >> 8) as u8 / 4, biome_hex as u8 / 4)
  }

  pub fn biome_at(&self, pos: Pos) -> Biome {
    let chunk_pos = pos.chunk();
    let mut biomes = [0; 256];
    self.generator.generate_biomes(chunk_pos, &mut biomes);

    let rel = pos.chunk_rel();
    let i = (rel.x() * 16 + rel.z()) as usize;
    let biome_id = biomes[i];
    Biome::from_raw_id(biome_id.into())
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
