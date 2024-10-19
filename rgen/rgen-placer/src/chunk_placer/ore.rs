use rgen_base::{BlockState, Pos};
use std::ops::RangeInclusive;

use crate::{grid::PointGrid, ChunkPlacer, Random, Rng};

pub struct Ore {
  pub ore:           BlockState,
  pub avg_per_chunk: f64,

  pub size:   RangeInclusive<i32>,
  pub height: RangeInclusive<i32>,
}

impl ChunkPlacer for Ore {
  fn place(
    &self,
    chunk: &mut crate::BiomeCachedChunk,
    rng: &mut crate::Rng,
    chunk_pos: rgen_base::ChunkPos,
  ) {
    let radius = self.size.end() / 2;
    let scale = 16.0;

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
      let mut pos = Pos::new((point.0 * scale) as i32, 0, (point.1 * scale) as i32);
      let vein_seed = seed ^ (((pos.x() * 2048) as u64) << 8) ^ (((pos.z() * 2048) as u64) << 16);
      let mut rng = Rng::new(vein_seed);

      pos.y = rng.rand_inclusive(*self.height.start(), *self.height.end());

      info!("placing ore at {:?}", pos);

      let size = rng.rand_inclusive(*self.size.start(), *self.size.end());
      for _ in 0..size {
        if pos.in_chunk(chunk_pos) {
          let rel = pos.chunk_rel();
          chunk.set(rel, self.ore);
        }

        match rng.rand_inclusive(0, 2) {
          0 => pos.x += rng.rand_inclusive(-1, 1),
          1 => pos.y += rng.rand_inclusive(-1, 1),
          _ => pos.z += rng.rand_inclusive(-1, 1),
        }
      }
    }
  }
}
