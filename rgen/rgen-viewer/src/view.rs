use std::{
  collections::{HashMap, HashSet},
  mem,
};

use crossbeam_channel::{Receiver, Sender, TrySendError};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use rgen_base::{Biome, ChunkPos, Pos};
use rgen_world::Context;
use sdl2::pixels::Color;

use crate::{render::RenderBuffer, world::WorldReadLock, RenderMode};

pub struct WorldViewer {
  pub mode: Mutex<RenderMode>,

  chunks:            RwLock<HashMap<ChunkPos, RenderBuffer>>,
  other_mode_chunks: Mutex<HashMap<RenderMode, HashMap<ChunkPos, RenderBuffer>>>,

  pub render_requested: Mutex<HashSet<ChunkPos>>,
  pub render_tx:        Sender<ChunkPos>,
  pub render_rx:        Receiver<ChunkPos>,

  pub completed_tx: Sender<(ChunkPos, RenderBuffer)>,
  pub completed_rx: Receiver<(ChunkPos, RenderBuffer)>,
}

impl WorldViewer {
  pub fn new() -> WorldViewer {
    let (tx, rx) = crossbeam_channel::bounded(64);
    let (ctx, crx) = crossbeam_channel::bounded(64);

    WorldViewer {
      mode:              Mutex::new(RenderMode::Brightness),
      chunks:            RwLock::new(HashMap::new()),
      other_mode_chunks: Mutex::new(HashMap::new()),

      render_requested: Mutex::new(HashSet::new()),
      render_tx:        tx,
      render_rx:        rx,

      completed_tx: ctx,
      completed_rx: crx,
    }
  }

  pub fn recv_chunks(&self) {
    let mut w = self.chunks.write();
    for (pos, chunk) in self.completed_rx.try_iter() {
      w.insert(pos, chunk);
    }
  }

  fn request(&self, pos: ChunkPos) -> bool {
    let mut render_requested = self.render_requested.lock();
    if render_requested.insert(pos) {
      match self.render_tx.try_send(pos) {
        Ok(_) => true,
        Err(TrySendError::Full(_)) => {
          render_requested.remove(&pos);
          false
        }
        Err(TrySendError::Disconnected(_)) => {
          panic!("Render thread disconnected");
        }
      }
    } else {
      true
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

  pub fn read_chunks(&self) -> RwLockReadGuard<HashMap<ChunkPos, RenderBuffer>> {
    self.chunks.read()
  }

  /// Returns `true` if the chunk was succesfully requested, `false` if the
  /// channel is full.
  pub fn request_render(&self, world: &WorldReadLock, chunk_pos: ChunkPos) -> bool {
    if world.has_chunk(chunk_pos + ChunkPos::new(1, 1))
      && world.has_chunk(chunk_pos + ChunkPos::new(1, 0))
      && world.has_chunk(chunk_pos + ChunkPos::new(1, -1))
      && world.has_chunk(chunk_pos + ChunkPos::new(0, 1))
      && world.has_chunk(chunk_pos)
      && world.has_chunk(chunk_pos + ChunkPos::new(0, -1))
      && world.has_chunk(chunk_pos + ChunkPos::new(-1, 1))
      && world.has_chunk(chunk_pos + ChunkPos::new(-1, 0))
      && world.has_chunk(chunk_pos + ChunkPos::new(-1, -1))
    {
      self.request(chunk_pos)
    } else {
      true
    }
  }

  pub fn render_chunk(&self, context: &Context, world: &WorldReadLock, chunk_pos: ChunkPos) {
    let mut chunk = RenderBuffer::new(16, 16);

    for rel_x in 0..16 {
      for rel_z in 0..16 {
        let pos = chunk_pos.min_block_pos() + Pos::new(rel_x, 0, rel_z);
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

        let brightness = match *self.mode.lock() {
          RenderMode::Height => (meter_height * 2.0) as u8,
          RenderMode::Slope => (cell_slope * 255.0 / std::f64::consts::PI) as u8,
          RenderMode::Aspect => {
            //let asp = (cell_aspect * 255.0 / std::f64::consts::PI) as u8;
            //println!("Aspect: {asp}");
            (cell_aspect * 255.0 / std::f64::consts::PI) as u8
          }
          RenderMode::Brightness => {
            //let bright = (brightness as f64 * 0.2 + meter_height as f64 * 2.0) as u8;
            //println!("Brightness: {bright}");
            (brightness as f64 * 0.2 + meter_height as f64 * 2.0) as u8
          }
          RenderMode::BiomeColors => 0,
          //
          //HSV
          //Hue:0-360         - This is the color of the terrain
          //Saturation:0-100  - This is the terrain height
          //Value:0-100       - Keep locked too set darkness to max-light
        };

        let height_color = Color::RGB(brightness, brightness, brightness);
        let biome_color = color_for_biome(context, biome);

        let biome_color = if meter_height < 64.0 { Color::RGB(0, 157, 196) } else { biome_color };

        let transparency = 40;
        let alpha = (255 * transparency / 100) as u8;
        let r = std::cmp::min(
          ((height_color.r as u16 * alpha as u16 + biome_color.r as u16 * (255 - alpha as u16))
            / 255) as u8,
          255,
        );
        let g = std::cmp::min(
          ((height_color.g as u16 * alpha as u16 + biome_color.g as u16 * (255 - alpha as u16))
            / 255) as u8,
          255,
        );
        let b = std::cmp::min(
          ((height_color.b as u16 * alpha as u16 + biome_color.b as u16 * (255 - alpha as u16))
            / 255) as u8,
          255,
        );
        chunk.set(rel_x, rel_z, Color::RGB(r, g, b));
      }
    }

    self.completed_tx.send((chunk_pos, chunk)).unwrap();
  }
}
pub fn color_for_biome(ctx: &Context, biome: Biome) -> Color {
  let biome_hex = match biome {
    b if b == ctx.biomes.ice_plains => 0x518ded,
    b if b == ctx.biomes.cold_taiga => 0x3265db,
    b if b == ctx.biomes.extreme_hills => 0x4f6aab,
    b if b == ctx.biomes.plains => 0x61b086,
    b if b == ctx.biomes.savanna => 0xa19d55,
    _ => {
      //println!("unknown biome {b:?}");
      0x000000
    }
  };

  Color::RGB((biome_hex >> 16) as u8 / 4, (biome_hex >> 8) as u8 / 4, biome_hex as u8 / 4)
}
