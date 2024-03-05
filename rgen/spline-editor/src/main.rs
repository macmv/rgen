use eframe::egui::{self, Slider};
use egui_plot::{Line, Plot, PlotPoints, Points};
use rgen_spline::Spline;

fn main() -> Result<(), eframe::Error> {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
    ..Default::default()
  };
  eframe::run_native("Spline Editor", options, Box::new(|cc| Box::<SplineEditor>::default()))
}

struct SplineEditor {
  spline: Spline<Vec<(f64, f64, f64)>>,
}

impl Default for SplineEditor {
  fn default() -> Self { Self { spline: Spline::new(vec![(0.0, 0.0, 1.0), (1.0, 120.0, 0.0)]) } }
}

impl eframe::App for SplineEditor {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Spline Editor");

      for i in 0..self.spline.storage.len() {
        ui.horizontal(|ui| {
          let t = self.spline.storage[i].0;
          ui.add(
            Slider::from_get_set(0.0..=1.0, |v| {
              if i == 0 || i == self.spline.storage.len() - 1 {
                return t;
              }

              if let Some(new_t) = v {
                let next_t = self.spline.storage[i + 1].0;
                if new_t >= next_t {
                  self.spline.storage[i].0 = next_t - 1e-6;
                  return self.spline.storage[i].0;
                }

                let prev_t = self.spline.storage[i - 1].0;
                if new_t < prev_t {
                  self.spline.storage[i].0 = prev_t;
                  return self.spline.storage[i].0;
                }

                self.spline.storage[i].0 = new_t;
                self.spline.storage[i].0
              } else {
                t
              }
            })
            .text("y"),
          );

          let v = &mut self.spline.storage[i];
          ui.add(Slider::new(&mut v.1, 0.0..=128.0).text("y"));
          ui.add(Slider::new(&mut v.2, -128.0..=128.0).text("k"));
        });
      }

      if ui.button("Add Key").clicked() {
        let mult = 1.0 - 1.0 / self.spline.storage.len() as f64;
        for i in 0..self.spline.storage.len() {
          self.spline.storage[i].0 *= mult;
        }

        self.spline.storage.push((1.0, 64.0, 1.0));
      }

      let spline: PlotPoints = (0..1000)
        .map(|i| {
          let x = i as f64 / 1000.0;
          let y = self.spline.sample_bezier(x);
          [x, y]
        })
        .collect();
      let line = Line::new(spline);

      let points =
        Points::new(self.spline.storage.iter().map(|k| [k.0, k.1]).collect::<Vec<_>>()).radius(5.0);

      Plot::new("spline")
        .include_x(0.0)
        .include_x(1.0)
        .include_y(0.0)
        .include_y(128.0)
        .view_aspect(2.0)
        .show(ui, |plot_ui| {
          plot_ui.line(line);
          plot_ui.points(points);

          let [min_x, min_y] = plot_ui.plot_bounds().min();
          let [max_x, max_y] = plot_ui.plot_bounds().max();

          let aspect = (max_y - min_y) / (max_x - min_x);
          dbg!(aspect);

          for i in 0..self.spline.storage.len() {
            let (x, y, k) = self.spline.storage[i];

            let min = [x - 0.1, y - k];
            let max = [x + 0.1, y + k];

            plot_ui.line(Line::new(PlotPoints::new(vec![min, max])));
          }
        });
    });
  }
}
