use sdl2::rect::Rect;
use splines::{Key, Spline};

pub struct SplineViewer {
  spline: Spline<f64, f64>,
}

impl SplineViewer {
  pub fn new() -> Self {
    SplineViewer {
      spline: Spline::from_vec(vec![
        Key::new(0.0, 120.0, splines::Interpolation::Cosine),
        Key::new(0.1, 40.0, splines::Interpolation::Cosine),
        Key::new(0.3, 40.0, splines::Interpolation::Cosine),
        Key::new(0.4, 70.0, splines::Interpolation::Cosine),
        Key::new(0.5, 80.0, splines::Interpolation::Cosine),
        Key::new(0.8, 140.0, splines::Interpolation::Cosine),
        Key::new(1.0, 150.0, splines::Interpolation::Cosine),
      ]),
    }
  }

  pub fn pan(&mut self, _dx: f64, dy: f64) { *self.spline.get_mut(1).unwrap().value += dy; }

  pub fn render(&self, render: &mut crate::Render) {
    for x in 0..100 {
      let y = self.spline.sample(x as f64 / 100.0).unwrap();

      render.canvas.draw_rect(Rect::new(x * 10, (200.0 - y) as i32, 10, 10)).unwrap();
    }
  }
}
