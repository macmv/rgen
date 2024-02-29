use std::{sync::Arc, time::Instant};

use crossbeam_channel::{Sender, TrySendError};
use parking_lot::RwLock;
use rgen_base::{Biome, ChunkPos, Pos};
use rgen_world::Context;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

mod render;
mod terrain;
mod view;
mod world;

use terrain::TerrainGenerator;
use world::World;

use crate::view::WorldViewer;

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
  let world = Arc::new(RwLock::new(World::new(context, terrain)));

  let request_chunk = spawn_generation_thread(&world);

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

  let mut hover_pos = Pos::new(0, 0, 0);

  let screen_width = 1920;
  let screen_height = 1080;

  let mut world_view = WorldViewer::new(screen_width, screen_height);

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

        Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
          world_view.set_mode(RenderMode::Height)
        }
        Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
          world_view.set_mode(RenderMode::Slope)
        }
        Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
          world_view.set_mode(RenderMode::Aspect)
        }
        Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
          world_view.set_mode(RenderMode::Brightness)
        }
        Event::KeyDown { keycode: Some(Keycode::Num5), .. } => {
          world_view.set_mode(RenderMode::BiomeColors)
        }

        Event::MouseMotion { x, y, .. } => {
          hover_pos = Pos::new(x / 4, 0, y / 4);
        }

        _ => {}
      }
    }

    let max_pos = Pos::new(screen_width as i32 / 4, 0, screen_height as i32 / 4);
    let max_chunk = ChunkPos::new(max_pos.x / 16, max_pos.z / 16);

    {
      let w = world.read();

      let t = Instant::now();

      'chunk_building: for chunk_x in 0..=max_chunk.x + 1 {
        for chunk_z in 0..=max_chunk.z + 1 {
          let chunk_pos = ChunkPos::new(chunk_x, chunk_z);

          // Only place chunks for 16ms.
          if t.elapsed().as_millis() > 16 {
            break 'chunk_building;
          }

          if w.has_chunk(chunk_pos) {
            world_view.place_chunk(&w, chunk_pos);
          } else {
            match request_chunk.try_send(chunk_pos) {
              Ok(()) => {}
              Err(TrySendError::Disconnected(_)) => {
                panic!("chunk generation died");
              }
              Err(TrySendError::Full(_)) => {}
            }
            continue;
          }
        }
      }

      // NB: Segfaults if you screw up the buffer size.
      world_view.grid.buffer.copy_to_sdl2(&mut screen_texture);
      render.canvas.copy(&screen_texture, None, None)?;

      let meter_height = w.height_at(hover_pos);

      if let Some(f) = &font {
        let mut f = FontRender { font: f, render: &mut render };

        f.render(0, 0, format!("X: {x:0.2} Z: {z:0.2}", x = hover_pos.x, z = hover_pos.z));
        f.render(0, 24, format!("Height: {meter_height:0.2}"));

        //let biome = world.biome_at(hover_pos);
        //f.render(0, 48, format!("Biome: {}",
        // world.context.biomes.name_of(biome)));
      }
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

fn spawn_generation_thread(world: &Arc<RwLock<World<TerrainGenerator>>>) -> Sender<ChunkPos> {
  // Spawn up 16 threads to generate chunks.
  const POOL_SIZE: usize = 16;

  let (tx, rx) = crossbeam_channel::bounded(POOL_SIZE * 8);

  for _ in 0..POOL_SIZE {
    let rx = rx.clone();
    let world = world.clone();

    std::thread::spawn(move || loop {
      let chunk_pos = match rx.recv() {
        Ok(p) => p,
        Err(_) => break,
      };
      let mut w = world.write();
      w.generate_chunk(chunk_pos);
    });
  }

  tx
}
