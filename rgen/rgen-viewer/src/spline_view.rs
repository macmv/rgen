use sdl2::rect::Rect;
use splines::{Key, Spline};

pub struct SplineViewer {
  spline: Spline<f64, f64>,
}

impl SplineViewer {
  pub fn new() -> Self {
    SplineViewer {
      spline: Spline::from_vec(vec![
        Key::new(0.0, 1.0, splines::Interpolation::Bezier(0.0)),
        Key::new(0.5, 1.1, splines::Interpolation::Bezier(1.0)),
        Key::new(1.0, 2.0, splines::Interpolation::Bezier(0.0)),
      ]),
    }
  }

  pub fn pan(&mut self, dx: f64, dy: f64) { *self.spline.get_mut(1).unwrap().value += dy; }

  pub fn render(&self, render: &mut crate::Render) {
    for x in 0..100 {
      let y = self.spline.sample(x as f64 / 100.0).unwrap();

      render.canvas.draw_rect(Rect::new(x * 10, (y * 100.0) as i32, 10, 10)).unwrap();
    }
  }
}
