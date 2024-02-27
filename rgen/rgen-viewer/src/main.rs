use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

pub fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  let window = video_subsystem
    .window("RGen Viewer", 1920, 1080)
    .position_centered()
    .build()
    .map_err(|e| e.to_string())?;

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

    for rect in &rects {
      let color = Color::RGB(rect.x() as u8, rect.y() as u8, 255);
      canvas.set_draw_color(color);
      canvas.fill_rect(rect.clone())?;
    }

    canvas.present();
  }

  Ok(())
}
