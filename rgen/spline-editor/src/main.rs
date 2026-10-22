use std::{
  ops::RangeInclusive,
  sync::{Arc, Mutex, mpsc},
};

use eframe::{
  egui::{self, Align2, Color32, DragValue, FontId, Pos2, Rect, Sense, Stroke, Vec2},
  egui_glow,
};
use egui_plot::{HLine, Legend, Line, Plot, Points, VLine};
use rgen_biome::{HeightParts, RawNoise, TerrainNoise};
use rgen_spline::{Cosine, Spline};

mod camera;
mod chunks;
mod gl;
mod grid;
mod pool;

use camera::Camera;
use grid::{Grid, Layer, SEA_LEVEL, TILE_BLOCKS};
use pool::{Pool, TileResult};

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
      let renderer = gl::Renderer::new(gl, cc.egui_ctx.clone()).expect("failed to build the terrain shader");
      Box::new(App::new(&cc.egui_ctx, Arc::new(Mutex::new(renderer))))
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

  seed:         u64,
  /// The chunk in the middle of the terrain.
  center_tile:  [i32; 2],
  /// The number of chunks along x and z that are being generated.
  tiles:        [usize; 2],
  /// The values of the size sliders. These are only applied once the slider is
  /// let go, as a new size regenerates everything.
  slider_tiles: [usize; 2],

  noise:     Arc<TerrainNoise>,
  pool:      Pool,
  results:   mpsc::Receiver<TileResult>,
  /// Bumped whenever a new region is started, so that stale chunks are ignored.
  epoch:     u64,
  /// What the current region was generated with.
  requested: Option<(u64, [i32; 2], [usize; 2])>,

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
  fn new(ctx: &egui::Context, renderer: Arc<Mutex<gl::Renderer>>) -> Self {
    let (tx, rx) = mpsc::channel();
    App {
      splines: Splines::from_game(),
      saved: Splines::from_game(),
      compare_saved: false,

      seed: 0,
      center_tile: [0, 0],
      tiles: [21, 21],
      slider_tiles: [21, 21],

      noise: Arc::new(TerrainNoise::new(0)),
      pool: Pool::new(ctx, tx),
      results: rx,
      epoch: 0,
      requested: None,

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
    self.request_region();
    self.receive_tiles(ctx);

    egui::SidePanel::left("controls").default_width(340.0).show(ctx, |ui| self.controls(ui));

    if self.dirty {
      let splines = if self.compare_saved { self.saved.clone() } else { self.splines.clone() };
      if let Some(grid) = &mut self.grid {
        let updates = grid.recompute_all(&splines, self.layer, self.flatten_water);
        let mut renderer = self.renderer.lock().unwrap();
        renderer.queue(grid.full_upload());
        renderer.queue(grid::Upload::Chunks(updates));
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

/// The most chunks along one side. Every block is a column, so a bigger world
/// takes a lot of memory.
const MAX_TILES: usize = 48;

/// The most chunks that are copied into the grid each frame.
const TILES_PER_FRAME: usize = 24;

impl App {
  fn active_splines(&self) -> &Splines {
    if self.compare_saved { &self.saved } else { &self.splines }
  }

  /// The world position of the terrain's minimum corner.
  fn origin(&self) -> [f64; 2] {
    let first = |c: i32, tiles: usize| f64::from(c - tiles as i32 / 2) * TILE_BLOCKS as f64;
    [first(self.center_tile[0], self.tiles[0]), first(self.center_tile[1], self.tiles[1])]
  }

  /// Starts generating a new region if the parameters changed.
  fn request_region(&mut self) {
    let key = (self.seed, self.center_tile, self.tiles);
    if self.requested == Some(key) {
      return;
    }
    self.requested = Some(key);
    self.epoch += 1;

    let mut renderer = self.renderer.lock().unwrap();
    let grid = Grid::new(self.origin(), self.tiles, renderer.brick_capacity());
    renderer.queue(grid.reset_upload());
    renderer.queue(grid.full_upload());
    drop(renderer);
    self.pool.start_region(self.epoch, &self.noise, grid.origin, self.tiles);
    self.grid = Some(grid);
  }

  /// Copies the chunks the workers finished into the terrain.
  fn receive_tiles(&mut self, ctx: &egui::Context) {
    let Some(grid) = &mut self.grid else { return };
    let splines = if self.compare_saved { &self.saved } else { &self.splines };

    let mut renderer = self.renderer.lock().unwrap();
    let mut received = 0;
    while received < TILES_PER_FRAME {
      let Ok(tile) = self.results.try_recv() else { return };
      if tile.epoch != self.epoch {
        continue;
      }

      for upload in
        grid.set_tile(tile.tx, tile.tz, &tile.raw, splines, self.layer, self.flatten_water)
      {
        renderer.queue(upload);
      }
      received += 1;
    }

    // There might be more waiting, so come back next frame.
    ctx.request_repaint();
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

      ui.label("Center chunk");
      ui.horizontal(|ui| {
        ui.add(DragValue::new(&mut self.center_tile[0]).speed(0.2));
        ui.add(DragValue::new(&mut self.center_tile[1]).speed(0.2));
      });
      ui.end_row();

      for (axis, label) in ["Width (x)", "Height (z)"].into_iter().enumerate() {
        ui.label(label);
        let response =
          ui.add(egui::Slider::new(&mut self.slider_tiles[axis], 1..=MAX_TILES).suffix(" chunks"));
        if response.drag_released() || (response.changed() && !response.dragged()) {
          self.tiles[axis] = self.slider_tiles[axis];
        }
        ui.end_row();
      }

      ui.label("Region");
      let [x, z] = self.tiles.map(|t| t * TILE_BLOCKS);
      ui.label(format!("{x} x {z} blocks"));
      ui.end_row();

      ui.label("Generated");
      let done = self.grid.as_ref().map_or(0, |g| g.loaded_count);
      let total = self.tiles[0] * self.tiles[1];
      ui.label(if done == total { "done".to_string() } else { format!("{done} / {total}") });
      ui.end_row();

      if self.grid.as_ref().is_some_and(|g| g.chunks.overflowed) {
        ui.label("Warning");
        ui.colored_label(Color32::from_rgb(255, 150, 60), "out of chunk slots");
        ui.end_row();
      }
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
    let raw = grid.raw(self.hover?)?;
    Some((raw, self.active_splines().parts(raw)))
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
    let longest = extent[0].max(extent[1]);

    if response.dragged_by(egui::PointerButton::Primary) {
      let d = response.drag_delta();
      self.camera.yaw -= d.x * 0.008;
      self.camera.pitch = (self.camera.pitch + d.y * 0.008).clamp(0.05, 1.5);
    }
    if response.dragged_by(egui::PointerButton::Secondary) {
      let d = response.drag_delta();
      self.camera.drag_pan([d.x, d.y], longest, rect);
    }
    if response.hovered() {
      let scroll = ui.input(|i| i.raw_scroll_delta.y);
      self.camera.zoom = (self.camera.zoom * (-scroll * 0.002).exp()).clamp(0.02, 4.0);
    }

    // The camera works in grid space: one unit is one block, and the terrain
    // covers 0..extent on x and z.
    let target = [
      extent[0] / 2.0 + self.camera.pan[0],
      SEA_LEVEL as f32,
      extent[1] / 2.0 + self.camera.pan[1],
    ];
    let view = self.camera.view(rect, longest, target);

    // Find the column under the mouse.
    self.hover = None;
    if let Some(mouse) = response.hover_pos() {
      let (origin, dir) = view.ray(mouse);
      self.hover = grid.raycast(origin, dir);
    }

    if let Some(idx) = self.hover {
      if self.section.picking && response.clicked() {
        let (i, j) = grid.coords(idx);
        let point = grid.world(i, j);
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
      hover:      self.hover.map(|idx| {
        let (i, j) = grid.coords(idx);
        (i as i32, j as i32)
      }),
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
    let line: Vec<Option<Pos2>> = self
      .section
      .samples
      .iter()
      .map(|s| {
        let mut h = splines.parts(s.noise).height.floor() as f32;
        if self.flatten_water {
          h = h.max(SEA_LEVEL.floor() as f32);
        }
        view
          .project([(s.x - grid.origin[0]) as f32, h + 1.0, (s.z - grid.origin[1]) as f32])
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
        let (i, j) = grid.coords(idx);
        let [x, z] = grid.world(i, j);
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
