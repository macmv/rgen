//! Generates the chunks of terrain on a pool of threads, the same way
//! `rgen-viewer` does: the main thread queues up chunks nearest to the center
//! first, and the workers pull them off as they get free.

use std::sync::{
  Arc, Condvar, Mutex,
  atomic::{AtomicU64, Ordering},
  mpsc,
};

use eframe::egui;
use rgen_biome::{RawNoise, TerrainNoise};

use crate::{
  Splines,
  grid::{Grid, Layer, TILE_BLOCKS, TileColumns},
};

/// What the workers need to turn noise into heights and colors. Whenever any of
/// this changes, it gets a new `id`.
pub struct Params {
  pub id:            u64,
  pub splines:       Splines,
  pub layer:         Layer,
  pub flatten_water: bool,
}

/// One chunk to generate.
pub struct Job {
  pub epoch: u64,
  pub noise: Arc<TerrainNoise>,
  pub tx:    usize,
  pub tz:    usize,
  /// The world position of the chunk's minimum corner.
  pub min:   [f64; 2],
}

pub struct TileResult {
  pub epoch: u64,
  pub tx:    usize,
  pub tz:    usize,
  /// `TILE_BLOCKS * TILE_BLOCKS` samples, with x changing fastest.
  pub raw:   Vec<RawNoise>,
  /// The columns, made with the params with this id.
  pub params_id: u64,
  pub columns:   TileColumns,
}

struct Queue {
  jobs:  Mutex<Vec<Job>>,
  ready: Condvar,
  /// Jobs from any other epoch are stale, and are skipped.
  epoch: AtomicU64,
  /// What new jobs are made with.
  params: Mutex<Arc<Params>>,
}

pub struct Pool {
  queue: Arc<Queue>,
}

impl Pool {
  pub fn new(ctx: &egui::Context, results: mpsc::Sender<TileResult>, params: Arc<Params>) -> Pool {
    let queue = Arc::new(Queue {
      jobs:  Mutex::new(vec![]),
      ready: Condvar::new(),
      epoch: AtomicU64::new(0),
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

          let mut raw = Vec::with_capacity(TILE_BLOCKS * TILE_BLOCKS);
          for j in 0..TILE_BLOCKS {
            for i in 0..TILE_BLOCKS {
              let x = job.min[0] + i as f64;
              let z = job.min[1] + j as f64;
              raw.push(job.noise.sample(x, z));
            }
          }

          let params = queue.params.lock().unwrap().clone();
          let columns =
            Grid::tile_columns(&raw, &params.splines, params.layer, params.flatten_water);

          let result = TileResult {
            epoch: job.epoch,
            tx: job.tx,
            tz: job.tz,
            raw,
            params_id: params.id,
            columns,
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

  /// Changes what chunks that haven't started yet are made with.
  pub fn set_params(&self, params: Arc<Params>) { *self.queue.params.lock().unwrap() = params; }

  /// Throws away any queued chunks, and queues every chunk of a new region.
  /// The chunks closest to the middle are generated first.
  pub fn start_region(
    &self,
    epoch: u64,
    noise: &Arc<TerrainNoise>,
    origin: [f64; 2],
    tiles: [usize; 2],
  ) {
    let middle = [tiles[0] as i32 / 2, tiles[1] as i32 / 2];

    let mut jobs_at: Vec<(usize, usize)> =
      (0..tiles[1]).flat_map(|tz| (0..tiles[0]).map(move |tx| (tx, tz))).collect();
    // The workers pop from the end, so put the closest chunks last.
    jobs_at.sort_by_key(|&(tx, tz)| {
      let (dx, dz) = (tx as i32 - middle[0], tz as i32 - middle[1]);
      std::cmp::Reverse((dx.abs().max(dz.abs()), dx * dx + dz * dz))
    });

    let jobs = jobs_at
      .into_iter()
      .map(|(tx, tz)| Job {
        epoch,
        noise: noise.clone(),
        tx,
        tz,
        min: [origin[0] + (tx * TILE_BLOCKS) as f64, origin[1] + (tz * TILE_BLOCKS) as f64],
      })
      .collect();

    self.queue.epoch.store(epoch, Ordering::SeqCst);
    *self.queue.jobs.lock().unwrap() = jobs;
    self.queue.ready.notify_all();
  }
}
