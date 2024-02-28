use rgen_base::{Biome, ChunkPos, Pos};
use rgen_placer::noise::NoiseGenerator;
use rgen_world::Context;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

mod terrain;
mod world;

use terrain::TerrainGenerator;
use world::World;

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

  let mut rects = vec![];

  let mut mode = RenderMode::Height;
  let mut hover_pos = Pos::new(0, 0, 0);

  'main: loop {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for event in events.poll_iter() {
      match event {
        Event::Quit { .. } => break 'main,

        Event::KeyDown { keycode: Some(Keycode::Num1), .. } => mode = RenderMode::Height,
        Event::KeyDown { keycode: Some(Keycode::Num2), .. } => mode = RenderMode::Slope,
        Event::KeyDown { keycode: Some(Keycode::Num3), .. } => mode = RenderMode::Aspect,
        Event::KeyDown { keycode: Some(Keycode::Num4), .. } => mode = RenderMode::Brightness,
        Event::KeyDown { keycode: Some(Keycode::Num5), .. } => mode = RenderMode::BiomeColors,

        Event::KeyDown { keycode: Some(keycode), .. } => {
          if keycode == Keycode::Escape {
            break 'main;
          } else if keycode == Keycode::Space {
            println!("space down");
            for i in 0..400 {
              canvas.fill_rect(Rect::new(i, i, 100, 100))?;
            }
          }
        }

        Event::MouseButtonDown { x, y, .. } => {
          rects.push(Rect::new(last_x, last_y, x as u32, y as u32));

          last_x = x;
          last_y = y;
          println!("mouse btn down at ({},{})", x, y);
        }

        Event::MouseMotion { x, y, .. } => {
          hover_pos = Pos::new(x / 4, 0, y / 4);
        }

        _ => {}
      }
    }

    for chunk_x in 0..30 {
      for chunk_z in 0..30 {
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

            let greycolor = Color::RGB(brightness, brightness, brightness);

            let color = match Biome::from_raw_id(biome_id.into()) {
              b if b == world.context.biomes.cold_taiga => 0xffff00,
              b if b == world.context.biomes.extreme_hills => 0xff0000,
              b if b == world.context.biomes.ice_plains => 0x0000ff,
              b if b == world.context.biomes.plains => 0x00ff00,
              b => {
                // println!("unknown biome {b:?}");
                0x000000
              }
            };

            let color = Color::RGB(
              ((color >> 16) as f64 * height) as u8,
              ((color >> 8) as f64 * height) as u8,
              (color as f64 * height) as u8,
            );
            canvas.set_draw_color(greycolor);
            canvas.fill_rect(Rect::new(pos.x * 4, pos.z * 4, 4, 4))?;
          }
        }
      }
    }

    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.draw_rect(Rect::new(hover_pos.x() * 4, hover_pos.z() * 4, 4, 4))?;

    for rect in &rects {
      let color = Color::RGB(rect.x() as u8, rect.y() as u8, 255);
      canvas.set_draw_color(color);
      canvas.fill_rect(rect.clone())?;
    }

    canvas.present();
  }

  Ok(())
}

impl World<TerrainGenerator> {
  pub fn height(&self, pos: Pos) -> f64 {
    let height =
      (self.generator.height_map.generate(pos.x as f64, pos.z as f64, self.generator.seed) + 1.0);
    height
  }
  pub fn meter_height(&self, pos: Pos) -> f64 {
    let meter_height = self.height(pos) * 64.0;
    meter_height
  }
}
