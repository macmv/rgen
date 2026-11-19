use std::{collections::HashMap, sync::Arc, time::Instant};

use rgen_base::{ChunkPos, Pos};
use rgen_biome::WorldBiomes;
use rgen_world::Context;
use sdl2::{
  event::Event,
  keyboard::Keycode,
  pixels::Color,
  rect::{Point, Rect},
  render::Texture,
};

mod color;
mod queue;
mod region;
mod render;
mod view;
mod world;
mod vectors;

use world::World;

use crate::{
  queue::RenderQueue,
  region::{REGION_SIZE, RegionPos},
  render::{FontRender, Render},
  view::{WorldViewer, climate_color, climate_legend, geography_color, geography_legend},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RenderMode {
  /// Renders biome colors.
  Biomes,

  /// Renders the continent land/water mask.
  LandWater,
  BroadElevation,
  Drainage,
  Runoff,
  Channels,
  WatershedChannels,

  /// Renders the result of `sample_height`.
  Height,

  /// Funny thing for testing.
  Voronoi,

  /// Geographic category used for surface biome selection.
  Geography,

  /// Climate category used for surface biome selection.
  Climate,
}

// Lower bound, stroke color, and legend label. Width grows by one pixel per band.
const RUNOFF_BANDS: &[(u64, u32, &str)] = &[
  (1, 0x497975, "1–3"),
  (4, 0x58aaae, "4–15"),
  (16, 0x67d3eb, "16–63"),
  (64, 0xc6f7ff, "64–255"),
  (256, 0xffe553, "256–1,023"),
  (1024, 0xff9a37, "1,024–4,095"),
  (4096, 0xff5151, "4,096–16,383"),
  (16384, 0xf57bd0, "16,384–65,535"),
  (65536, 0xc7a1ff, "65,536+"),
];

impl RenderMode {
  fn is_channels(self) -> bool { matches!(self, Self::Channels | Self::WatershedChannels) }
}

const MIN_ZOOM: f64 = 0.125;
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
  let terrain = WorldBiomes::new(&context.blocks, context.seed);
  let world = Arc::new(World::new(terrain));
  let world_view = Arc::new(WorldViewer::new());

  let queue = Arc::new(RenderQueue::new());
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

  let mut show_points = false;
  let point_colors = [
    Color::RGB(70, 211, 255), Color::RGB(81, 241, 155), Color::RGB(244, 223, 70),
    Color::RGB(255, 155, 49), Color::RGB(242, 78, 49), Color::RGB(155, 36, 112),
  ];
  let mut sample_points = point_colors.map(|_| Vec::new());
  let mut point_view = None;
  let thresholds = [1u64, 16, 64];
  let mut channel_mode = RenderMode::Channels;
  let mut threshold_index = 1;
  let mut runoff_tiles = HashMap::new();
  let mut zoom = 0.5_f64;
  // The top-left corner of the screen, in fractional blocks.
  let mut view_coords = (0.0, 0.0);
  let mut drag_pos = None;

  let texture_creator = render.canvas.texture_creator();
  let mut texture_cache = HashMap::<RegionPos, Texture>::new();

  let mut last_frame = Instant::now();

  let mut settings = Settings { chunk_borders: false, show_key: false, vectors: false };
  let mut vectors = vectors::VectorOverlay::new(seed);

  'main: loop {
    let mut redraw = false;

    for event in render.events.poll_iter() {
      match event {
        Event::Quit { .. } => break 'main,

        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,

        Event::KeyDown { keycode: Some(Keycode::B), .. } => {
          world_view.set_mode(RenderMode::Biomes);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
          world_view.set_mode(RenderMode::LandWater);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
          world_view.set_mode(RenderMode::BroadElevation);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
          world_view.set_mode(RenderMode::Drainage);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
          world_view.set_mode(RenderMode::Runoff);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::F), repeat: false, .. } if world_view.mode.lock().is_channels() => {
          threshold_index = (threshold_index + 1) % thresholds.len();
        }
        Event::KeyDown { keycode: Some(Keycode::Num5), .. } => {
          world_view.set_mode(channel_mode);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::H), .. } => {
          world_view.set_mode(RenderMode::Height);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::Num6), .. } => {
          world_view.set_mode(RenderMode::Height);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::Num9), .. } => {
          world_view.set_mode(RenderMode::Voronoi);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::Num7), .. } => {
          world_view.set_mode(RenderMode::Geography);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::Num8), .. } => {
          world_view.set_mode(RenderMode::Climate);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::V), repeat: false, .. } => {
          settings.vectors = !settings.vectors;
          println!("Drainage vectors: {}", if settings.vectors { "on" } else { "off" });
        }
        Event::KeyDown { keycode: Some(Keycode::P), repeat: false, .. } => show_points = !show_points,
        Event::KeyDown { keycode: Some(Keycode::K | Keycode::I), repeat: false, .. } => settings.show_key = !settings.show_key,

        Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } if world_view.mode.lock().is_channels() => {
          channel_mode = if channel_mode == RenderMode::Channels { RenderMode::WatershedChannels } else { RenderMode::Channels };
          world_view.set_mode(channel_mode);
          redraw = true;
        }
        Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
          let enabled = world_view.toggle_water_hatch();
          println!("Water hatch: {}", if enabled { "on" } else { "off" });
          redraw = true;
        }

        Event::KeyDown { keycode: Some(Keycode::G), .. } => {
          settings.chunk_borders = !settings.chunk_borders;
        }

        Event::MouseButtonDown { x, y, .. } => drag_pos = Some((x, y)),
        Event::MouseButtonUp { .. } => drag_pos = None,

        Event::MouseWheel { y, precise_y, direction, .. } => {
          let mut scroll = if precise_y == 0.0 { y as f64 } else { precise_y as f64 };
          if direction == sdl2::mouse::MouseWheelDirection::Flipped {
            scroll = -scroll;
          }
          let zoom_after = (zoom * 2.0_f64.powf(scroll)).clamp(MIN_ZOOM, MAX_ZOOM);

          let mouse_block_x = view_coords.0 + mouse_pos.0 as f64 / zoom;
          let mouse_block_y = view_coords.1 + mouse_pos.1 as f64 / zoom;

          view_coords.0 = -(mouse_pos.0 as f64 / zoom_after) + mouse_block_x;
          view_coords.1 = -(mouse_pos.1 as f64 / zoom_after) + mouse_block_y;

          zoom = zoom_after;
        }

        Event::MouseMotion { x, y, .. } => {
          mouse_pos = (x, y);
          hover_pos = Pos::new(
            (view_coords.0 + x as f64 / zoom) as i32,
            255, // Top of the world, to pick the surface biome.
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

    if redraw {
      texture_cache.clear();
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

    {
      let rendered_chunks = world_view.read_chunks();

      queue.update(&rendered_chunks, |state| {
        state.min_chunk = min_chunk;
        state.max_chunk = max_chunk;
        state.center = (min_chunk + max_chunk) / 2;

        let half_screen = state.center - min_chunk;
        state.radius = half_screen.x.max(half_screen.z);

        redraw
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
                    c.dimensions().0,
                    c.dimensions().1,
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
            tex,
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

      if matches!(*world_view.mode.lock(), RenderMode::Channels | RenderMode::WatershedChannels | RenderMode::Runoff) {
        let step = rgen_biome::ELEVATION_SPACING;
        let span = 8 * step;
        let left = (view_coords.0.floor() as i32 - step).div_euclid(span);
        let top = (view_coords.1.floor() as i32 - step).div_euclid(span);
        let right = ((view_coords.0 + screen_width as f64 / zoom) as i32 + step).div_euclid(span);
        let bottom = ((view_coords.1 + screen_height as f64 / zoom) as i32 + step).div_euclid(span);
        let minimum = if world_view.mode.lock().is_channels() { thresholds[threshold_index] } else { 1 };
        runoff_tiles.retain(|&(x,z),_| (left..=right).contains(&x) && (top..=bottom).contains(&z));
        let project = |x:i32,z:i32| Point::new(((x as f64-view_coords.0)*zoom).round() as i32,
          ((z as f64-view_coords.1)*zoom).round() as i32);
        for tz in top..=bottom {
          for tx in left..=right {
            let edges = runoff_tiles.entry((tx,tz)).or_insert_with(|| {
              let mut edges = Vec::new();
              for gz in 0..8 {
                for gx in 0..8 {
                  let x = tx*span+gx*step;
                  let z = tz*span+gz*step;
                  let node = Pos::new(x,0,z);
                  if !world.generator.is_continent_land(node) { continue; }
                  let (dx,dz) = world.generator.drainage(node).direction;
                  if (dx,dz) == (0,0) { continue; }
                  edges.push((x,z,x+i32::from(dx)*step,z+i32::from(dz)*step,world.generator.runoff(node)));
                }
              }
              edges
            });
            for &(x,z,ex,ez,count) in edges.iter() {
              if count < minimum { continue; }
              let band = RUNOFF_BANDS.partition_point(|&(min, _, _)| min <= count).saturating_sub(1);
              let color = RUNOFF_BANDS[band].1;
              let width = band as i32 + 1;
              let a = project(x,z);
              let b = project(ex,ez);
              render.canvas.set_draw_color(Color::RGB((color>>16) as u8,(color>>8) as u8,color as u8));
              for i in 0..width {
                let shift = i-width/2;
                let offset = if (ex-x).abs()>=(ez-z).abs() {Point::new(0,shift)} else {Point::new(shift,0)};
                render.canvas.draw_line(a+offset,b+offset)?;
              }
            }
          }
        }
      }
      if settings.vectors {
        vectors.draw(&mut render, view_coords, zoom, screen_width, screen_height)?;
      }

      if show_points && *world_view.mode.lock() == RenderMode::BroadElevation {
        let spacing = f64::from(rgen_biome::ELEVATION_SPACING);
        let viewport = (view_coords, zoom, screen_width, screen_height);
        if point_view != Some(viewport) {
          for batch in &mut sample_points { batch.clear(); }
          let first_x = (view_coords.0 / spacing).ceil() as i32;
          let first_z = (view_coords.1 / spacing).ceil() as i32;
          let end_x = ((view_coords.0 + f64::from(screen_width) / zoom) / spacing).ceil() as i32;
          let end_z = ((view_coords.1 + f64::from(screen_height) / zoom) / spacing).ceil() as i32;
          for gz in first_z..end_z {
            for gx in first_x..end_x {
              let pos = Pos::new(gx * rgen_biome::ELEVATION_SPACING, 0, gz * rgen_biome::ELEVATION_SPACING);
              let height = world.generator.broad_elevation(pos);
              let band = [64.0, 80.0, 96.0, 112.0, 128.0].partition_point(|&edge| height >= edge);
              let x = ((f64::from(pos.x) - view_coords.0) * zoom).round() as i32;
              let y = ((f64::from(pos.z) - view_coords.1) * zoom).round() as i32;
              sample_points[band].push(Rect::new(x - 2, y - 2, 5, 5));
            }
          }
          point_view = Some(viewport);
        }
        for (color, batch) in point_colors.iter().zip(&sample_points) {
          if !batch.is_empty() {
            render.canvas.set_draw_color(*color);
            render.canvas.fill_rects(batch)?;
          }
        }
      }

      if matches!(*world_view.mode.lock(),RenderMode::Drainage | RenderMode::Runoff) {
        let d = world.generator.drainage(hover_pos);
        let step = rgen_biome::ELEVATION_SPACING;
        let x = (hover_pos.x + step/2).div_euclid(step)*step;
        let z = (hover_pos.z + step/2).div_euclid(step)*step;
        let screen = |x:i32,z:i32| Point::new(((x as f64-view_coords.0)*zoom) as i32,
          ((z as f64-view_coords.1)*zoom) as i32);
        let end = screen(x+step*i32::from(d.direction.0),z+step*i32::from(d.direction.1));
        render.canvas.set_draw_color(Color::WHITE);
        render.canvas.draw_line(screen(x,z),end)?;
        render.canvas.fill_rect(Rect::new(end.x-2,end.y-2,5,5))?;
      }
      let meter_height = world.height_at(hover_pos);

      if let Some(f) = &font {
        let mut f = FontRender { font: f, render: &mut render };

        f.render(0, 0, format!("X: {x:0.2} Z: {z:0.2}", x = hover_pos.x, z = hover_pos.z));
        f.render(0, 24, format!("Height: {meter_height:0.2}"));

        let biome = world.generator.choose_biome(hover_pos);
        f.render(0, 48, format!("Biome: {}", biome.name));

        let surface = if world.generator.is_continent_land(hover_pos) { "Land" } else { "Ocean" };
        let elevation = world.generator.broad_elevation(hover_pos);


        f.render(0, 72, format!("Continent mask: {surface}"));
        f.render(0, 96, format!("Broad elevation: Y {:.1}", elevation));
        if world_view.mode.lock().is_channels() {
          f.render(0,144,format!("5  Channel preview   F: minimum runoff {}+   W: watersheds {}", thresholds[threshold_index], if channel_mode == RenderMode::WatershedChannels { "on" } else { "off" }));
        }
        if matches!(*world_view.mode.lock(), RenderMode::Channels | RenderMode::WatershedChannels | RenderMode::Runoff) {
          let count = world.generator.runoff(hover_pos);
          f.render(0,120,format!("Runoff: {count} units   Upstream area: {} blocks²",count*4096));
        }
        if *world_view.mode.lock() == RenderMode::Drainage {
          let d = world.generator.drainage(hover_pos);
          f.render(0, 120, format!("Downstream step {:?}   Outlet {:?}   Spill Y {:.1}", d.direction, d.outlet, d.spill_y));
        }
      }

      if settings.show_key {
        let mode = *world_view.mode.lock();
        if let Some(font) = &font {
          match mode {
            RenderMode::Geography => draw_key(
              &mut render,
              font,
              screen_width,
              "Geography",
              geography_legend()
                .into_iter()
                .map(|(category, label)| (geography_color(category), label))
                .collect(),
            )?,
            RenderMode::Climate => draw_key(
              &mut render,
              font,
              screen_width,
              "Climate",
              climate_legend()
                .into_iter()
                .map(|(category, label)| (climate_color(category), label))
                .collect(),
            )?,
            RenderMode::Channels | RenderMode::WatershedChannels | RenderMode::Runoff => draw_key(
              &mut render, font, screen_width, "Upstream cells (64×64 blocks)",
              RUNOFF_BANDS.iter().map(|&(_, color, label)| (crate::color::Color::from_hex(color), label)).collect(),
            )?,
            other => {
              let (title, rows): (_, &[(u32, &str)]) = match other {
                RenderMode::Biomes => ("B  Biomes", &[(0x70a65b,"Biome surface"),(0x009dc4,"Water")]),
                RenderMode::LandWater => ("1  Continents", &[(0x70a65b,"Land"),(0x4263a8,"Ocean")]),
                RenderMode::BroadElevation => ("2  Elevation (P: points)", &[(0x46d3ff,"Ocean"),(0x51f19b,"Y 64-79"),(0xf4df46,"Y 80-95"),(0xff9b31,"Y 96-111"),(0xf24e31,"Y 112-127"),(0x9b2470,"Y 128+")]),
                RenderMode::Drainage => ("3  Drainage", &[(0x4263a8,"Ocean"),(0xbfa764,"Colors group outlets"),(0xffffff,"Dot: downstream")]),

                RenderMode::Height => ("6  Carved terrain", &[(0,"Low"),(0xffffff,"High")]),
                RenderMode::Voronoi => ("9  Voronoi", &[(0xff00ff,"Colors identify cells")]),
                _ => unreachable!(),
              };
              draw_key(&mut render,font,screen_width,title,rows.iter().map(|&(c,l)| (crate::color::Color::from_hex(c),l)).collect())?;
            }
          }
        }
      }

      render.canvas.set_draw_color(Color::RGB(0, 0, 255));
      render.canvas.draw_rect(Rect::new(
        hover_pos.x() * zoom as i32 - (view_coords.0 * zoom) as i32,
        hover_pos.z() * zoom as i32 - (view_coords.1 * zoom) as i32,
        zoom as u32,
        zoom as u32,
      ))?;
    }

    if settings.chunk_borders {
      let min_chunk = view_pos.chunk();
      let max_chunk = max_pos.chunk();

      let min_pos = min_chunk.min_block_pos();
      let max_pos = max_chunk.min_block_pos() + Pos::new(15, 0, 15);

      render.canvas.set_draw_color(Color::RGB(255, 255, 0));
      for x in min_chunk.x..=max_chunk.x {
        let pos = ChunkPos::new(x, 0).min_block_pos();

        render.canvas.draw_line(
          Point::new(
            (pos.x as f64 * zoom) as i32 - (view_coords.0 * zoom) as i32,
            (min_pos.z as f64 * zoom) as i32 - (view_coords.1 * zoom) as i32,
          ),
          Point::new(
            (pos.x as f64 * zoom) as i32 - (view_coords.0 * zoom) as i32,
            (max_pos.z as f64 * zoom) as i32 - (view_coords.1 * zoom) as i32,
          ),
        )?;
      }

      for z in min_chunk.z..=max_chunk.z {
        let pos = ChunkPos::new(0, z).min_block_pos();

        render.canvas.draw_line(
          Point::new(
            (min_pos.x as f64 * zoom) as i32 - (view_coords.0 * zoom) as i32,
            (pos.z as f64 * zoom) as i32 - (view_coords.1 * zoom) as i32,
          ),
          Point::new(
            (max_pos.x as f64 * zoom) as i32 - (view_coords.0 * zoom) as i32,
            (pos.z as f64 * zoom) as i32 - (view_coords.1 * zoom) as i32,
          ),
        )?;
      }
    }

    if settings.vectors {
      if let Some(font) = &font {
        FontRender { font, render: &mut render }.render(8, screen_height as i32 - 30,
          "V: vectors | cyan: river | orange: valley | arrows: downstream");
      }
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

struct Settings {
  chunk_borders: bool,
  vectors: bool,
  show_key:      bool,
}

fn draw_key(
  render: &mut Render,
  font: &sdl2::ttf::Font<'_, '_>,
  screen_width: u32,
  title: &str,
  entries: Vec<(crate::color::Color, &'static str)>,
) -> Result<(), String> {
  let panel_width = 230.max(font.size_of(title).map_or(0, |(w, _)| w as i32) + 24);
  let row_height = 22;
  let panel_height = 40 + entries.len() as i32 * row_height;
  let left = screen_width as i32 - panel_width - 12;
  let top = 12;

  render.canvas.set_draw_color(Color::RGB(24, 24, 32));
  render.canvas.fill_rect(Rect::new(left, top, panel_width as u32, panel_height as u32))?;
  FontRender { font, render }.render(left + 10, top + 7, title);

  for (index, (swatch, label)) in entries.into_iter().enumerate() {
    let y = top + 34 + index as i32 * row_height;
    render.canvas.set_draw_color(Color::RGB(
      (swatch.r * 255.0) as u8,
      (swatch.g * 255.0) as u8,
      (swatch.b * 255.0) as u8,
    ));
    render.canvas.fill_rect(Rect::new(left + 10, y + 3, 16, 16))?;
    FontRender { font, render }.render(left + 34, y, label);
  }
  Ok(())
}
