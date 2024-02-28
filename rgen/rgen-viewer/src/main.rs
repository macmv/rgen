use rgen_base::{Biome, ChunkPos, Pos};
use rgen_placer::noise::NoiseGenerator;
use rgen_world::Context;
use sdl2::{
  event::Event,
  keyboard::Keycode,
  pixels::{Color, PixelFormat, PixelFormatEnum},
  rect::Rect,
};

mod terrain;
mod world;

use terrain::TerrainGenerator;
use world::World;

pub fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem
    .window("RGen Viewer", 1920, 1080)
    .position_centered()
    .build()
    .map_err(|e| e.to_string())?;

  let context = Context::new_test(1234);
  let terrain = TerrainGenerator::new(&context.blocks, &context.biomes, context.seed);
  let mut world = World::new(context, terrain);

  let mut events = sdl_context.event_pump()?;

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

  canvas.set_draw_color(Color::RGB(0, 0, 0));
  canvas.clear();
  canvas.present();

  let mut last_x = 0;
  let mut last_y = 0;

  let mut rects = vec![];

  'main: loop {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for event in events.poll_iter() {
      match event {
        Event::Quit { .. } => break 'main,

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

        _ => {}
      }
    }

    for chunk_x in 0..4 {
      for chunk_z in 0..8 {
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

            let color = match Biome::from_raw_id(biome_id.into()) {
              b if b == world.context.biomes.cold_taiga => 0xffff00,
              b if b == world.context.biomes.extreme_hills => 0xff0000,
              b if b == world.context.biomes.ice_plains => 0x0000ff,
              b if b == world.context.biomes.plains => 0x00ff00,
              b => {
                println!("unknown biome {b:?}");
                0x000000
              }
            };

            let color = Color::RGB(
              ((color >> 16) as f64 * height) as u8,
              ((color >> 8) as f64 * height) as u8,
              (color as f64 * height) as u8,
            );
            canvas.set_draw_color(color);
            canvas.fill_rect(Rect::new(pos.x * 10, pos.z * 10, 10, 10))?;
          }
        }
      }
    }

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
