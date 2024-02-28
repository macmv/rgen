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
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem
    .window("RGen Viewer", 1920, 1080)
    .position_centered()
    .build()
    .map_err(|e| e.to_string())?;

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

  let context = Context::new_test(seed);
  let terrain = TerrainGenerator::new(&context.blocks, &context.biomes, context.seed);
  let world = World::new(context, terrain);

  let mut events = sdl_context.event_pump()?;

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas.clear();
  canvas.present();

  let mut last_x = 0;
  let mut last_y = 0;

  let mut mode = RenderMode::Height;
  let mut hover_pos = Pos::new(0, 0, 0);

  let screen_width = 1920;
  let screen_height = 1080;

  let mut grid = RenderGrid::new(screen_width, screen_height, 4);

  let creator = canvas.texture_creator();
  let mut screen_texture = creator
    .create_texture_streaming(
      Some(sdl2::pixels::PixelFormatEnum::ARGB8888),
      screen_width as u32,
      screen_height as u32,
    )
    .unwrap();

  'main: loop {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for event in events.poll_iter() {
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
            let height = world.height(pos);
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

            let biome_hex = match Biome::from_raw_id(biome_id.into()) {
              b if b == world.context.biomes.cold_taiga => 0xffff00,
              b if b == world.context.biomes.extreme_hills => 0xff0000,
              b if b == world.context.biomes.ice_plains => 0x0000ff,
              b if b == world.context.biomes.plains => 0x00ff00,
              b if b == world.context.biomes.savanna => 0xffff00,
              b => {
                println!("unknown biome {b:?}");
                0x000000
              }
            };
            let biome_color = Color::RGB(
              (biome_hex >> 16) as u8 / 8,
              (biome_hex >> 8) as u8 / 8,
              biome_hex as u8 / 8,
            );
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
    canvas.copy(&screen_texture, None, None)?;

    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.draw_rect(Rect::new(hover_pos.x() * 4, hover_pos.z() * 4, 4, 4))?;

    canvas.present();
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
}
