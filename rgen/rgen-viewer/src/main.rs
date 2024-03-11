use std::{collections::HashMap, sync::Arc, time::Instant};

use rgen_base::Pos;
use rgen_world::Context;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Texture};

mod queue;
mod region;
mod render;
mod terrain;
mod view;
mod world;

use terrain::TerrainGenerator;
use world::World;

use crate::{
  queue::RenderQueue,
  region::{RegionPos, REGION_SIZE},
  view::WorldViewer,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

const MIN_ZOOM: f64 = 0.5;
const MAX_ZOOM: f64 = 32.0;

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
  let world = Arc::new(World::new(context, terrain));
  let world_view = Arc::new(WorldViewer::new());

  let queue = Arc::new(RenderQueue::new());
  queue.spawn_generation_threads(&world);
  queue.spawn_render_threads(&world, &world_view);

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

  // Mouse position in pixels.
  let mut mouse_pos = (0, 0);
  // Current block hoverred on.
  let mut hover_pos = Pos::new(0, 0, 0);

  let mut zoom = 0.5_f64;
  // The top-left corner of the screen, in fractional blocks.
  let mut view_coords = (0.0, 0.0);
  let mut drag_pos = None;

  let texture_creator = render.canvas.texture_creator();
  let mut texture_cache = HashMap::<RegionPos, Texture>::new();

  let mut last_frame = Instant::now();

  'main: loop {
    for event in render.events.poll_iter() {
      match event {
        Event::Quit { .. } => break 'main,

        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,

        Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
          world_view.set_mode(RenderMode::Height);
          texture_cache.clear();
        }
        Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
          world_view.set_mode(RenderMode::Slope);
          texture_cache.clear();
        }
        Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
          world_view.set_mode(RenderMode::Aspect);
          texture_cache.clear();
        }
        Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
          world_view.set_mode(RenderMode::Brightness);
          texture_cache.clear();
        }
        Event::KeyDown { keycode: Some(Keycode::Num5), .. } => {
          world_view.set_mode(RenderMode::BiomeColors);
          texture_cache.clear();
        }

        Event::MouseButtonDown { x, y, .. } => drag_pos = Some((x, y)),
        Event::MouseButtonUp { .. } => drag_pos = None,

        Event::MouseWheel { y, .. } => {
          let zoom_after =
            if y > 0 { (zoom * 2.0).min(MAX_ZOOM) } else { (zoom / 2.0).max(MIN_ZOOM) };

          let mouse_block_x = view_coords.0 + mouse_pos.0 as f64 / zoom;
          let mouse_block_y = view_coords.1 + mouse_pos.1 as f64 / zoom;

          view_coords.0 = -(mouse_pos.0 as f64 / zoom_after as f64) + mouse_block_x;
          view_coords.1 = -(mouse_pos.1 as f64 / zoom_after as f64) + mouse_block_y;

          zoom = zoom_after;
        }

        Event::MouseMotion { x, y, .. } => {
          mouse_pos = (x, y);
          hover_pos = Pos::new(
            (view_coords.0 + x as f64 / zoom) as i32,
            0,
            (view_coords.1 + y as f64 / zoom) as i32,
          );

          if let Some((i_x, i_y)) = drag_pos {
            let d_x = (i_x - x) as f64 / zoom;
            let d_y = (i_y - y) as f64 / zoom;

            view_coords.0 += d_x;
            view_coords.1 += d_y;

            drag_pos = Some((x, y));
          }
        }

        _ => {}
      }
    }

    render.clear();

    let screen_width = render.canvas.output_size().unwrap().0;
    let screen_height = render.canvas.output_size().unwrap().1;

    let view_pos = Pos::new(view_coords.0.floor() as i32, 0, view_coords.1.floor() as i32);
    let max_pos = view_pos
      + Pos::new((screen_width as f64 / zoom) as i32, 0, (screen_height as f64 / zoom) as i32);

    // -1 to +1 to make sure we render all chunks that are partially in view.
    // We add an extra 1 chunk outside of that to make panning smoother.
    let min_chunk = RegionPos::from_pos(view_pos) + RegionPos::new(-2, -2);
    let max_chunk = RegionPos::from_pos(max_pos) + RegionPos::new(2, 2);

    world_view.recv_chunks();
    world.recv_chunks();

    {
      let generated_chunks = world.read();
      let rendered_chunks = world_view.read_chunks();

      queue.update(&generated_chunks, &rendered_chunks, |state| {
        state.min_chunk = min_chunk;
        state.max_chunk = max_chunk;
        state.center = (min_chunk + max_chunk) / 2;

        let half_screen = state.center - min_chunk;
        state.radius = half_screen.x.max(half_screen.z);
      });

      for chunk_x in min_chunk.x..=max_chunk.x {
        for chunk_z in min_chunk.z..=max_chunk.z {
          let region_pos = RegionPos::new(chunk_x, chunk_z);

          let tex = match texture_cache.get(&region_pos) {
            Some(t) => t,
            None => {
              if let Some(c) = rendered_chunks.get(&region_pos) {
                let mut tex = texture_creator
                  .create_texture_streaming(
                    Some(sdl2::pixels::PixelFormatEnum::ARGB8888),
                    REGION_SIZE as u32,
                    REGION_SIZE as u32,
                  )
                  .unwrap();

                c.copy_to_sdl2(&mut tex);

                texture_cache.insert(region_pos, tex);
                texture_cache.get(&region_pos).unwrap()
              } else {
                continue;
              }
            }
          };

          let pos = region_pos.min_block_pos();
          render.canvas.copy(
            &tex,
            None,
            Some(Rect::new(
              (pos.x as f64 * zoom - view_coords.0 * zoom) as i32,
              (pos.z as f64 * zoom - view_coords.1 * zoom) as i32,
              (zoom * REGION_SIZE as f64) as u32,
              (zoom * REGION_SIZE as f64) as u32,
            )),
          )?;
        }
      }

      let meter_height = generated_chunks.height_at(hover_pos);

      if let Some(f) = &font {
        let mut f = FontRender { font: f, render: &mut render };

        f.render(0, 0, format!("X: {x:0.2} Z: {z:0.2}", x = hover_pos.x, z = hover_pos.z));
        f.render(0, 24, format!("Height: {meter_height:0.2}"));

        let biome = world.generator.biomes.choose_biome(world.generator.seed, hover_pos);
        f.render(0, 48, format!("Biome: {}", biome.name));
      }

      render.canvas.set_draw_color(Color::RGB(0, 0, 255));
      render.canvas.draw_rect(Rect::new(
        hover_pos.x() * zoom as i32 - (view_coords.0 * zoom) as i32,
        hover_pos.z() * zoom as i32 - (view_coords.1 * zoom) as i32,
        zoom as u32,
        zoom as u32,
      ))?;
    }

    render.present();

    let elapsed = last_frame.elapsed();
    last_frame = Instant::now();
    // Don't render too much.
    if elapsed.as_millis() < 16 {
      std::thread::sleep(std::time::Duration::from_millis(16) - elapsed);
    }
  }

  Ok(())
}

impl World<TerrainGenerator> {}

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
