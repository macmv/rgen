use std::ops::RangeInclusive;

use super::{
  splotch::Footprint,
  util::{Blob, blobs_near_radius},
};
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};
use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

/// Organic deposits following the exposed underwater floor, before vegetation.
pub struct WaterbedPatches {
  pub replace:       BlockFilter,
  pub place:         BlockState,
  pub radius:        RangeInclusive<u8>,
  /// Total deposit thickness including the bed surface.
  pub depth:         RangeInclusive<u8>,
  /// Candidate patches clustered within eight blocks of each source center.
  pub attempts:      u32,
  pub avg_per_chunk: f64,
}

impl ChunkPlacer for WaterbedPatches {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    self.place_materials(chunk, rng, chunk_pos, &[]);
  }
}

struct MixedWaterbedPatches {
  patches: WaterbedPatches,
  materials: Vec<BlockState>,
}

impl ChunkPlacer for MixedWaterbedPatches {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    self.patches.place_materials(chunk, rng, chunk_pos, &self.materials);
  }
}

impl WaterbedPatches {
  /// Choose independently from the supplied materials inside each deposit.
  pub fn with_mix(self, materials: Vec<BlockState>) -> impl ChunkPlacer {
    assert!(!materials.is_empty());
    MixedWaterbedPatches { patches: self, materials }
  }

  fn place_materials(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos, materials: &[BlockState]) {
    if self.radius.is_empty()
      || self.depth.is_empty()
      || *self.depth.end() == 0
      || self.attempts == 0
    {
      return;
    }
    let mut floors = [None; 256];
    let mut scanned = [false; 256];
    // Include the candidate offset as well as the patch radius in source replay.
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
        self.place_blob_materials(chunk, chunk_pos, &blob, &mut floors, &mut scanned, materials);
      }
    }
  }
}

impl WaterbedPatches {
  #[cfg(test)]
  fn place_blob(
    &self, chunk: &mut BiomeCachedChunk, chunk_pos: ChunkPos, blob: &Blob,
    floors: &mut [Option<i32>; 256], scanned: &mut [bool; 256],
  ) {
    self.place_blob_materials(chunk, chunk_pos, blob, floors, scanned, &[]);
  }

  fn place_blob_materials(
    &self,
    chunk: &mut BiomeCachedChunk,
    chunk_pos: ChunkPos,
    blob: &Blob,
    floors: &mut [Option<i32>; 256],
    scanned: &mut [bool; 256],
    materials: &[BlockState],
  ) {
    let mut rng = Rng::new(blob.seed);
    let radius = rng.range(i32::from(*self.radius.start())..=i32::from(*self.radius.end()));
    let min = chunk_pos.min_block_pos();
    let x0 = (blob.x - radius).max(min.x);
    let x1 = (blob.x + radius).min(min.x + 15);
    let z0 = (blob.z - radius).max(min.z);
    let z1 = (blob.z + radius).min(min.z + 15);
    if x0 > x1 || z0 > z1 {
      return;
    }
    let shape = Footprint::new(&mut rng, radius);
    for x in x0..=x1 {
      for z in z0..=z1 {
        let mut rng = blob.column_rng(x, z);
        if !shape.contains(x - blob.x, z - blob.z, &mut rng) {
          continue;
        }
        let column = ChunkRelPos::new((x - min.x) as u8, 0, (z - min.z) as u8);
        let index = column.z() as usize * 16 + column.x() as usize;
        if !scanned[index] {
          floors[index] = exposed_water_floor(chunk, column);
          scanned[index] = true;
        }
        let Some(y) = floors[index] else {
          continue;
        };
        let floor = column.with_y(y);
        if !chunk.is_surface_active(floor) || chunk.get(floor.with_y(y + 1)) != block![water] {
          continue;
        }
        let depth = rng.range(i32::from(*self.depth.start())..=i32::from(*self.depth.end()));
        for d in 0..depth.min(y) {
          let pos = floor.with_y(y - d);
          let current = chunk.get(pos);
          if !chunk.is_surface_active(pos)
            || current == block![air]
            || current == block![water]
            || !self.replace.contains(current)
          {
            break;
          }
          chunk.set(pos, if materials.is_empty() { self.place } else { *rng.choose(materials) });
        }
      }
    }
  }
}

// Scan at most once per touched column. Do not decorate underground pools under
// dry land; these are river/lake bed deposits, not cave decorations.
fn exposed_water_floor(chunk: &BiomeCachedChunk, column: ChunkRelPos) -> Option<i32> {
  let top = (1..=255).rev().find(|&y| chunk.get(column.with_y(y)) != block![air])?;
  if chunk.get(column.with_y(top)) != block![water] {
    return None;
  }
  let mut y = top;
  while y > 0 && chunk.get(column.with_y(y)) == block![water] {
    y -= 1;
  }
  if y > 0 && chunk.get(column.with_y(y)) != block![air] { Some(y) } else { None }
}

#[cfg(test)]
mod tests {
  use super::{super::ground_tests::registry, *};
  use crate::TemporaryBiome;
  use rgen_base::Chunk;
  use rgen_world::BlockInfoSupplier;

  fn bed(info: &BlockInfoSupplier, slope: bool) -> Chunk {
    let mut chunk = Chunk::new();
    for x in 0..16 {
      for z in 0..16 {
        let floor = if slope { 56 + i32::from(x) / 2 } else { 60 };
        for y in floor - 3..=64 {
          chunk.set(
            ChunkRelPos::new(x, y, z),
            info.encode(if y <= floor { block![sand] } else { block![water] }),
          );
        }
      }
    }
    chunk
  }

  fn placer(depth: u8) -> WaterbedPatches {
    WaterbedPatches {
      replace:       block![sand].into(),
      place:         block![clay],
      radius:        5..=5,
      depth:         depth..=depth,
      attempts:      16,
      avg_per_chunk: 2.0,
    }
  }

  #[test]
  fn deposits_follow_sloping_bed_without_changing_water() {
    let info = registry();
    for depth in [1, 3] {
      let mut chunk = bed(&info, true);
      let original = chunk.clone();
      placer(depth).place(
        &mut BiomeCachedChunk::new(&info, &mut chunk),
        &mut Rng::new(42),
        ChunkPos::new(-1, 0),
      );
      assert_ne!(chunk.data(), original.data());
      let mut repeated = original.clone();
      placer(depth).place(
        &mut BiomeCachedChunk::new(&info, &mut repeated),
        &mut Rng::new(42),
        ChunkPos::new(-1, 0),
      );
      assert_eq!(chunk.data(), repeated.data());
      for x in 0..16 {
        for z in 0..16 {
          let floor = 56 + i32::from(x) / 2;
          for y in floor + 1..=64 {
            assert_eq!(chunk.get(ChunkRelPos::new(x, y, z)), info.encode(block![water]));
          }
          assert_eq!(
            chunk.get(ChunkRelPos::new(x, floor - i32::from(depth), z)),
            info.encode(block![sand])
          );
          for y in 65..256 {
            assert_eq!(chunk.get(ChunkRelPos::new(x, y, z)), rgen_base::StateId::AIR);
          }
        }
      }
    }
  }

  #[test]
  fn exposed_bed_below_cave_cutoff_uses_surface_biome() {
    let info = registry();
    let mut chunk = bed(&info, false);
    let blob = Blob { x: 8, z: 8, seed: 42 };
    let mut cached = BiomeCachedChunk::new(&info, &mut chunk);
    for x in 0..16 {
      for z in 0..16 {
        cached.set_column(ChunkRelPos::new(x, 0, z), crate::BiomeColumn {
          surface: TemporaryBiome(0),
          cave: TemporaryBiome(1),
          min_height: 63,
        });
      }
    }
    assert!(!cached.is_active(ChunkRelPos::new(8, 60, 8)));
    placer(1).place_blob(
      &mut cached, ChunkPos::new(0, 0), &blob,
      &mut [None; 256], &mut [false; 256],
    );
    assert_eq!(cached.get(ChunkRelPos::new(8, 60, 8)), block![clay]);
    assert_eq!(cached.get(ChunkRelPos::new(8, 61, 8)), block![water]);
  }

  #[test]
  fn underwater_blob_continues_across_negative_chunk_border() {
    let info = registry();
    let blob = Blob { x: 0, z: 8, seed: 42 };
    for (pos, edge_x) in [(ChunkPos::new(-1, 0), 15), (ChunkPos::new(0, 0), 0)] {
      let mut chunk = bed(&info, false);
      placer(1).place_blob(
        &mut BiomeCachedChunk::new(&info, &mut chunk),
        pos,
        &blob,
        &mut [None; 256],
        &mut [false; 256],
      );
      assert_eq!(chunk.get(ChunkRelPos::new(edge_x, 60, 8)), info.encode(block![clay]));
      assert_eq!(chunk.get(ChunkRelPos::new(edge_x, 61, 8)), info.encode(block![water]));
    }
  }

  #[test]
  fn skips_dry_covered_inactive_and_nonreplaceable_beds() {
    let info = registry();
    for mode in 0..4 {
      let mut chunk = bed(&info, false);
      for x in 0..16 {
        for z in 0..16 {
          if mode == 0 {
            for y in 61..=64 {
              chunk.set(ChunkRelPos::new(x, y, z), info.encode(block![air]));
            }
          }
          if mode == 1 {
            chunk.set(ChunkRelPos::new(x, 65, z), info.encode(block![stone]));
          }
          if mode == 2 {
            chunk.set(ChunkRelPos::new(x, 60, z), info.encode(block![stone]));
          }
        }
      }
      let original = chunk.clone();
      let mut cached = BiomeCachedChunk::new(&info, &mut chunk);
      if mode == 3 {
        cached.set_active(TemporaryBiome(1));
      }
      placer(3).place(&mut cached, &mut Rng::new(42), ChunkPos::new(0, 0));
      assert_eq!(chunk.data(), original.data());
    }
  }

  #[test]
  fn depth_stops_at_air_or_protected_material() {
    let info = registry();
    for barrier in [block![air], block![stone], block![water]] {
      let mut chunk = bed(&info, false);
      for x in 0..16 {
        for z in 0..16 {
          chunk.set(ChunkRelPos::new(x, 59, z), info.encode(barrier));
        }
      }
      placer(3).place(
        &mut BiomeCachedChunk::new(&info, &mut chunk),
        &mut Rng::new(42),
        ChunkPos::new(0, 0),
      );
      for x in 0..16 {
        for z in 0..16 {
          assert_eq!(chunk.get(ChunkRelPos::new(x, 59, z)), info.encode(barrier));
          assert_eq!(chunk.get(ChunkRelPos::new(x, 58, z)), info.encode(block![sand]));
        }
      }
    }
  }
}
