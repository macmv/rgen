use std::{collections::HashMap, mem};

use crossbeam_channel::{Receiver, Sender};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use rgen_base::Pos;
use sdl2::pixels::Color;

use crate::{
  region::{RegionPos, REGION_SIZE},
  render::RenderBuffer,
  world::WorldReadLock,
  RenderMode,
};

pub struct WorldViewer {
  pub mode: Mutex<RenderMode>,

  chunks:            RwLock<HashMap<RegionPos, RenderBuffer>>,
  other_mode_chunks: Mutex<HashMap<RenderMode, HashMap<RegionPos, RenderBuffer>>>,

  pub completed_tx: Sender<(RegionPos, RenderMode, RenderBuffer)>,
  pub completed_rx: Receiver<(RegionPos, RenderMode, RenderBuffer)>,
}

impl WorldViewer {
  pub fn new() -> WorldViewer {
    let (ctx, crx) = crossbeam_channel::bounded(64);

    WorldViewer {
      mode:              Mutex::new(RenderMode::Biomes),
      chunks:            RwLock::new(HashMap::new()),
      other_mode_chunks: Mutex::new(HashMap::new()),

      completed_tx: ctx,
      completed_rx: crx,
    }
  }

  pub fn recv_chunks(&self) {
    let self_mode = *self.mode.lock();
    let mut w = self.chunks.write();
    for (pos, mode, chunk) in self.completed_rx.try_iter() {
      if mode == self_mode {
        w.insert(pos, chunk);
      } else {
        self.other_mode_chunks.lock().entry(mode).or_insert_with(HashMap::new).insert(pos, chunk);
      }
    }
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

  pub fn read_chunks(&self) -> RwLockReadGuard<HashMap<RegionPos, RenderBuffer>> {
    self.chunks.read()
  }

  pub fn render_chunk(&self, world: &WorldReadLock, region_pos: RegionPos) {
    let mut chunk = RenderBuffer::new(REGION_SIZE as u32, REGION_SIZE as u32);
    let mode = *self.mode.lock();

    for rel_x in 0..REGION_SIZE {
      for rel_z in 0..REGION_SIZE {
        let pos = region_pos.min_block_pos() + Pos::new(rel_x, 0, rel_z);
        let column = world.column_at(pos);

        let biome = column.biome;
        let meter_height = column.height as f64;

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

        let brightness = ((((solar_incidence_angle).cos() + 1.0) / 2.0) * 255.0) as u8;

        let brightness = (brightness as f64 * 0.2 + meter_height as f64 * 2.0) as u8;

        let height_color = Color::RGB(brightness, brightness, brightness);
        let biome_color = biome.color();

        let biome_color = if meter_height < 64.0 { Color::RGB(0, 157, 196) } else { biome_color };

        let main_color = match mode {
          RenderMode::Biomes => biome_color,
          RenderMode::Continentalness => Color::RGB(
            (biome.continentalness * 255.0) as u8,
            (biome.continentalness * 255.0) as u8,
            (biome.continentalness * 255.0) as u8,
          ),
        };

        let transparency = 40;
        let alpha = (255 * transparency / 100) as u8;
        let r = ((height_color.r as u16 * alpha as u16
          + main_color.r as u16 * (255 - alpha as u16))
          / 255) as u8;
        let g = ((height_color.g as u16 * alpha as u16
          + main_color.g as u16 * (255 - alpha as u16))
          / 255) as u8;
        let b = ((height_color.b as u16 * alpha as u16
          + main_color.b as u16 * (255 - alpha as u16))
          / 255) as u8;
        chunk.set(rel_x, rel_z, Color::RGB(r, g, b));
      }
    }

    self.completed_tx.send((region_pos, mode, chunk)).unwrap();
  }
}
