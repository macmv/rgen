use std::{
  collections::HashSet,
  ops::RangeInclusive,
  sync::{Arc, Mutex, mpsc},
};

use eframe::{
  egui::{self, Align2, Color32, DragValue, FontId, Pos2, Rect, Sense, Stroke, Vec2},
  egui_glow,
};
use egui_plot::{HLine, Legend, Line, Plot, Points, VLine};
use rgen_base::Pos;
use rgen_biome::{HeightParts, RawNoise, TerrainNoise, WorldBiomes};
use rgen_world::Context;
use rgen_spline::{Cosine, Spline};

mod camera;
mod chunks;
mod gl;
mod grid;
mod pool;

use camera::Camera;
use grid::{Grid, Layer, MAX_RADIUS, SEA_LEVEL, TILE_BLOCKS, UNLOAD_MARGIN};
use pool::{Params, Pool, TileResult};

/// The seed from the first argument, or a random one.
fn initial_seed() -> u64 {
  match std::env::args().nth(1) {
    Some(arg) => arg.parse().unwrap_or_else(|e| {
      eprintln!("Invalid seed: {e}");
      std::process::exit(1);
    }),
    None => std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .map_or(0, |d| d.as_nanos() as u64),
  }
}

fn make_world(seed: u64) -> Arc<WorldBiomes> {
  let context = Context::new_test(seed);
  Arc::new(WorldBiomes::new(&context.blocks, context.seed))
}

fn main() -> Result<(), eframe::Error> {
  let seed = initial_seed();
  println!("Using seed {seed}");

  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 900.0]),
    ..Default::default()
  };
  eframe::run_native(
    "Terrain Inspector",
    options,
    Box::new(move |cc| {
      let gl = cc.gl.as_ref().expect("the terrain view needs OpenGL");
      let renderer = gl::Renderer::new(gl, cc.egui_ctx.clone()).expect("failed to build the terrain shader");
      Box::new(App::new(&cc.egui_ctx, Arc::new(Mutex::new(renderer)), seed))
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

  seed:          u64,
  /// How many chunks around the camera are generated.
  view_distance: i32,

  noise:     Arc<TerrainNoise>,
  /// The full generator, which is what picks biomes.
  world:     Arc<WorldBiomes>,
  pool:      Pool,
  /// What the workers are making columns with.
  params:    Arc<Params>,
  results:   mpsc::Receiver<TileResult>,
  /// Bumped whenever the world starts over, so that stale chunks are ignored.
  epoch:     u64,
  /// The chunks that are generated, or are queued or being generated.
  requested: HashSet<[i32; 2]>,
  /// The center and view distance that chunks were last queued for.
  planned:   Option<([i32; 2], i32)>,

  grid:  Grid,
  /// Set when the splines or display options change, so the grid colors need to
  /// be rebuilt.
  dirty: bool,

  layer:         Layer,
  flatten_water: bool,
  /// Draws the borders of the chunks around the camera.
  chunk_borders: bool,
  camera:        Camera,
  /// The world position of the column under the mouse, from the last frame.
  hover:         Option<[i32; 2]>,

  renderer: Arc<Mutex<gl::Renderer>>,

  section: Section,
}

impl App {
  fn new(ctx: &egui::Context, renderer: Arc<Mutex<gl::Renderer>>, seed: u64) -> Self {
    let (tx, rx) = mpsc::channel();
    let params = Arc::new(Params {
      id:            0,
      splines:       Splines::from_game(),
      layer:         Layer::Height,
      flatten_water: true,
    });
    let grid = Grid::new(renderer.lock().unwrap().brick_capacity());
    renderer.lock().unwrap().queue(grid.reset_upload());
    App {
      splines: Splines::from_game(),
      saved: Splines::from_game(),
      compare_saved: false,

      seed,
      view_distance: 10,

      noise: Arc::new(TerrainNoise::new(seed)),
      world: make_world(seed),
      pool: Pool::new(ctx, tx, params.clone()),
      params,
      results: rx,
      epoch: 0,
      requested: HashSet::new(),
      planned: None,

      grid,
      dirty: false,

      layer: Layer::Height,
      flatten_water: true,
      chunk_borders: false,
      camera: Camera::default(),
      hover: None,

      renderer,

      section: Section { a: None, b: None, picking: false, samples: vec![] },
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    self.plan_chunks();
    self.receive_tiles(ctx);

    egui::SidePanel::left("controls").default_width(340.0).show(ctx, |ui| self.controls(ui));

    if self.dirty {
      self.params = Arc::new(Params {
        id:            self.params.id + 1,
        splines:       self.active_splines().clone(),
        layer:         self.layer,
        flatten_water: self.flatten_water,
      });
      self.pool.set_params(self.params.clone());
      let uploads = self.grid.recompute_all(
        &self.params.splines,
        self.params.layer,
        self.params.flatten_water,
      );
      let mut renderer = self.renderer.lock().unwrap();
      for upload in uploads {
        renderer.queue(upload);
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

/// The camera's zoom is relative to this many blocks, so that it doesn't change
/// when the view distance does.
const CAMERA_EXTENT: f32 = 21.0 * TILE_BLOCKS as f32;

/// How long a frame spends copying chunks into the grid, once it has copied one.
/// Past that, the rest wait for the next frame, so that the window stays
/// responsive.
const TILE_BUDGET: std::time::Duration = std::time::Duration::from_millis(6);

impl App {
  fn active_splines(&self) -> &Splines {
    if self.compare_saved { &self.saved } else { &self.splines }
  }

  /// The chunk the camera is looking at.
  fn center_tile(&self) -> [i32; 2] {
    self.camera.target.map(|t| (t / TILE_BLOCKS as f64).floor() as i32)
  }

  /// Throws away all the terrain, and starts generating it again.
  fn restart(&mut self) {
    self.epoch += 1;
    self.requested.clear();
    self.planned = None;
    self.pool.reset(self.epoch);

    self.grid = Grid::new(self.renderer.lock().unwrap().brick_capacity());
    self.renderer.lock().unwrap().queue(self.grid.reset_upload());
  }

  /// Moves the grid to follow the camera, and queues any chunks within the view
  /// distance that aren't generated yet. Chunks that are too far away are
  /// thrown out, and the rest are left alone.
  fn plan_chunks(&mut self) {
    let center = self.center_tile();
    let radius = self.view_distance;
    if self.planned == Some((center, radius)) {
      return;
    }
    self.planned = Some((center, radius));

    self.grid.set_center(center);
    // Generated chunks stay a bit past the view distance, but any that are
    // still waiting to be generated are only wanted inside it.
    let updates = self.grid.evict_outside(center, radius + UNLOAD_MARGIN);
    if !updates.is_empty() {
      self.renderer.lock().unwrap().queue(grid::Upload::Chunks(updates));
    }
    self.requested.retain(|&t| self.grid.contains(t) || grid::in_range(center, radius, t));

    let new: Vec<[i32; 2]> =
      grid::tiles_in_range(center, radius).filter(|t| !self.requested.contains(t)).collect();
    self.requested.extend(new.iter().copied());
    self.pool.plan(self.epoch, &self.noise, &self.world, center, radius, &new);
  }

  /// Copies the chunks the workers finished into the terrain.
  fn receive_tiles(&mut self, ctx: &egui::Context) {
    let start = std::time::Instant::now();

    let mut renderer = self.renderer.lock().unwrap();
    while start.elapsed() < TILE_BUDGET {
      let Ok(tile) = self.results.try_recv() else { return };
      // Chunks that went out of range while they were generated are dropped.
      if tile.epoch != self.epoch || !self.requested.contains(&tile.tile) {
        continue;
      }

      // Columns made with older splines are out of date, so make them again.
      let columns = if tile.params_id == self.params.id {
        tile.columns
      } else {
        let p = &self.params;
        Grid::tile_columns(&tile.raw, &tile.biomes, &p.splines, p.layer, p.flatten_water)
      };

      for upload in self.grid.set_tile(tile.tile, &tile.raw, &tile.biomes, &columns) {
        renderer.queue(upload);
      }
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
        self.world = make_world(self.seed);
        self.section.resample(&self.noise);
        self.restart();
      }
      ui.end_row();

      ui.label("Looking at");
      ui.horizontal(|ui| {
        ui.add(DragValue::new(&mut self.camera.target[0]).speed(8.0));
        ui.add(DragValue::new(&mut self.camera.target[1]).speed(8.0));
      });
      ui.end_row();

      ui.label("View distance");
      ui.add(egui::Slider::new(&mut self.view_distance, 1..=MAX_RADIUS).suffix(" chunks"));
      ui.end_row();

      ui.label("Generated");
      let center = self.center_tile();
      let in_view = || grid::tiles_in_range(center, self.view_distance);
      let done = in_view().filter(|&t| self.grid.contains(t)).count();
      let total = in_view().count();
      ui.label(if done == total { "done".to_string() } else { format!("{done} / {total}") });
      ui.end_row();

      if self.grid.chunks.overflowed {
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
    ui.checkbox(&mut self.chunk_borders, "Show chunk borders");
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
    let raw = self.grid.raw_at(self.hover?)?;
    Some((raw, self.active_splines().parts(raw)))
  }

  fn terrain_view(&mut self, ui: &mut egui::Ui) {
    let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
    let rect = response.rect;
    painter.rect_filled(rect, 0.0, Color32::from_rgb(28, 30, 38));

    let grid = &self.grid;
    let longest = CAMERA_EXTENT;

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

    // The camera works in window space: one unit is one block, and the window
    // covers 0..size on x and z.
    let window = grid.window_origin();
    let target = [
      (self.camera.target[0] - window[0]) as f32,
      SEA_LEVEL as f32,
      (self.camera.target[1] - window[1]) as f32,
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
        let point = idx.map(f64::from);
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
      shift:      grid.shift(),
      hover:      self.hover.map(|idx| {
        let [i, j] = grid.to_window(idx);
        (i, j)
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
          .project([(s.x - window[0]) as f32, h + 1.0, (s.z - window[1]) as f32])
          .map(|p| p.0)
      })
      .collect();
    for w in line.windows(2) {
      if let [Some(a), Some(b)] = w {
        painter.line_segment([*a, *b], Stroke::new(2.5_f32, Color32::from_rgb(255, 210, 40)));
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

    if self.chunk_borders {
      self.paint_chunk_borders(&painter, &view, window);
    }

    // Readout.
    let text = match self.hovered() {
      Some((r, p)) => {
        let [x, z] = self.hover.unwrap();
        let biome = self.world.choose_surface_biome(Pos::new(x, 255, z)).name;
        format!(
          "x {x:.0}  z {z:.0}\n\
           biome           {biome}\n\
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

  /// Draws the borders of the 16 block chunks around the camera in yellow, and
  /// the borders of the generated 128 block chunks in orange, at sea level.
  fn paint_chunk_borders(&self, painter: &egui::Painter, view: &camera::View, window: [f64; 2]) {
    // How far from the camera the 16 block borders are drawn, in blocks.
    const NEAR: i32 = 16 * 16;

    let center = self.camera.target.map(|t| t.floor() as i32);
    let y = SEA_LEVEL as f32;
    let to_screen = |x: i32, z: i32| {
      view.project([(f64::from(x) - window[0]) as f32, y, (f64::from(z) - window[1]) as f32])
    };
    let line = |a: [i32; 2], b: [i32; 2], stroke: Stroke| {
      if let (Some(a), Some(b)) = (to_screen(a[0], a[1]), to_screen(b[0], b[1])) {
        painter.line_segment([a.0, b.0], stroke);
      }
    };

    let small = Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(255, 255, 0, 110));
    let min = center.map(|c| (c - NEAR).div_euclid(16) * 16);
    let max = center.map(|c| (c + NEAR).div_euclid(16) * 16);
    for x in (min[0]..=max[0]).step_by(16) {
      line([x, min[1]], [x, max[1]], small);
    }
    for z in (min[1]..=max[1]).step_by(16) {
      line([min[0], z], [max[0], z], small);
    }

    let big = Stroke::new(1.5_f32, Color32::from_rgb(255, 140, 40));
    let tile = TILE_BLOCKS as i32;
    let radius = self.view_distance + 1;
    let c = self.center_tile();
    for t in -radius..=radius {
      let (x, z) = ((c[0] + t) * tile, (c[1] + t) * tile);
      let (lo, hi) = ((c[0] - radius) * tile, (c[0] + radius + 1) * tile);
      line([x, (c[1] - radius) * tile], [x, (c[1] + radius + 1) * tile], big);
      line([lo, z], [hi, z], big);
    }
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
        plot.line(Line::new(series(&|p| p.height)).name("height").width(3.0_f32).color(Color32::WHITE));
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
        Points::new(spline.storage.iter().map(|k| [k.0, k.1]).collect::<Vec<_>>()).radius(4.0_f32),
      );

      if let Some(x) = marker {
        let x = x.clamp(0.0, 1.0);
        plot.vline(VLine::new(x).color(Color32::from_rgb(255, 210, 40)));
        plot.points(
          Points::new(vec![[x, spline.sample::<Cosine>(x)]])
            .radius(6.0_f32)
            .color(Color32::from_rgb(255, 210, 40)),
        );
      }
    });
}
