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
  // This spline maps noise values to Y heights.
  continentalness_spline: Spline<Vec<(f64, f64)>>,

  // These two splines map noise values to multipliers away from sea level.
  erosion_spline:       Spline<Vec<(f64, f64)>>,
  peaks_valleys_spline: Spline<Vec<(f64, f64)>>,
  height_impact_spline: Spline<Vec<(f64, f64)>>,

  erosion:         f64,
  continentalness: f64,
  peaks_valleys:   f64,
}

impl Default for SplineEditor {
  fn default() -> Self {
    Self {
      continentalness_spline: Spline::new(vec![
        (0.00, 88.0),
        (0.01, 35.0),
        (0.15, 38.0),
        (0.26, 52.0),
        (0.40, 65.0),
        (0.81, 85.0),
        (0.91, 103.0),
        (1.00, 128.0),
      ]),
      erosion_spline:         Spline::new(vec![
        (0.00, 1.0),
        (0.01, 0.8),
        (0.15, 0.7),
        (0.26, 0.5),
        (0.40, 0.3),
        (0.81, 0.2),
        (0.91, 0.1),
        (1.00, 0.0),
      ]),
      height_impact_spline:   Spline::new(vec![
        (0.00, 1.0),
        (0.01, 0.0),
        (0.45, 0.0),
        (0.55, 1.0),
        (1.00, 1.0),
      ]),
      peaks_valleys_spline:   Spline::new(vec![
        (0.00, 16.0),
        (0.40, 8.0),
        (0.47, 2.0),
        (0.50, 0.0),
        (0.53, 2.0),
        (0.60, 8.0),
        (1.00, 16.0),
      ]),

      continentalness: 0.0,
      erosion:         0.0,
      peaks_valleys:   0.0,
    }
  }
}

impl eframe::App for SplineEditor {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Spline Editor");

      ui.horizontal(|ui| {
        draw_editor(ui, &mut self.continentalness_spline, 0.0..=128.0);
        ui.separator();
        draw_editor(ui, &mut self.erosion_spline, 0.0..=1.0);
        ui.separator();
        draw_editor(ui, &mut self.peaks_valleys_spline, 0.0..=32.0);
        ui.separator();
        draw_editor(ui, &mut self.height_impact_spline, 0.0..=1.0);
      });

      ui.add(Slider::new(&mut self.continentalness, 0.0..=1.0).text("Continentalness"));
      ui.add(Slider::new(&mut self.erosion, 0.0..=1.0).text("Erosion"));
      ui.add(Slider::new(&mut self.peaks_valleys, 0.0..=1.0).text("Peaks and Valleys"));

      Plot::new("spline")
        .include_x(0.0)
        .include_x(1.0)
        .include_y(0.0)
        .include_y(128.0)
        .view_aspect(2.0)
        .show(ui, |plot_ui| {
          plot_spline(plot_ui, &self.continentalness_spline, 1.0);
          plot_spline(plot_ui, &self.erosion_spline, 128.0);
          plot_spline(plot_ui, &self.peaks_valleys_spline, 4.0);
          plot_spline(plot_ui, &self.height_impact_spline, 128.0);

          plot_sample(plot_ui, |x| {
            let c = self.continentalness_spline.sample::<Cosine>(x);
            let impact = self.height_impact_spline.sample::<Cosine>(c / 128.0);
            let p = self.peaks_valleys_spline.sample::<Cosine>(self.peaks_valleys);
            let e = self.erosion_spline.sample::<Cosine>(self.erosion);

            fn lerp(a: f64, b: f64, t: f64) -> f64 { a * (1.0 - t) + b * t }
            let e = lerp(0.2, e, impact);
            let p = lerp(8.0, p, impact);

            ((((c - 64.0) * 4.0) + 64.0) + p - 64.0) * e + 64.0
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
