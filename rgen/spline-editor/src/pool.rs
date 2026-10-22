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

use crate::grid::TILES;

/// One chunk to generate.
pub struct Job {
  pub epoch: u64,
  pub noise: Arc<TerrainNoise>,
  pub tx:    usize,
  pub tz:    usize,
  /// The world position of the chunk's minimum corner.
  pub min:   [f64; 2],
  pub step:  f64,
  pub cols:  usize,
}

pub struct TileResult {
  pub epoch: u64,
  pub tx:    usize,
  pub tz:    usize,
  /// `cols * cols` samples, with x changing fastest.
  pub raw:   Vec<RawNoise>,
}

struct Queue {
  jobs:  Mutex<Vec<Job>>,
  ready: Condvar,
  /// Jobs from any other epoch are stale, and are skipped.
  epoch: AtomicU64,
}

pub struct Pool {
  queue: Arc<Queue>,
}

impl Pool {
  pub fn new(ctx: &egui::Context, results: mpsc::Sender<TileResult>) -> Pool {
    let queue =
      Arc::new(Queue { jobs: Mutex::new(vec![]), ready: Condvar::new(), epoch: AtomicU64::new(0) });

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

          let mut raw = Vec::with_capacity(job.cols * job.cols);
          for j in 0..job.cols {
            for i in 0..job.cols {
              let x = job.min[0] + i as f64 * job.step;
              let z = job.min[1] + j as f64 * job.step;
              raw.push(job.noise.sample(x, z));
            }
          }

          if results.send(TileResult { epoch: job.epoch, tx: job.tx, tz: job.tz, raw }).is_err() {
            return;
          }
          ctx.request_repaint();
        }
      });
    }

    Pool { queue }
  }

  /// Throws away any queued chunks, and queues every chunk of a new region.
  /// The chunks closest to the middle are generated first.
  pub fn start_region(&self, epoch: u64, noise: &Arc<TerrainNoise>, origin: [f64; 2], step: f64, cols: usize) {
    let tile_blocks = cols as f64 * step;
    let middle = TILES as i32 / 2;

    let mut tiles: Vec<(usize, usize)> =
      (0..TILES).flat_map(|tz| (0..TILES).map(move |tx| (tx, tz))).collect();
    // The workers pop from the end, so put the closest chunks last.
    tiles.sort_by_key(|&(tx, tz)| {
      let (dx, dz) = (tx as i32 - middle, tz as i32 - middle);
      std::cmp::Reverse((dx.abs().max(dz.abs()), dx * dx + dz * dz))
    });

    let jobs = tiles
      .into_iter()
      .map(|(tx, tz)| Job {
        epoch,
        noise: noise.clone(),
        tx,
        tz,
        min: [origin[0] + tx as f64 * tile_blocks, origin[1] + tz as f64 * tile_blocks],
        step,
        cols,
      })
      .collect();

    self.queue.epoch.store(epoch, Ordering::SeqCst);
    *self.queue.jobs.lock().unwrap() = jobs;
    self.queue.ready.notify_all();
  }
}
