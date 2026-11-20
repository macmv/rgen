use std::ops::RangeInclusive;

use super::{
  splotch::Footprint,
  util::{Blob, blobs_near_radius},
};
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};
use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

/// Organic shoreline patches. Only water inside this chunk qualifies: nearby
/// water across a chunk boundary is deliberately ignored, never guessed.
pub struct ShorelineSplotch {
  pub replace:       BlockFilter,
  pub place:         Vec<BlockState>,
  pub radius:        RangeInclusive<u8>,
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

impl ChunkPlacer for ShorelineSplotch {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    if self.place.is_empty() || self.radius.is_empty() || self.attempts == 0 {
      return;
    }
    // Find eligible exposed shoreline once, rather than repeating the water
    // search for every overlapping patch attempt.
    let mut shoreline = [None; 256];
    for x in 0..16 {
      for z in 0..16 {
        let column = ChunkRelPos::new(x, 0, z);
        let Some(&y) = chunk.chunk.surfaces(column).first() else {
          continue;
        };
        let ground = column.with_y(i32::from(y));
        if y > 0
          && y < 255
          && chunk.is_active(ground)
          && self.replace.contains(chunk.get(ground))
          && chunk.get(ground.with_y(i32::from(y) + 1)) == block![air]
          && near_water(chunk, ground)
        {
          shoreline[z as usize * 16 + x as usize] = Some(i32::from(y));
        }
      }
    }
    if shoreline.iter().all(Option::is_none) {
      return;
    }
    let min = chunk_pos.min_block_pos();
    for source in blobs_near_radius(
      rng.next(),
      chunk_pos,
      self.avg_per_chunk,
      8 + i32::from(*self.radius.end()),
    ) {
      let mut source_rng = Rng::new(source.seed);
      for _ in 0..self.attempts {
        let blob = Blob {
          x:    source.x + source_rng.range(-8..=8),
          z:    source.z + source_rng.range(-8..=8),
          seed: source_rng.next(),
        };
        let mut shape_rng = Rng::new(blob.seed);
        let radius =
          shape_rng.range(i32::from(*self.radius.start())..=i32::from(*self.radius.end()));
        let x0 = (blob.x - radius).max(min.x);
        let x1 = (blob.x + radius).min(min.x + 15);
        let z0 = (blob.z - radius).max(min.z);
        let z1 = (blob.z + radius).min(min.z + 15);
        if x0 > x1 || z0 > z1 {
          continue;
        }
        let shape = Footprint::new(&mut shape_rng, radius);
        for x in x0..=x1 {
          for z in z0..=z1 {
            let rx = (x - min.x) as u8;
            let rz = (z - min.z) as u8;
            let Some(y) = shoreline[rz as usize * 16 + rx as usize] else {
              continue;
            };
            let pos = ChunkRelPos::new(rx, y, rz);
            let mut column_rng = blob.column_rng(x, z);
            if self.replace.contains(chunk.get(pos))
              && shape.contains(x - blob.x, z - blob.z, &mut column_rng)
            {
              chunk.set(pos, *column_rng.choose(&self.place));
            }
          }
        }
      }
    }
  }
}

/// Existing two-block horizontal radius, with water within one level of ground.
fn near_water(chunk: &BiomeCachedChunk, ground: ChunkRelPos) -> bool {
  for dx in -2_i32..=2 {
    for dz in -2_i32..=2 {
      if (dx == 0 && dz == 0) || dx * dx + dz * dz > 4 {
        continue;
      }
      let x = i32::from(ground.x()) + dx;
      let z = i32::from(ground.z()) + dz;
      if !(0..16).contains(&x) || !(0..16).contains(&z) {
        continue;
      }
      for dy in -1..=1 {
        let y = ground.y() + dy;
        if (0..256).contains(&y)
          && chunk.get(ChunkRelPos::new(x as u8, y, z as u8)) == block![water]
        {
          return true;
        }
      }
    }
  }
  false
}

#[cfg(test)]
mod tests {
  use super::{
    super::ground_tests::{registry, terrain},
    *,
  };
  use crate::TemporaryBiome;

  #[test]
  fn water_checks_stop_at_chunk_edges_and_keep_distance_rules() {
    let info = registry();
    let mut chunk = terrain(&info, 64);
    chunk.set(ChunkRelPos::new(15, 64, 8), info.encode(block![water]));
    let cached = BiomeCachedChunk::new(&info, &mut chunk);
    assert!(!near_water(&cached, ChunkRelPos::new(0, 64, 8)));
    assert!(near_water(&cached, ChunkRelPos::new(14, 64, 8)));
    assert!(near_water(&cached, ChunkRelPos::new(13, 64, 8)));
    assert!(!near_water(&cached, ChunkRelPos::new(12, 64, 8)));
    assert!(!near_water(&cached, ChunkRelPos::new(13, 64, 9)));
    assert!(near_water(&cached, ChunkRelPos::new(14, 65, 8)));
    assert!(!near_water(&cached, ChunkRelPos::new(14, 66, 8)));
  }

  #[test]
  fn places_only_exposed_active_shoreline_and_keeps_water() {
    let info = registry();
    let placer = ShorelineSplotch {
      replace:       block![grass].into(),
      place:         vec![block![sand]],
      radius:        2..=4,
      attempts:      100,
      avg_per_chunk: 4.0,
    };
    for mode in 0..4 {
      let mut chunk = terrain(&info, 64);
      if mode != 0 {
        for z in 0..16 {
          chunk.set(ChunkRelPos::new(8, 64, z), info.encode(block![water]));
        }
      }
      if mode == 2 {
        for x in 0..16 {
          for z in 0..16 {
            chunk.set(ChunkRelPos::new(x, 65, z), info.encode(block![stone]));
          }
        }
      }
      let original = chunk.clone();
      let mut cached = BiomeCachedChunk::new(&info, &mut chunk);
      if mode == 3 {
        cached.set_active(TemporaryBiome(1));
      }
      placer.place(&mut cached, &mut Rng::new(42), ChunkPos::new(-1, 0));
      if mode != 1 {
        assert_eq!(chunk.data(), original.data());
        continue;
      }
      assert_ne!(chunk.data(), original.data());
      let mut repeated = original.clone();
      placer.place(
        &mut BiomeCachedChunk::new(&info, &mut repeated),
        &mut Rng::new(42),
        ChunkPos::new(-1, 0),
      );
      assert_eq!(chunk.data(), repeated.data());
      for x in 0..16 {
        for z in 0..16 {
          let p = ChunkRelPos::new(x, 64, z);
          if chunk.get(p) == info.encode(block![sand]) {
            assert!((6..=10).contains(&x));
          }
          if x == 8 {
            assert_eq!(chunk.get(p), info.encode(block![water]));
          }
          assert_eq!(chunk.get(p.with_y(63)), original.get(p.with_y(63)));
        }
      }
    }
  }
}
