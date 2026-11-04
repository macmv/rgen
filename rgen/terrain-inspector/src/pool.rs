//! Generates the chunks of terrain on a pool of threads. The main thread
//! queues up the chunks around the camera, and the workers pull the nearest one
//! off as they get free. As the camera moves, the queue is replanned: chunks
//! that are now too far away are dropped, and new ones are added.

use std::sync::{
  Arc, Condvar, Mutex,
  atomic::{AtomicU64, Ordering},
  mpsc,
};

use eframe::egui::{self, Color32};
use rgen_base::Pos;
use rgen_biome::{RawNoise, TerrainNoise, WorldBiomes};

use crate::{
  Splines,
  grid::{Grid, Layer, TILE_BLOCKS, distance_sq},
  real::{RealTerrain, TileBlocks},
};

/// What the workers need to color the columns. Whenever any of this changes,
/// it gets a new `id`.
pub struct Params {
  pub id:      u64,
  pub splines: Splines,
  pub layer:   Layer,
}

/// One chunk to generate.
pub struct Job {
  pub epoch: u64,
  pub noise: Arc<TerrainNoise>,
  pub world: Arc<WorldBiomes>,
  pub real:  Arc<RealTerrain>,
  /// The chunk's position, in chunks.
  pub tile:  [i32; 2],
}

pub struct TileResult {
  pub epoch:     u64,
  pub tile:      [i32; 2],
  /// `TILE_BLOCKS * TILE_BLOCKS` samples, with x changing fastest.
  pub raw:       Vec<RawNoise>,
  /// The color of the biome of each column, as `0xrrggbb`, in the same order
  /// as `raw`.
  pub biomes:    Vec<u32>,
  /// The color of each column, made with the params with this id.
  pub params_id: u64,
  pub colors:    Vec<Color32>,
  /// The blocks, made with the generator.
  pub blocks:    TileBlocks,
}

struct Queue {
  jobs:   Mutex<Vec<Job>>,
  ready:  Condvar,
  /// Jobs from any other epoch are stale, and are skipped.
  epoch:  AtomicU64,
  /// What new jobs are colored with.
  params: Mutex<Arc<Params>>,
}

pub struct Pool {
  queue: Arc<Queue>,
}

impl Pool {
  pub fn new(ctx: &egui::Context, results: mpsc::Sender<TileResult>, params: Arc<Params>) -> Pool {
    let queue = Arc::new(Queue {
      jobs:   Mutex::new(vec![]),
      ready:  Condvar::new(),
      epoch:  AtomicU64::new(0),
      params: Mutex::new(params),
    });

    let threads = std::thread::available_parallelism().map_or(1, |t| t.get());
    for _ in 0..threads {
      let queue = queue.clone();
      let results = results.clone();
      let ctx = ctx.clone();

      std::thread::spawn(move || {
        loop {
          let job = {
            let mut jobs = queue.jobs.lock().unwrap();
            loop {
              if let Some(job) = jobs.pop() {
                break job;
              }
              jobs = queue.ready.wait(jobs).unwrap();
            }
          };

          if job.epoch != queue.epoch.load(Ordering::SeqCst) {
            continue;
          }

          let min = job.tile.map(|t| f64::from(t) * TILE_BLOCKS as f64);
          let mut raw = Vec::with_capacity(TILE_BLOCKS * TILE_BLOCKS);
          let mut biomes = Vec::with_capacity(TILE_BLOCKS * TILE_BLOCKS);
          for j in 0..TILE_BLOCKS {
            for i in 0..TILE_BLOCKS {
              let x = min[0] + i as f64;
              let z = min[1] + j as f64;
              raw.push(job.noise.sample(x, z));
              // The top of the world, to pick the surface biome.
              biomes
                .push(job.world.choose_surface_biome(Pos::new(x as i32, 255, z as i32)).color());
            }
          }

          let params = queue.params.lock().unwrap().clone();
          let colors = Grid::tile_colors(&raw, &biomes, &params.splines, params.layer);

          let blocks = job.real.generate(&job.world, job.tile);

          let result = TileResult {
            epoch: job.epoch,
            tile: job.tile,
            raw,
            biomes,
            params_id: params.id,
            colors,
            blocks,
          };
          if results.send(result).is_err() {
            return;
          }
          ctx.request_repaint();
        }
      });
    }

    Pool { queue }
  }

  /// Changes what chunks that haven't started yet are colored with.
  pub fn set_params(&self, params: Arc<Params>) { *self.queue.params.lock().unwrap() = params; }

  /// Throws away every queued chunk, and starts a new epoch. Chunks that are
  /// being generated finish, but are ignored.
  pub fn reset(&self, epoch: u64) {
    self.queue.epoch.store(epoch, Ordering::SeqCst);
    self.queue.jobs.lock().unwrap().clear();
  }

  /// Drops the queued chunks that are further than `radius` chunks from
  /// `center`, and queues `new`. The chunks closest to the center are
  /// generated first.
  pub fn plan(
    &self,
    epoch: u64,
    noise: &Arc<TerrainNoise>,
    world: &Arc<WorldBiomes>,
    real: &Arc<RealTerrain>,
    center: [i32; 2],
    radius: i32,
    new: &[[i32; 2]],
  ) {
    let mut jobs = self.queue.jobs.lock().unwrap();
    jobs.retain(|job| job.epoch == epoch && crate::grid::in_range(center, radius, job.tile));
    jobs.extend(new.iter().map(|&tile| Job {
      epoch,
      noise: noise.clone(),
      world: world.clone(),
      real: real.clone(),
      tile,
    }));
    // The workers pop from the end, so put the closest chunks last.
    jobs.sort_by_key(|job| std::cmp::Reverse(distance_sq(center, job.tile)));
    drop(jobs);

    self.queue.ready.notify_all();
  }
}
