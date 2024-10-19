use rgen_base::{BlockState, Pos};
use std::ops::RangeInclusive;

use crate::{grid::PointGrid, ChunkPlacer, Random, Rng};

pub struct Ore {
  pub ore:           BlockState,
  pub avg_per_chunk: f64,

  pub size:   RangeInclusive<i32>,
  pub height: RangeInclusive<i32>,
  pub width:  f64,
}

impl ChunkPlacer for Ore {
  fn place(
    &self,
    chunk: &mut crate::BiomeCachedChunk,
    rng: &mut crate::Rng,
    chunk_pos: rgen_base::ChunkPos,
  ) {
    let radius = self.size.end();
    let scale = 16.0 / self.avg_per_chunk;

    let min_pos = chunk_pos.min_block_pos();
    let ore_min_x = ((min_pos.x - radius) as f64) / scale;
    let ore_min_z = ((min_pos.z - radius) as f64) / scale;
    let ore_max_x = ((min_pos.x + 16 + radius) as f64) / scale;
    let ore_max_z = ((min_pos.z + 16 + radius) as f64) / scale;

    // FIXME: This is always given the same random number generator, so we can pick
    // a seed off it by just grabbing the next number. This can't be relied on
    // though! Need to rework the `ChunkPlacer` api a bit.
    let seed = rng.next();
    let points = PointGrid.points_in_area(seed, ore_min_x, ore_min_z, ore_max_x, ore_max_z);

    for point in points {
      let mut pos = (point.0 * scale, 0.0, point.1 * scale);
      let vein_seed = seed ^ (((pos.0 * 2048.0) as u64) << 8) ^ (((pos.2 * 2048.0) as u64) << 16);
      let mut rng = Rng::new(vein_seed);

      pos.1 = rng.rand_inclusive(*self.height.start(), *self.height.end()) as f64;

      let mut vx = rng.rand_inclusive(-100, 100) as f64 / 100.0;
      let mut vy = rng.rand_inclusive(-100, 100) as f64 / 100.0;
      let mut vz = rng.rand_inclusive(-100, 100) as f64 / 100.0;

      let size = rng.rand_inclusive(*self.size.start(), *self.size.end());
      for _ in 0..size {
        for dx in -self.width.ceil() as i32..=self.width.ceil() as i32 {
          for dy in -self.width.ceil() as i32..=self.width.ceil() as i32 {
            for dz in -self.width.ceil() as i32..=self.width.ceil() as i32 {
              let dist = (dx * dx + dy * dy + dz * dz) as f64;
              if dist > self.width * self.width {
                continue;
              }

              let p = Pos::new(pos.0 as i32 + dx, pos.1 as i32 + dy, pos.2 as i32 + dz);

              if p.in_chunk(chunk_pos) {
                let rel = p.chunk_rel();
                chunk.set(rel, self.ore);
              }
            }
          }
        }

        vx += rng.rand_inclusive(-50, 50) as f64 / 100.0;
        vy += rng.rand_inclusive(-50, 50) as f64 / 100.0;
        vz += rng.rand_inclusive(-50, 50) as f64 / 100.0;

        pos.0 += vx;
        pos.1 += vy;
        pos.2 += vz;
      }
    }
  }
}
