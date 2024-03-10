use std::ops::RangeInclusive;

use eframe::egui::{self, Slider};
use egui_plot::{Line, Plot, PlotUi, Points};
use rgen_spline::{Cosine, Spline};

fn main() -> Result<(), eframe::Error> {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
    ..Default::default()
  };
  eframe::run_native("Spline Editor", options, Box::new(|_cc| Box::<SplineEditor>::default()))
}

struct SplineEditor {
  spline:       Spline<Vec<(f64, f64)>>,
  other_spline: Spline<Vec<(f64, f64)>>,
  lerp_spline:  Spline<Vec<(f64, f64)>>,

  lerp: f64,
}

impl Default for SplineEditor {
  fn default() -> Self {
    Self {
      spline:       Spline::new(vec![
        (0.00, 88.0),
        (0.01, 35.0),
        (0.15, 38.0),
        (0.26, 52.0),
        (0.40, 65.0),
        (0.81, 85.0),
        (0.91, 103.0),
        (1.00, 128.0),
      ]),
      other_spline: Spline::new(vec![
        (0.00, 60.0),
        (0.01, 60.0),
        (0.15, 60.0),
        (0.26, 60.0),
        (0.40, 60.0),
        (0.81, 60.0),
        (0.91, 60.0),
        (1.00, 60.0),
      ]),
      lerp_spline:  Spline::new(vec![
        (0.00, 0.0),
        (0.48, 0.0),
        (0.50, 1.0),
        (0.52, 0.0),
        (1.00, 0.0),
      ]),

      lerp: 0.0,
    }
  }
}

impl eframe::App for SplineEditor {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Spline Editor");

      ui.horizontal(|ui| {
        draw_editor(ui, &mut self.spline, 0.0..=128.0);
        ui.separator();
        draw_editor(ui, &mut self.other_spline, 0.0..=128.0);
        ui.separator();
        draw_editor(ui, &mut self.lerp_spline, 0.0..=1.0);
      });

      ui.add(Slider::new(&mut self.lerp, 0.0..=1.0).text("lerp"));

      Plot::new("spline")
        .include_x(0.0)
        .include_x(1.0)
        .include_y(0.0)
        .include_y(128.0)
        .view_aspect(2.0)
        .show(ui, |plot_ui| {
          plot_spline(plot_ui, &self.spline, 1.0);
          plot_spline(plot_ui, &self.other_spline, 1.0);
          plot_spline(plot_ui, &self.lerp_spline, 128.0);
          plot_sample(plot_ui, |x| {
            let a = self.spline.sample::<Cosine>(x);
            let b = self.other_spline.sample::<Cosine>(x);

            a + (b - a) * self.lerp_spline.sample::<Cosine>(self.lerp)
          });
        });
    });
  }
}

fn draw_editor(
  ui: &mut egui::Ui,
  spline: &mut Spline<Vec<(f64, f64)>>,
  range: RangeInclusive<f64>,
) {
  ui.vertical(|ui| {
    for i in 0..spline.storage.len() {
      let range = range.clone();
      ui.horizontal(|ui| {
        let t = spline.storage[i].0;
        ui.add(
          Slider::from_get_set(0.0..=1.0, |v| {
            if i == 0 || i == spline.storage.len() - 1 {
              return t;
            }

            if let Some(new_t) = v {
              let next_t = spline.storage[i + 1].0;
              if new_t >= next_t {
                spline.storage[i].0 = next_t - 1e-6;
                return spline.storage[i].0;
              }

              let prev_t = spline.storage[i - 1].0;
              if new_t < prev_t {
                spline.storage[i].0 = prev_t;
                return spline.storage[i].0;
              }

              spline.storage[i].0 = new_t;
              spline.storage[i].0
            } else {
              t
            }
          })
          .text("y"),
        );

        let v = &mut spline.storage[i];
        ui.add(Slider::new(&mut v.1, range).text("y"));
      });
    }

    if ui.button("Add Key").clicked() {
      let mult = 1.0 - 1.0 / spline.storage.len() as f64;
      for i in 0..spline.storage.len() {
        spline.storage[i].0 *= mult;
      }

      spline.storage.push((1.0, range.end() / 2.0));
    }
  });
}

fn plot_spline(plot_ui: &mut PlotUi, spline: &Spline<Vec<(f64, f64)>>, scale: f64) {
  let line = Line::new(
    (0..1000)
      .map(|i| {
        let x = i as f64 / 1000.0;
        let y = spline.sample::<Cosine>(x);
        [x, y * scale]
      })
      .collect::<Vec<_>>(),
  );

  let points =
    Points::new(spline.storage.iter().map(|k| [k.0, k.1 * scale]).collect::<Vec<_>>()).radius(5.0);

  plot_ui.line(line);
  plot_ui.points(points);
}

fn plot_sample(plot_ui: &mut PlotUi, sample: impl Fn(f64) -> f64) {
  let line = Line::new(
    (0..1000)
      .map(|i| {
        let x = i as f64 / 1000.0;
        let y = sample(x);
        [x, y]
      })
      .collect::<Vec<_>>(),
  );

  plot_ui.line(line);
}
