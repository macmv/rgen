use std::{
  collections::HashMap,
  mem,
  sync::atomic::{AtomicBool, AtomicU64, Ordering},
  time::Duration,
};

use crossbeam_channel::{Receiver, Sender};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use rgen_base::Pos;
use rgen_biome::{ClimateType, GeographicType};
use rgen_placer::noise::{
  NoiseGenerator, OpenSimplexNoise, SeededNoise, ShiftedNoise, VoronoiNoise,
};

use crate::{
  RenderMode,
  color::Color,
  region::{REGION_SIZE, RegionPos},
  render::RenderBuffer,
  world::World,
};

pub const fn geography_legend() -> [(GeographicType, &'static str); 9] {
  [
    (GeographicType::MushroomIsland, "Mushroom Island"),
    (GeographicType::Ocean, "Ocean"),
    (GeographicType::Beach, "Beach"),
    (GeographicType::Canyon, "Canyon"),
    (GeographicType::River, "River"),
    (GeographicType::Delta, "Delta"),
    (GeographicType::Standard, "Standard"),
    (GeographicType::Hills, "Hills"),
    (GeographicType::Mountains, "Mountains"),
  ]
}

pub const fn climate_legend() -> [(ClimateType, &'static str); 14] {
  [
    (ClimateType::IceCap, "Ice Cap"),
    (ClimateType::Tundra, "Tundra"),
    (ClimateType::SubArctic, "Sub-Arctic"),
    (ClimateType::CoolTemperate, "Cool Temperate"),
    (ClimateType::DryTemperate, "Dry Temperate"),
    (ClimateType::WarmTemperate, "Warm Temperate"),
    (ClimateType::WetTemperate, "Wet Temperate"),
    (ClimateType::Mediterranean, "Mediterranean"),
    (ClimateType::Monsoon, "Monsoon"),
    (ClimateType::Savanna, "Savanna"),
    (ClimateType::HotDesert, "Hot Desert"),
    (ClimateType::BadLands, "Badlands"),
    (ClimateType::Tropical, "Tropical"),
    (ClimateType::DebugClimate, "Debug Climate"),
  ]
}

pub fn geography_color(category: GeographicType) -> Color {
  Color::from_hex(match category {
    GeographicType::MushroomIsland => 0xb56acb,
    GeographicType::Ocean => 0x2456a6,
    GeographicType::Beach => 0xe8d47a,
    GeographicType::Canyon => 0xa64b32,
    GeographicType::River => 0x38b4d8,
    GeographicType::Delta => 0xe58a32,
    GeographicType::Standard => 0x78ad55,
    GeographicType::Hills => 0xb1bd55,
    GeographicType::Mountains => 0x8999b0,
  })
}

pub fn climate_color(category: ClimateType) -> Color {
  Color::from_hex(match category {
    ClimateType::IceCap => 0xe8f4ff,
    ClimateType::Tundra => 0xc1d8d0,
    ClimateType::SubArctic => 0x719e98,
    ClimateType::CoolTemperate => 0x4b9b82,
    ClimateType::DryTemperate => 0xb7ae67,
    ClimateType::WarmTemperate => 0x80bd55,
    ClimateType::WetTemperate => 0x299b83,
    ClimateType::Mediterranean => 0xd7b64e,
    ClimateType::Monsoon => 0x168b63,
    ClimateType::Savanna => 0xd3a43b,
    ClimateType::HotDesert => 0xe4772c,
    ClimateType::BadLands => 0xb24d3c,
    ClimateType::Tropical => 0x176b4b,
    ClimateType::DebugClimate => 0xff00ff,
    ClimateType::GeoTest => 0xffffff,
  })
}

pub struct WorldViewer {
  pub mode: Mutex<RenderMode>,

  chunks:            RwLock<HashMap<RegionPos, RenderBuffer>>,
  other_mode_chunks: Mutex<HashMap<RenderMode, HashMap<RegionPos, RenderBuffer>>>,
  water_hatch:       AtomicBool,
  render_epoch:      AtomicU64,

  pub completed_tx: Sender<(RegionPos, RenderMode, u64, RenderBuffer)>,
  pub completed_rx: Receiver<(RegionPos, RenderMode, u64, RenderBuffer)>,

  voronoi_noise: ShiftedNoise<VoronoiNoise, OpenSimplexNoise>,
}

impl WorldViewer {
  pub fn new() -> WorldViewer {
    let (ctx, crx) = crossbeam_channel::bounded(64);

    WorldViewer {
      mode:              Mutex::new(RenderMode::Biomes),
      chunks:            RwLock::new(HashMap::new()),
      other_mode_chunks: Mutex::new(HashMap::new()),
      water_hatch:       AtomicBool::new(false),
      render_epoch:      AtomicU64::new(0),

      completed_tx: ctx,
      completed_rx: crx,

      voronoi_noise: ShiftedNoise::new(
        VoronoiNoise::new(0, 128),
        OpenSimplexNoise::new(0),
        1.0,
        1.0,
      ),
    }
  }

  pub fn recv_chunks(&self) {
    let self_mode = *self.mode.lock();
    let Some(mut w) = self.chunks.try_write_for(Duration::from_millis(10)) else {
      return;
    };
    let epoch = self.render_epoch.load(Ordering::SeqCst);
    for (pos, mode, chunk_epoch, chunk) in self.completed_rx.try_iter() {
      if chunk_epoch != epoch {
        continue;
      }
      if mode == self_mode {
        w.insert(pos, chunk);
      } else {
        self.other_mode_chunks.lock().entry(mode).or_default().insert(pos, chunk);
      }
    }
  }

  pub fn toggle_water_hatch(&self) -> bool {
    let enabled = !self.water_hatch.load(Ordering::SeqCst);
    self.water_hatch.store(enabled, Ordering::SeqCst);
    self.render_epoch.fetch_add(1, Ordering::SeqCst);
    self.chunks.write().clear();
    self.other_mode_chunks.lock().clear();
    enabled
  }

  pub fn set_mode(&self, mode: RenderMode) {
    let mut self_mode = self.mode.lock();
    if mode == *self_mode {
      return;
    }

    let mut chunks = self.chunks.write();
    let mut other_mode_chunks = self.other_mode_chunks.lock();

    match other_mode_chunks.remove(&mode) {
      Some(mut other) => {
        mem::swap(&mut *chunks, &mut other);
        other_mode_chunks.insert(*self_mode, other);
      }
      None => {
        other_mode_chunks.insert(*self_mode, mem::take(&mut *chunks));
      }
    }

    *self_mode = mode;
  }

  pub fn read_chunks(&self) -> RwLockReadGuard<'_, HashMap<RegionPos, RenderBuffer>> {
    self.chunks.read()
  }

  pub fn render_chunk(&self, world: &World, region_pos: RegionPos) {
    let mode = *self.mode.lock();
    let epoch = self.render_epoch.load(Ordering::SeqCst);
    let water_hatch = self.water_hatch.load(Ordering::SeqCst);
    let scale = if water_hatch { 4 } else { 1 };
    let mut chunk = RenderBuffer::new(
      REGION_SIZE as u32 * scale as u32,
      REGION_SIZE as u32 * scale as u32,
    );

    for rel_x in 0..REGION_SIZE {
      for rel_z in 0..REGION_SIZE {
        // Sample at the top of the world for surface biomes.
        let pos = region_pos.min_block_pos() + Pos::new(rel_x, 255, rel_z);
        let column = world.column_at(pos);

        let voronoi = self.voronoi_noise.generate(pos.x as f64 / 64.0, pos.z as f64 / 64.0);
        let voronoi = (voronoi % 10) as f64 / 10.0;

        let biome = column.biome;
        let meter_height = column.height;

        let block_distance = 1;
        // ╔═╦═╦═╗
        // ║a║b║c║
        // ╠═╬═╬═╣     MINECRAFT
        // ║d║é║f║     - X & Z is flat plane
        // ╠═╬═╬═╣     - Y is up
        // ║g║h║i║
        // ╚═╩═╩═╝ <- var table  || block_distance

        let a = world.height_at(pos + Pos::new(-block_distance, 0, block_distance));
        let b = world.height_at(pos + Pos::new(0, 0, block_distance));
        let c = world.height_at(pos + Pos::new(block_distance, 0, block_distance));

        let d = world.height_at(pos + Pos::new(-block_distance, 0, 0));
        let f = world.height_at(pos + Pos::new(block_distance, 0, 0));

        let g = world.height_at(pos + Pos::new(-block_distance, 0, -block_distance));
        let h = world.height_at(pos + Pos::new(0, 0, -block_distance));
        let i = world.height_at(pos + Pos::new(block_distance, 0, -block_distance));

        let dz_dx = ((c + (2.0 * f) + i) * 4.0 - (a + (2.0 * d) + g) * 4.0) / (8.0 * 1.0);
        //[dz/dx] = ((c + 2f + i)*4/wght1 - (a + 2d + g)*4/wght2) / (8 * x_cellsize)

        let dz_dy = ((g + (2.0 * h) + i) * 4.0 - (a + (2.0 * b) + c) * 4.0) / (8.0 * 1.0);
        //[dz/dy] = ((g + 2h + i)*4/wght3 - (a + 2b + c)*4/wght4) / (8 * y_cellsize)

        //claculates cell slope at that location

        let cell_slope = ((dz_dx).powi(2) + (dz_dy).powi(2)).sqrt().atan();
        //dbg!(cell_slope);
        //Slope = arctan(sqrt((dz/dx)^2 + (dz/dy)^2))

        //calulates cell aspect this is the direction the cell is facing as a slope
        let cell_aspect = dz_dx.atan2(-dz_dy);
        //arctan2(dz/dx, -dz/dy)

        let azimuth = 315.0 / 180.0 * std::f64::consts::PI;
        let altidue = 45.0 / 180.0 * std::f64::consts::PI;

        let solar_incidence_angle = (altidue.sin() * cell_slope.sin()
          + altidue.cos() * cell_slope.cos() * (azimuth - cell_aspect).cos())
        .acos();

        let brightness = ((solar_incidence_angle).cos() + 1.0) / 2.0;

        let height_color =
          Color::from_gray(brightness as f32 * 0.2 + (meter_height as f32 / 256.0) * 2.0);
        let biome_color = biome.color;

        let biome_color = if meter_height < 64.0 { Color::from_hex(0x009dc4) } else { biome_color };

        let alpha = 40.0 / 100.0;
        let color = match mode {
          RenderMode::Biomes => height_color.fade(biome_color, alpha),
          RenderMode::Continentalness => Color::from_gray(biome.continentalness as f32),
          RenderMode::Erosion => Color::from_gray(biome.erosion as f32),
          RenderMode::PeaksValleys => Color::from_gray(biome.peaks_valleys as f32),
          RenderMode::Height => Color::from_gray(column.height as f32 / 256.0),
          RenderMode::Voronoi => match voronoi {
            x if x < 0.1 => Color::from_hex(0x000000),
            x if x < 0.2 => Color::from_hex(0xff0000),
            x if x < 0.3 => Color::from_hex(0x00ff00),
            x if x < 0.4 => Color::from_hex(0x0000ff),
            x if x < 0.5 => Color::from_hex(0xffff00),
            x if x < 0.6 => Color::from_hex(0xff00ff),
            x if x < 0.7 => Color::from_hex(0xffffff),
            x if x < 0.8 => Color::from_hex(0xffaa00),
            x if x < 0.9 => Color::from_hex(0xff00aa),
            _ => Color::from_hex(0xffaaaa),
          },
          RenderMode::Geography => {
            height_color.fade(geography_color(world.generator.geographic_type(pos)), alpha)
          }
          RenderMode::Climate => {
            height_color.fade(climate_color(world.generator.climate_type(pos)), alpha)
          }
        };

        let is_water = meter_height < 64.0;
        let border_left = water_hatch && is_water && d >= 64.0;
        let border_right = water_hatch && is_water && f >= 64.0;
        let border_top = water_hatch && is_water && h >= 64.0;
        let border_bottom = water_hatch && is_water && b >= 64.0;
        for sub_x in 0..scale {
          for sub_z in 0..scale {
            let on_outline = (border_left && sub_x == 0)
              || (border_right && sub_x == scale - 1)
              || (border_top && sub_z == 0)
              || (border_bottom && sub_z == scale - 1);
            let on_hatch = water_hatch
              && is_water
              && (pos.x * scale + sub_x + pos.z * scale + sub_z).rem_euclid(12) < 2;
            let pixel_color = if on_outline {
              Color::from_hex(0x073b55)
            } else if on_hatch {
              color.fade(Color::from_hex(0x9be5ff), 0.35)
            } else {
              color
            };
            chunk.set(rel_x * scale + sub_x, rel_z * scale + sub_z, pixel_color.to_sdl2());
          }
        }
      }
    }

    self.completed_tx.send((region_pos, mode, epoch, chunk)).unwrap();
  }
}
