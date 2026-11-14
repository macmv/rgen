use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use super::util::{Blob, blobs_near_radius};
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

/// A terrain-following deposit with a lobed outline and explicit thickness.
pub struct Splotch {
  pub replace:       BlockFilter,
  pub below_replace: BlockFilter,
  pub place:         BlockState,
  pub avg_per_chunk: f64,
  /// Maximum horizontal reach from the patch center.
  pub radius:        RangeInclusive<u8>,
  /// Total thickness, including the surface. Stops at non-replaceable ground.
  pub depth:         RangeInclusive<u8>,
}

/// Uses the same footprint as Splotch, choosing a material for every block.
pub struct MixedSplotch {
  pub replace:       BlockFilter,
  pub below_replace: BlockFilter,
  pub place:         Vec<BlockState>,
  pub avg_per_chunk: f64,
  pub radius:        RangeInclusive<u8>,
  pub depth:         RangeInclusive<u8>,
}

impl Default for Splotch {
  fn default() -> Self {
    Self {
      replace:       block![grass].into(),
      below_replace: block![dirt].into(),
      place:         block![gravel],
      radius:        2..=5,
      depth:         1..=3,
      avg_per_chunk: 1.0,
    }
  }
}

impl ChunkPlacer for Splotch {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    place_patches(
      chunk,
      rng,
      chunk_pos,
      &self.replace,
      &self.below_replace,
      &[self.place],
      self.avg_per_chunk,
      &self.radius,
      &self.depth,
    );
  }
}

impl ChunkPlacer for MixedSplotch {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    place_patches(
      chunk,
      rng,
      chunk_pos,
      &self.replace,
      &self.below_replace,
      &self.place,
      self.avg_per_chunk,
      &self.radius,
      &self.depth,
    );
  }
}

// All lobes overlap the center. Rotation and unequal axes avoid repeated
// circles; column-seeded nibbling only affects the outer rim, not the solid
// interior.
pub(super) struct Footprint {
  lobes:  [(f64, f64, f64, f64); 4],
  sin:    f64,
  cos:    f64,
  radius: i32,
}

impl Footprint {
  pub(super) fn new(rng: &mut Rng, radius: i32) -> Self {
    let (sin, cos) = rng.range(0.0..std::f64::consts::TAU).sin_cos();
    let r = f64::from(radius.max(1));
    let lobes = std::array::from_fn(|_| {
      (
        rng.range(-0.25..0.25) * r,
        rng.range(-0.25..0.25) * r,
        rng.range(0.5..0.9) * r,
        rng.range(0.45..0.8) * r,
      )
    });
    Self { lobes, sin, cos, radius }
  }

  pub(super) fn contains(&self, dx: i32, dz: i32, rng: &mut Rng) -> bool {
    if dx.abs() > self.radius || dz.abs() > self.radius {
      return false;
    }
    if dx == 0 && dz == 0 {
      return true;
    }
    let x = f64::from(dx) * self.cos - f64::from(dz) * self.sin;
    let z = f64::from(dx) * self.sin + f64::from(dz) * self.cos;
    let distance = self
      .lobes
      .iter()
      .map(|&(cx, cz, rx, rz)| ((x - cx) / rx).powi(2) + ((z - cz) / rz).powi(2))
      .fold(f64::INFINITY, f64::min);
    distance <= 1.0 && (distance < 0.8 || rng.range(0..4) != 0)
  }
}

#[allow(clippy::too_many_arguments)]
fn place_patches(
  chunk: &mut BiomeCachedChunk,
  rng: &mut Rng,
  chunk_pos: ChunkPos,
  replace: &BlockFilter,
  below_replace: &BlockFilter,
  materials: &[BlockState],
  avg: f64,
  radius: &RangeInclusive<u8>,
  depth: &RangeInclusive<u8>,
) {
  if materials.is_empty() || radius.is_empty() || depth.is_empty() || *depth.end() == 0 {
    return;
  }
  for blob in blobs_near_radius(rng.next(), chunk_pos, avg, i32::from(*radius.end())) {
    place_blob(chunk, chunk_pos, &blob, replace, below_replace, materials, radius, depth);
  }
}

#[allow(clippy::too_many_arguments)]
fn place_blob(
  chunk: &mut BiomeCachedChunk,
  chunk_pos: ChunkPos,
  blob: &Blob,
  replace: &BlockFilter,
  below_replace: &BlockFilter,
  materials: &[BlockState],
  radius: &RangeInclusive<u8>,
  depth: &RangeInclusive<u8>,
) {
  let mut shape_rng = Rng::new(blob.seed);
  let r = shape_rng.range(i32::from(*radius.start())..=i32::from(*radius.end()));
  let shape = Footprint::new(&mut shape_rng, r);
  let min = chunk_pos.min_block_pos();
  // Visit only the footprint's intersection with this chunk.
  for x in (blob.x - r).max(min.x)..=(blob.x + r).min(min.x + 15) {
    for z in (blob.z - r).max(min.z)..=(blob.z + r).min(min.z + 15) {
      let mut column_rng = blob.column_rng(x, z);
      if !shape.contains(x - blob.x, z - blob.z, &mut column_rng) {
        continue;
      }
      let column = ChunkRelPos::new((x - min.x) as u8, 0, (z - min.z) as u8);
      // Follow each terrain surface; never depend on the center's elevation.
      for i in 0..chunk.chunk.surfaces(column).len() {
        let y = i32::from(chunk.chunk.surfaces(column)[i]);
        if !(1..=254).contains(&y) {
          continue;
        }
        let surface = column.with_y(y);
        if !chunk.is_active(surface)
          || !replace.contains(chunk.get(surface))
          || chunk.get(surface.with_y(y + 1)) != block![air]
        {
          continue;
        }
        let thickness = column_rng.range(i32::from(*depth.start())..=i32::from(*depth.end()));
        for d in 0..thickness.min(y) {
          let at = column.with_y(y - d);
          if !chunk.is_active(at) || (d > 0 && !below_replace.contains(chunk.get(at))) {
            break;
          }
          chunk.set(at, *column_rng.choose(materials));
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{
    super::ground_tests::{registry, terrain},
    *,
  };
  use crate::TemporaryBiome;

  #[test]
  fn deposit_follows_slope_and_stops_at_substrate() {
    let info = registry();
    let mut chunk = rgen_base::Chunk::new();
    for x in 0..16 {
      for z in 0..16 {
        let y = 60 + i32::from(x) / 2;
        let p = ChunkRelPos::new(x, y, z);
        chunk.add_surface(p);
        chunk.set(p, info.encode(block![grass]));
        chunk.set(p.with_y(y - 1), info.encode(block![dirt]));
        chunk.set(p.with_y(y - 2), info.encode(block![stone]));
        // Buried dirt must not be reached through the intervening stone.
        chunk.set(p.with_y(y - 3), info.encode(block![dirt]));
      }
    }
    let blob = Blob { x: 8, z: 8, seed: 42 };
    let surface: BlockFilter = block![grass].into();
    let below: BlockFilter = block![dirt].into();
    place_blob(
      &mut BiomeCachedChunk::new(&info, &mut chunk),
      ChunkPos::new(0, 0),
      &blob,
      &surface,
      &below,
      &[block![sand]],
      &(7..=7),
      &(3..=3),
    );
    let mut changed = 0;
    for x in 0..16 {
      for z in 0..16 {
        let y = 60 + i32::from(x) / 2;
        let p = ChunkRelPos::new(x, y, z);
        if chunk.get(p) == info.encode(block![sand]) {
          changed += 1;
          assert_eq!(chunk.get(p.with_y(y - 1)), info.encode(block![sand]));
        }
        assert_eq!(chunk.get(p.with_y(y - 2)), info.encode(block![stone]));
        assert_eq!(chunk.get(p.with_y(y - 3)), info.encode(block![dirt]));
        assert_eq!(chunk.get(p.with_y(y + 1)), rgen_base::StateId::AIR);
      }
    }
    assert!(changed > 10);
  }

  #[test]
  fn mixed_patch_crosses_negative_chunk_border_and_replays_identically() {
    let info = registry();
    let blob = Blob { x: 0, z: 8, seed: 42 };
    let surface: BlockFilter = block![grass].into();
    let below: BlockFilter = block![dirt].into();
    let materials = [block![sand], block![gravel]];
    let render = |pos| {
      let mut chunk = terrain(&info, 64);
      place_blob(
        &mut BiomeCachedChunk::new(&info, &mut chunk),
        pos,
        &blob,
        &surface,
        &below,
        &materials,
        &(7..=7),
        &(3..=3),
      );
      chunk
    };
    let left = render(ChunkPos::new(-1, 0));
    let right = render(ChunkPos::new(0, 0));
    assert_eq!(right.data(), render(ChunkPos::new(0, 0)).data());
    assert_eq!(left.data(), render(ChunkPos::new(-1, 0)).data());
    for (chunk, x) in [(&left, 15), (&right, 0)] {
      assert_ne!(chunk.get(ChunkRelPos::new(x, 64, 8)), info.encode(block![grass]));
      assert!(chunk.data().contains(&info.encode(block![sand]).0));
      assert!(chunk.data().contains(&info.encode(block![gravel]).0));
      for x in 0..16 {
        for z in 0..16 {
          assert_eq!(chunk.get(ChunkRelPos::new(x, 61, z)), info.encode(block![dirt]));
        }
      }
    }
  }

  #[test]
  fn patches_respect_biomes_exposure_and_zero_depth() {
    let info = registry();
    let placer = Splotch { avg_per_chunk: 20.0, ..Splotch::default() };
    for mode in 0..4 {
      let mut chunk = terrain(&info, 64);
      if mode == 1 {
        for x in 0..16 {
          for z in 0..16 {
            // A covered surface is not an exposed deposit site.
            chunk.set(ChunkRelPos::new(x, 65, z), info.encode(block![stone]));
          }
        }
      }
      let original = chunk.clone();
      let mut cached = BiomeCachedChunk::new(&info, &mut chunk);
      if mode == 0 {
        cached.set_active(TemporaryBiome(1));
      }
      match mode {
        2 => Splotch { depth: 0..=0, ..Splotch::default() }.place(
          &mut cached,
          &mut Rng::new(1),
          ChunkPos::new(0, 0),
        ),
        3 => MixedSplotch {
          replace:       block![grass].into(),
          below_replace: block![dirt].into(),
          place:         vec![],
          avg_per_chunk: 20.0,
          radius:        2..=5,
          depth:         1..=3,
        }
        .place(&mut cached, &mut Rng::new(1), ChunkPos::new(0, 0)),
        _ => placer.place(&mut cached, &mut Rng::new(1), ChunkPos::new(0, 0)),
      }
      assert_eq!(chunk.data(), original.data());
    }
  }

  #[test]
  fn organic_shapes_keep_core_and_bounds_but_vary_outline() {
    let mut outlines = std::collections::HashSet::new();
    for seed in 0..20 {
      let shape = Footprint::new(&mut Rng::new(seed), 6);
      assert!(shape.contains(0, 0, &mut Rng::new(seed)));
      assert!(!shape.contains(7, 0, &mut Rng::new(seed)));
      let mut outline = Vec::new();
      for x in -6..=6 {
        for z in -6..=6 {
          outline.push(shape.contains(x, z, &mut Rng::new(seed)));
        }
      }
      outlines.insert(outline);
    }
    assert!(outlines.len() > 15);
  }
}
