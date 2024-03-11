use std::{collections::HashMap, sync::Arc, time::Instant};

use rgen_base::{ChunkPos, Pos};
use rgen_world::Context;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Texture};

mod render;
mod spline_view;
mod terrain;
mod view;
mod world;

use terrain::TerrainGenerator;
use world::World;

use crate::view::WorldViewer;

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

  spawn_generation_thread(&world, &world_view);

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

  let mut zoom = 4;
  // The top-left corner of the screen, in fractional blocks.
  let mut view_coords = (0.0, 0.0);
  let mut drag_pos = None;

  let mut spline_view = spline_view::SplineViewer::new();

  let texture_creator = render.canvas.texture_creator();
  let mut texture_cache = HashMap::<ChunkPos, Texture>::new();

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
            if y > 0 { (zoom as i32 * 2).min(32) as u32 } else { (zoom as i32 / 2).max(1) as u32 };

          let mouse_block_x = view_coords.0 + mouse_pos.0 as f64 / zoom as f64;
          let mouse_block_y = view_coords.1 + mouse_pos.1 as f64 / zoom as f64;

          view_coords.0 = -(mouse_pos.0 as f64 / zoom_after as f64) + mouse_block_x;
          view_coords.1 = -(mouse_pos.1 as f64 / zoom_after as f64) + mouse_block_y;

          zoom = zoom_after;
        }

        Event::MouseMotion { x, y, .. } => {
          mouse_pos = (x, y);
          hover_pos = Pos::new(
            (view_coords.0 + x as f64 / zoom as f64).round() as i32,
            0,
            (view_coords.1 + y as f64 / zoom as f64).round() as i32,
          );

          if let Some((i_x, i_y)) = drag_pos {
            let d_x = (i_x - x) as f64 / zoom as f64;
            let d_y = (i_y - y) as f64 / zoom as f64;

            view_coords.0 += d_x;
            view_coords.1 += d_y;

            drag_pos = Some((x, y));

            spline_view.pan(d_x, d_y);
          }
        }

        _ => {}
      }
    }

    render.clear();

    let screen_width = render.canvas.output_size().unwrap().0;
    let screen_height = render.canvas.output_size().unwrap().1;

    let view_pos = Pos::new(view_coords.0.floor() as i32, 0, view_coords.1.floor() as i32);
    let max_pos =
      view_pos + Pos::new(screen_width as i32 / zoom as i32, 0, screen_height as i32 / zoom as i32);

    // -1 to +1 to make sure we render all chunks that are partially in view.
    // We add an extra 1 chunk outside of that to make panning smoother.
    let min_chunk = view_pos.chunk() + ChunkPos::new(-2, -2);
    let max_chunk = max_pos.chunk() + ChunkPos::new(2, 2);

    {
      let t = Instant::now();
      let generated_chunks = world.read();
      let rendered_chunks = world_view.read_chunks();

      // Loop in a spiral to generate the middle first.
      let middle_chunk = (min_chunk + max_chunk) / 2;

      let half_screen = middle_chunk - min_chunk;
      let radius = half_screen.x.max(half_screen.z);

      'chunk_building: for i in 0..radius {
        let min_circle = middle_chunk - ChunkPos::new(i, i);
        let max_circle = middle_chunk + ChunkPos::new(i, i);

        for x in min_circle.x..=max_circle.x {
          for z in min_circle.z..=max_circle.z {
            let chunk_pos = ChunkPos::new(x, z);

            if chunk_pos.x < min_chunk.x
              || chunk_pos.x > max_chunk.x
              || chunk_pos.z < min_chunk.z
              || chunk_pos.z > max_chunk.z
            {
              continue;
            }

            // Only place chunks for 16ms.
            if t.elapsed().as_millis() > 16 {
              break 'chunk_building;
            }

            match (generated_chunks.has_chunk(chunk_pos), rendered_chunks.get(&chunk_pos)) {
              (true, Some(_)) => continue,
              (true, None) => world_view.request_render(&generated_chunks, chunk_pos),
              (false, _) => world.request(chunk_pos),
            }
          }
        }
      }

      for chunk_x in min_chunk.x..=max_chunk.x {
        for chunk_z in min_chunk.z..=max_chunk.z {
          let chunk_pos = ChunkPos::new(chunk_x, chunk_z);

          let tex = match texture_cache.get(&chunk_pos) {
            Some(t) => t,
            None => {
              if let Some(c) = rendered_chunks.get(&chunk_pos) {
                let mut tex = texture_creator
                  .create_texture_streaming(Some(sdl2::pixels::PixelFormatEnum::ARGB8888), 16, 16)
                  .unwrap();

                c.copy_to_sdl2(&mut tex);

                texture_cache.insert(chunk_pos, tex);
                texture_cache.get(&chunk_pos).unwrap()
              } else {
                continue;
              }
            }
          };

          let pos = chunk_pos.min_block_pos();
          render.canvas.copy(
            &tex,
            None,
            Some(Rect::new(
              pos.x * zoom as i32 - (view_coords.0 * zoom as f64) as i32,
              pos.z * zoom as i32 - (view_coords.1 * zoom as f64) as i32,
              zoom * 16,
              zoom * 16,
            )),
          )?;
        }
      }

      let meter_height = generated_chunks.height_at(hover_pos);

      if let Some(f) = &font {
        let mut f = FontRender { font: f, render: &mut render };

        f.render(0, 0, format!("X: {x:0.2} Z: {z:0.2}", x = hover_pos.x, z = hover_pos.z));
        f.render(0, 24, format!("Height: {meter_height:0.2}"));

        let biome = generated_chunks.column_at(hover_pos).biome;
        f.render(0, 48, format!("Biome: {}", world.context.biomes.name_of(biome)));
      }

      render.canvas.set_draw_color(Color::RGB(0, 0, 255));
      render.canvas.draw_rect(Rect::new(
        hover_pos.x() * zoom as i32 - (view_coords.0 * zoom as f64) as i32,
        hover_pos.z() * zoom as i32 - (view_coords.1 * zoom as f64) as i32,
        zoom,
        zoom,
      ))?;
    }

    spline_view.render(&mut render);

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

fn spawn_generation_thread(world: &Arc<World<TerrainGenerator>>, view: &Arc<WorldViewer>) {
  // Spawn up 16 threads to generate chunks.
  const POOL_SIZE: usize = 16;

  // Generation threads
  for _ in 0..POOL_SIZE {
    let rx = world.request_rx.clone();
    let world = world.clone();

    std::thread::spawn(move || loop {
      let chunk_pos = match rx.recv() {
        Ok(p) => p,
        Err(_) => break,
      };

      world.build_chunk(chunk_pos);
    });
  }

  // Rendering threads
  for _ in 0..POOL_SIZE {
    let rx = view.render_rx.clone();
    let world = world.clone();
    let view = view.clone();

    std::thread::spawn(move || loop {
      let chunk_pos = match rx.recv() {
        Ok(p) => p,
        Err(_) => break,
      };

      view.render_chunk(&world.context, &world.read(), chunk_pos);
    });
  }
}
