use eframe::egui::{self, Slider};
use egui_plot::{Line, Plot, PlotPoints, Points};
use splines::{Key, Spline};

fn main() -> Result<(), eframe::Error> {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
    ..Default::default()
  };
  eframe::run_native("Spline Editor", options, Box::new(|cc| Box::<SplineEditor>::default()))
}

struct SplineEditor {
  spline: Spline<f64, f64>,
}

impl Default for SplineEditor {
  fn default() -> Self {
    Self {
      spline: Spline::from_vec(vec![
        Key::new(0.0, 0.0, splines::Interpolation::Cosine),
        Key::new(1.0, 120.0, splines::Interpolation::Cosine),
      ]),
    }
  }
}

impl eframe::App for SplineEditor {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Spline Editor");

      for i in 0..self.spline.keys().len() {
        ui.horizontal(|ui| {
          let t = self.spline.keys()[i].t;
          ui.add(
            Slider::from_get_set(0.0..=1.0, |v| {
              if i == 0 || i == self.spline.keys().len() - 1 {
                return t;
              }

              if let Some(new_t) = v {
                let next_t = self.spline.keys()[i + 1].t;
                if new_t >= next_t {
                  return self
                    .spline
                    .replace(i, |k| {
                      let mut k = k.clone();
                      // splines uses unstable sorting, so this key must always be less than the
                      // next key.
                      k.t = next_t - 1e-6;
                      k
                    })
                    .unwrap()
                    .t;
                }

                let prev_t = self.spline.keys()[i - 1].t;
                if new_t < prev_t {
                  return self
                    .spline
                    .replace(i, |k| {
                      let mut k = k.clone();
                      k.t = prev_t;
                      k
                    })
                    .unwrap()
                    .t;
                }

                self
                  .spline
                  .replace(i, |k| {
                    let mut k = k.clone();
                    k.t = new_t;
                    k
                  })
                  .unwrap()
                  .t
              } else {
                t
              }
            })
            .text("y"),
          );

          let v = self.spline.get_mut(i).unwrap();
          ui.add(Slider::new(v.value, 0.0..=128.0).text("y"));
        });
      }

      if ui.button("Add Key").clicked() {
        let mult = 1.0 - 1.0 / self.spline.keys().len() as f64;
        for i in 0..self.spline.keys().len() {
          self.spline.replace(i, |k| {
            let mut k = k.clone();
            k.t *= mult;
            k
          });
        }

        self.spline.add(Key::new(1.0, 64.0, splines::Interpolation::Cosine));
      }

      let spline: PlotPoints = (0..1000)
        .map(|i| {
          let x = i as f64 / 1000.0;
          let y = self.spline.sample(x).unwrap();
          [x, y]
        })
        .collect();
      let line = Line::new(spline);

      let points =
        Points::new(self.spline.keys().iter().map(|k| [k.t, k.value]).collect::<Vec<_>>())
          .radius(5.0);

      Plot::new("spline")
        .include_x(0.0)
        .include_x(1.0)
        .include_y(0.0)
        .include_y(128.0)
        .view_aspect(2.0)
        .show(ui, |plot_ui| {
          plot_ui.line(line);
          plot_ui.points(points);
        });
    });
  }
}
