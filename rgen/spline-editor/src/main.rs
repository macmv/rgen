use std::{
  ops::RangeInclusive,
  sync::{Arc, Mutex, mpsc},
};

use eframe::egui_glow;
use eframe::egui::{
  self, Align2, Color32, DragValue, FontId, Pos2, Rect, Sense, Stroke, Vec2,
};
use egui_plot::{HLine, Legend, Line, Plot, Points, VLine};
use rgen_biome::{HeightParts, RawNoise, TerrainNoise};
use rgen_spline::{Cosine, Spline};

mod camera;
mod gl;
mod grid;

use camera::Camera;
use grid::{Grid, Layer, SEA_LEVEL};

fn main() -> Result<(), eframe::Error> {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 900.0]),
    ..Default::default()
  };
  eframe::run_native(
    "Terrain Inspector",
    options,
    Box::new(|cc| {
      let gl = cc.gl.as_ref().expect("the terrain view needs OpenGL");
      let renderer = gl::Renderer::new(gl).expect("failed to build the terrain shader");
      Box::new(App::new(Arc::new(Mutex::new(renderer))))
    }),
  )
}

type Keys = Spline<Vec<(f64, f64)>>;

#[derive(Clone)]
pub struct Splines {
  continentalness: Keys,
  erosion:         Keys,
  peaks_valleys:   Keys,
  height_impact:   Keys,
}

impl Splines {
  fn from_game() -> Self {
    Splines {
      continentalness: Spline::new(rgen_biome::CONTINENTALNESS.storage.to_vec()),
      erosion:         Spline::new(rgen_biome::EROSION.storage.to_vec()),
      peaks_valleys:   Spline::new(rgen_biome::PEAKS_VALLEYS.storage.to_vec()),
      height_impact:   Spline::new(rgen_biome::HEIGHT_IMPACT.storage.to_vec()),
    }
  }

  pub fn parts(&self, raw: RawNoise) -> HeightParts {
    rgen_biome::height_parts(
      &self.continentalness,
      &self.erosion,
      &self.peaks_valleys,
      &self.height_impact,
      raw,
    )
  }

  /// Formats the splines as the statics in `rgen-biome/src/lib.rs`.
  fn to_rust(&self) -> String {
    fn fmt(v: f64) -> String {
      let s = format!("{v:.3}");
      let s = s.trim_end_matches('0');
      if s.ends_with('.') { format!("{s}0") } else { s.to_string() }
    }
    fn one(name: &str, spline: &Keys) -> String {
      let mut out =
        format!("  pub static ref {name}: Spline<&'static [(f64, f64)]> = Spline::new(&[\n");
      for (t, v) in &spline.storage {
        out.push_str(&format!("    ({}, {}),\n", fmt(*t), fmt(*v)));
      }
      out.push_str("  ]);\n");
      out
    }

    [
      one("CONTINENTALNESS", &self.continentalness),
      one("EROSION", &self.erosion),
      one("PEAKS_VALLEYS", &self.peaks_valleys),
      one("HEIGHT_IMPACT", &self.height_impact),
    ]
    .join("\n")
  }
}

/// A line across the terrain, sampled directly from the noise (not the grid) so
/// the profile is smooth.
struct Section {
  a:       Option<[f64; 2]>,
  b:       Option<[f64; 2]>,
  /// True while the next click on the terrain sets an endpoint.
  picking: bool,

  samples: Vec<SectionSample>,
}

struct SectionSample {
  /// Distance along the section, in blocks.
  distance: f64,
  x:        f64,
  z:        f64,
  noise:    RawNoise,
}

const SECTION_SAMPLES: usize = 256;

impl Section {
  fn resample(&mut self, noise: &TerrainNoise) {
    self.samples.clear();
    let (Some(a), Some(b)) = (self.a, self.b) else { return };

    let length = ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
    for i in 0..SECTION_SAMPLES {
      let t = i as f64 / (SECTION_SAMPLES - 1) as f64;
      let x = a[0] + (b[0] - a[0]) * t;
      let z = a[1] + (b[1] - a[1]) * t;
      self.samples.push(SectionSample { distance: length * t, x, z, noise: noise.sample(x, z) });
    }
  }
}

struct App {
  splines:       Splines,
  /// The splines as they are in the game, for comparing against.
  saved:         Splines,
  compare_saved: bool,

  seed:   u64,
  center: [f64; 2],
  /// The width of the terrain, in blocks. Changing the block size doesn't
  /// change this, only how finely it's sampled.
  width:  u32,
  /// Blocks along each side of a column.
  step:   u32,

  noise:      Arc<TerrainNoise>,
  requested:  Option<(u64, [i64; 2], u32, u32)>,
  request_id: u64,
  tx:         mpsc::Sender<(u64, Vec<RawNoise>)>,
  rx:         mpsc::Receiver<(u64, Vec<RawNoise>)>,

  grid:  Option<Grid>,
  /// Set when the splines or display options change, so the grid colors need to
  /// be rebuilt.
  dirty: bool,

  layer:         Layer,
  flatten_water: bool,
  camera:        Camera,
  /// The grid vertex under the mouse, from the last frame.
  hover:         Option<usize>,

  renderer: Arc<Mutex<gl::Renderer>>,

  section: Section,
}

impl App {
  fn new(renderer: Arc<Mutex<gl::Renderer>>) -> Self {
    let (tx, rx) = mpsc::channel();
    App {
      splines: Splines::from_game(),
      saved: Splines::from_game(),
      compare_saved: false,

      seed: 0,
      center: [0.0, 0.0],
      width: 2048,
      step: 4,

      noise: Arc::new(TerrainNoise::new(0)),
      requested: None,
      request_id: 0,
      tx,
      rx,

      grid: None,
      dirty: false,

      layer: Layer::Height,
      flatten_water: true,
      camera: Camera::default(),
      hover: None,

      renderer,

      section: Section { a: None, b: None, picking: false, samples: vec![] },
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    while let Ok((id, raw)) = self.rx.try_recv() {
      if id == self.request_id {
        self.grid = Some(Grid::new(raw, self.columns(), self.center, f64::from(self.step)));
        self.dirty = true;
      }
    }
    self.request_grid(ctx);

    egui::SidePanel::left("controls").default_width(340.0).show(ctx, |ui| self.controls(ui));

    if self.dirty {
      let splines = if self.compare_saved { self.saved.clone() } else { self.splines.clone() };
      if let Some(grid) = &mut self.grid {
        grid.recompute(&splines, self.layer, self.flatten_water);

        let colors = grid.colors.iter().flat_map(|c| c.to_array()).collect();
        self.renderer.lock().unwrap().set_terrain(gl::Upload {
          n: grid.n,
          heights: grid.height.clone(),
          colors,
        });
      }
      self.dirty = false;
    }

    egui::TopBottomPanel::bottom("section")
      .resizable(true)
      .default_height(260.0)
      .show(ctx, |ui| self.section_panel(ui));

    egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| self.terrain_view(ui));
  }
}

/// The largest block size.
const MAX_STEP: u32 = 32;
/// The most columns along one side. A bigger grid is too slow to recolor
/// while editing splines.
const MAX_COLUMNS: u32 = 1024;

impl App {
  /// Columns along each side of the terrain.
  fn columns(&self) -> usize { (self.width / self.step) as usize }

  /// The smallest block size that keeps the grid under `MAX_COLUMNS`.
  fn min_step(&self) -> u32 { (self.width / MAX_COLUMNS).max(1) }

  fn active_splines(&self) -> &Splines {
    if self.compare_saved { &self.saved } else { &self.splines }
  }

  /// Starts sampling the noise in the background if the parameters changed.
  fn request_grid(&mut self, ctx: &egui::Context) {
    let key = (self.seed, [self.center[0] as i64, self.center[1] as i64], self.width, self.step);
    if self.requested == Some(key) {
      return;
    }
    self.requested = Some(key);
    self.request_id += 1;

    let id = self.request_id;
    let noise = self.noise.clone();
    let (center, n, step) = (self.center, self.columns(), f64::from(self.step));
    let tx = self.tx.clone();
    let ctx = ctx.clone();

    std::thread::spawn(move || {
      let mut raw = vec![RawNoise { continentalness: 0.0, peaks_valleys: 0.0, erosion: 0.0 }; n * n];

      // Split the rows between all the cores, as a large grid is a lot of noise.
      let threads = std::thread::available_parallelism().map_or(1, |t| t.get());
      let rows_per_thread = n.div_ceil(threads);
      std::thread::scope(|scope| {
        for (chunk_index, chunk) in raw.chunks_mut(rows_per_thread * n).enumerate() {
          let noise = &noise;
          scope.spawn(move || {
            for (k, out) in chunk.iter_mut().enumerate() {
              let idx = chunk_index * rows_per_thread * n + k;
              let (i, j) = (idx / n, idx % n);
              let x = center[0] + (i as f64 - n as f64 / 2.0) * step;
              let z = center[1] + (j as f64 - n as f64 / 2.0) * step;
              *out = noise.sample(x, z);
            }
          });
        }
      });

      let _ = tx.send((id, raw));
      ctx.request_repaint();
    });
  }

  fn controls(&mut self, ui: &mut egui::Ui) {
    ui.heading("Terrain Inspector");

    egui::Grid::new("params").num_columns(2).show(ui, |ui| {
      ui.label("Seed");
      if ui.add(DragValue::new(&mut self.seed)).changed() {
        self.noise = Arc::new(TerrainNoise::new(self.seed));
        self.section.resample(&self.noise);
      }
      ui.end_row();

      ui.label("Center x, z");
      ui.horizontal(|ui| {
        let speed = f64::from(self.step) * 4.0;
        ui.add(DragValue::new(&mut self.center[0]).speed(speed));
        ui.add(DragValue::new(&mut self.center[1]).speed(speed));
      });
      ui.end_row();

      ui.label("Width");
      egui::ComboBox::from_id_source("width")
        .selected_text(format!("{} blocks", self.width))
        .show_ui(ui, |ui| {
          for width in [512, 1024, 2048, 4096, 8192] {
            ui.selectable_value(&mut self.width, width, format!("{width} blocks"));
          }
        });
      ui.end_row();

      // Keep the block size in range when the width changes.
      self.step = self.step.clamp(self.min_step(), MAX_STEP);

      ui.label("Block size");
      egui::ComboBox::from_id_source("step")
        .selected_text(format!("{0}x{0}x{0}", self.step))
        .show_ui(ui, |ui| {
          for step in (0..).map(|i| 1u32 << i).take_while(|&s| s <= MAX_STEP) {
            if step >= self.min_step() {
              ui.selectable_value(&mut self.step, step, format!("{step}x{step}x{step}"));
            }
          }
        });
      ui.end_row();

      ui.label("Columns");
      ui.label(format!("{0}x{0}", self.columns()));
      ui.end_row();
    });

    egui::ComboBox::from_label("Layer").selected_text(self.layer.name()).show_ui(ui, |ui| {
      for layer in Layer::ALL {
        self.dirty |= ui.selectable_value(&mut self.layer, layer, layer.name()).changed();
      }
    });

    self.dirty |= ui.checkbox(&mut self.flatten_water, "Flatten below sea level").changed();
    self.dirty |= ui.checkbox(&mut self.compare_saved, "Show splines saved in the game").changed();

    ui.horizontal(|ui| {
      if ui.button("Copy as Rust").clicked() {
        ui.ctx().copy_text(self.splines.to_rust());
      }
      if ui.button("Reset splines").clicked() {
        self.splines = self.saved.clone();
        self.dirty = true;
      }
    });

    ui.separator();

    // Each spline plot marks where the hovered vertex sits on its curve.
    let hovered = self.hovered();

    egui::ScrollArea::vertical().show(ui, |ui| {
      let mut changed = false;

      changed |= spline_editor(
        ui,
        "Continentalness",
        &mut self.splines.continentalness,
        0.0..=128.0,
        hovered.map(|(r, _)| r.continentalness),
      );
      changed |= spline_editor(
        ui,
        "Erosion",
        &mut self.splines.erosion,
        0.0..=1.0,
        hovered.map(|(r, _)| r.erosion),
      );
      changed |= spline_editor(
        ui,
        "Peaks and valleys",
        &mut self.splines.peaks_valleys,
        -128.0..=256.0,
        hovered.map(|(r, _)| r.peaks_valleys),
      );
      changed |= spline_editor(
        ui,
        "Height impact (x is continentalness / 128)",
        &mut self.splines.height_impact,
        0.0..=1.0,
        hovered.map(|(_, p)| p.c / 128.0),
      );

      self.dirty |= changed;
    });
  }

  /// The noise and height parts of the vertex the mouse was over last frame.
  fn hovered(&self) -> Option<(RawNoise, HeightParts)> {
    let grid = self.grid.as_ref()?;
    let idx = self.hover?;
    Some((*grid.raw.get(idx)?, *grid.parts.get(idx)?))
  }

  fn terrain_view(&mut self, ui: &mut egui::Ui) {
    let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
    let rect = response.rect;
    painter.rect_filled(rect, 0.0, Color32::from_rgb(28, 30, 38));

    let Some(grid) = &self.grid else {
      painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        "Sampling...",
        FontId::proportional(18.0),
        Color32::GRAY,
      );
      return;
    };
    if grid.colors.is_empty() {
      return;
    }

    let extent = grid.extent();

    if response.dragged_by(egui::PointerButton::Primary) {
      let d = response.drag_delta();
      self.camera.yaw -= d.x * 0.008;
      self.camera.pitch = (self.camera.pitch + d.y * 0.008).clamp(0.05, 1.5);
    }
    if response.dragged_by(egui::PointerButton::Secondary) {
      let d = response.drag_delta();
      self.camera.drag_pan([d.x, d.y], extent, rect);
    }
    if response.hovered() {
      let scroll = ui.input(|i| i.raw_scroll_delta.y);
      self.camera.zoom = (self.camera.zoom * (-scroll * 0.002).exp()).clamp(0.02, 4.0);
    }

    // The camera works in grid space: one unit is one block, and the terrain
    // covers 0..extent on x and z.
    let target = [extent / 2.0 + self.camera.pan[0], SEA_LEVEL as f32, extent / 2.0 + self.camera.pan[1]];
    let view = self.camera.view(rect, extent, target);

    // Find the column under the mouse.
    self.hover = None;
    if let Some(mouse) = response.hover_pos() {
      let (origin, dir) = view.ray(mouse);
      self.hover = grid.raycast(origin, dir);
    }

    if let Some(idx) = self.hover {
      if self.section.picking && response.clicked() {
        let point = grid.world(idx / grid.n, idx % grid.n);
        if self.section.a.is_none() || self.section.b.is_some() {
          self.section.a = Some(point);
          self.section.b = None;
        } else {
          self.section.b = Some(point);
          self.section.picking = false;
        }
        self.section.resample(&self.noise);
      }
    }

    let frame = gl::Frame {
      eye:        view.eye(),
      forward:    view.forward(),
      right:      view.right_scaled(rect.aspect_ratio()),
      up:         view.up_scaled(),
      pixel:      2.0 * 22.5f32.to_radians().tan() / rect.height(),
      step:       grid.step as f32,
      max_height: grid.max_height() + 1.0,
      hover:      self.hover.map(|idx| ((idx / grid.n) as i32, (idx % grid.n) as i32)),
    };
    let renderer = self.renderer.clone();
    painter.add(egui::PaintCallback {
      rect,
      callback: Arc::new(egui_glow::CallbackFn::new(move |info, painter| {
        renderer.lock().unwrap().paint(&info, painter.gl(), &frame);
      })),
    });

    // The section line, following the terrain.
    let splines = self.active_splines();
    let snap = |h: f64| ((h / grid.step).floor() * grid.step) as f32;
    let line: Vec<Option<Pos2>> = self
      .section
      .samples
      .iter()
      .map(|s| {
        let mut h = snap(splines.parts(s.noise).height);
        if self.flatten_water {
          h = h.max(snap(SEA_LEVEL));
        }
        view
          .project([
            (s.x - grid.center[0]) as f32 + extent / 2.0,
            h + grid.step as f32,
            (s.z - grid.center[1]) as f32 + extent / 2.0,
          ])
          .map(|p| p.0)
      })
      .collect();
    for w in line.windows(2) {
      if let [Some(a), Some(b)] = w {
        painter.line_segment([*a, *b], Stroke::new(2.5, Color32::from_rgb(255, 210, 40)));
      }
    }
    for (label, end) in [("A", line.first()), ("B", line.last())] {
      if let Some(Some(p)) = end {
        painter.circle_filled(*p, 5.0, Color32::from_rgb(255, 210, 40));
        painter.text(
          *p + Vec2::new(8.0, -8.0),
          Align2::LEFT_BOTTOM,
          label,
          FontId::proportional(16.0),
          Color32::WHITE,
        );
      }
    }

    // Readout.
    let text = match self.hovered() {
      Some((r, p)) => {
        let idx = self.hover.unwrap();
        let [x, z] = grid.world(idx / grid.n, idx % grid.n);
        format!(
          "x {x:.0}  z {z:.0}\n\
           continentalness {:.3} -> c {:.1}\n\
           erosion         {:.3} -> e {:.3}\n\
           peaks/valleys   {:.3} -> p {:.1}\n\
           height impact   {:.2}\n\
           height          {:.1}",
          r.continentalness, p.c, r.erosion, p.e, r.peaks_valleys, p.p, p.impact, p.height,
        )
      }
      None => "drag: orbit   right drag: pan   scroll: zoom   hover: inspect".to_string(),
    };
    let galley = painter.layout_no_wrap(text, FontId::monospace(13.0), Color32::from_gray(235));
    let pos = rect.left_top() + Vec2::splat(14.0);
    painter.rect_filled(
      Rect::from_min_size(pos, galley.size()).expand(6.0),
      4.0,
      Color32::from_black_alpha(170),
    );
    painter.galley(pos, galley, Color32::from_gray(235));
  }

  fn section_panel(&mut self, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
      ui.strong("Cross section");

      let label =
        if self.section.picking { "Click two points on the terrain..." } else { "Pick A/B" };
      if ui.selectable_label(self.section.picking, label).clicked() {
        self.section.picking = !self.section.picking;
        if self.section.picking {
          self.section.a = None;
          self.section.b = None;
          self.section.samples.clear();
        }
      }
      if ui.button("Clear").clicked() {
        self.section = Section { a: None, b: None, picking: false, samples: vec![] };
      }
    });

    if self.section.samples.is_empty() {
      ui.label("Use Pick A/B, then click two points on the terrain.");
      return;
    }

    let splines = self.active_splines();
    let series = |f: &dyn Fn(HeightParts) -> f64| -> Vec<[f64; 2]> {
      self.section.samples.iter().map(|s| [s.distance, f(splines.parts(s.noise))]).collect()
    };

    Plot::new("section")
      .legend(Legend::default())
      .allow_zoom(false)
      .allow_scroll(false)
      .allow_boxed_zoom(false)
      .include_y(0.0)
      .include_y(160.0)
      .show(ui, |plot| {
        plot.hline(HLine::new(SEA_LEVEL).name("sea level").color(Color32::from_rgb(70, 140, 210)));
        plot.line(Line::new(series(&|p| p.c)).name("continentalness (c)"));
        plot.line(Line::new(series(&|p| p.p)).name("peaks/valleys (p)"));
        plot.line(Line::new(series(&|p| p.e * 128.0)).name("erosion (e x 128)"));
        plot.line(Line::new(series(&|p| p.impact * 128.0)).name("height impact (x 128)"));
        plot.line(Line::new(series(&|p| p.height)).name("height").width(3.0).color(Color32::WHITE));
      });
  }
}

/// Draws the plot and key editor for one spline. Returns true if it changed.
fn spline_editor(
  ui: &mut egui::Ui,
  name: &str,
  spline: &mut Keys,
  range: RangeInclusive<f64>,
  marker: Option<f64>,
) -> bool {
  let mut changed = false;

  egui::CollapsingHeader::new(name).default_open(true).show(ui, |ui| {
    plot_spline(ui, name, spline, &range, marker);

    let len = spline.storage.len();
    let span = range.end() - range.start();
    let mut remove = None;

    for i in 0..len {
      ui.horizontal(|ui| {
        let fixed = i == 0 || i == len - 1;
        let lo = if i == 0 { 0.0 } else { spline.storage[i - 1].0 + 1e-4 };
        let hi = if i == len - 1 { 1.0 } else { spline.storage[i + 1].0 - 1e-4 };

        changed |= ui
          .add_enabled(
            !fixed,
            DragValue::new(&mut spline.storage[i].0).speed(0.002).clamp_range(lo..=hi.max(lo)),
          )
          .changed();
        changed |= ui
          .add(
            DragValue::new(&mut spline.storage[i].1).speed(span / 400.0).clamp_range(range.clone()),
          )
          .changed();

        if !fixed && len > 2 && ui.small_button("x").clicked() {
          remove = Some(i);
        }
      });
    }

    if let Some(i) = remove {
      spline.storage.remove(i);
      changed = true;
    }

    if ui.button("Add key").clicked() {
      // Split the widest gap, at the curve's current value.
      let gap = (0..len - 1)
        .max_by(|&a, &b| {
          let w = |i: usize| spline.storage[i + 1].0 - spline.storage[i].0;
          w(a).total_cmp(&w(b))
        })
        .unwrap();
      let t = (spline.storage[gap].0 + spline.storage[gap + 1].0) / 2.0;
      let v = spline.sample::<Cosine>(t);
      spline.storage.insert(gap + 1, (t, v));
      changed = true;
    }
  });

  changed
}

fn plot_spline(
  ui: &mut egui::Ui,
  name: &str,
  spline: &Keys,
  range: &RangeInclusive<f64>,
  marker: Option<f64>,
) {
  Plot::new(name)
    .height(110.0)
    .allow_drag(false)
    .allow_zoom(false)
    .allow_scroll(false)
    .allow_boxed_zoom(false)
    .include_x(0.0)
    .include_x(1.0)
    .include_y(*range.start())
    .include_y(*range.end())
    .show(ui, |plot| {
      plot.line(Line::new(
        (0..=200)
          .map(|i| {
            let x = i as f64 / 200.0;
            [x, spline.sample::<Cosine>(x)]
          })
          .collect::<Vec<_>>(),
      ));
      plot.points(
        Points::new(spline.storage.iter().map(|k| [k.0, k.1]).collect::<Vec<_>>()).radius(4.0),
      );

      if let Some(x) = marker {
        let x = x.clamp(0.0, 1.0);
        plot.vline(VLine::new(x).color(Color32::from_rgb(255, 210, 40)));
        plot.points(
          Points::new(vec![[x, spline.sample::<Cosine>(x)]])
            .radius(6.0)
            .color(Color32::from_rgb(255, 210, 40)),
        );
      }
    });
}
