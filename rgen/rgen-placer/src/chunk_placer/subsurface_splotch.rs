use std::ops::RangeInclusive;

use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use super::util::blobs_near_radius;
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

/// A small surface deposit with a stone-rich substrate beneath it.
pub struct SubsurfaceSplotch {
  pub surface_replace: BlockFilter,
  pub below_replace:   BlockFilter,
  pub place:           BlockState,
  pub below_mix:       Vec<BlockState>,
  pub radius:          RangeInclusive<u8>,
  pub depth:           RangeInclusive<u8>,
  pub avg_per_chunk:   f64,
  /// Chance that the patch stretches along one horizontal axis, from 0 to 100.
  pub stretch_chance:  u8,
}

impl ChunkPlacer for SubsurfaceSplotch {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    if self.below_mix.is_empty() {
      return;
    }

    let min = chunk_pos.min_block_pos();
    for blob in blobs_near_radius(
      rng.next(),
      chunk_pos,
      self.avg_per_chunk,
      i32::from(*self.radius.end()) * 2,
    ) {
      let mut rng = Rng::new(blob.seed);
      let radius = rng.range(*self.radius.start() as i32..=*self.radius.end() as i32);
      let stretched = rng.range(0..100) < i32::from(self.stretch_chance);
      let stretch_x = stretched && rng.range(0..2) == 0;
      let stretch_z = stretched && !stretch_x;
      let radius_x = radius * if stretch_x { 2 } else { 1 };
      let radius_z = radius * if stretch_z { 2 } else { 1 };
      let radius_product = radius_x * radius_z;

      for dx in (-radius_x).max(min.x - blob.x)..=radius_x.min(min.x + 15 - blob.x) {
        for dz in (-radius_z).max(min.z - blob.z)..=radius_z.min(min.z + 15 - blob.z) {
          let ellipse_distance = dx * dx * radius_z * radius_z + dz * dz * radius_x * radius_x;
          if ellipse_distance > radius_product * radius_product {
            continue;
          }

          let x = blob.x + dx;
          let z = blob.z + dz;
          let (rx, rz) = (x - min.x, z - min.z);
          if !(0..16).contains(&rx) || !(0..16).contains(&rz) {
            continue;
          }
          let column = ChunkRelPos::new(rx as u8, 0, rz as u8);
          let mut rng = blob.column_rng(x, z);
          if (dx != 0 || dz != 0) && rng.range(0..5) == 0 {
            continue;
          }
          let Some(surface_y) = chunk.chunk.surfaces(column).iter().copied().max().map(i32::from)
          else {
            continue;
          };
          if !(1..=254).contains(&surface_y) {
            continue;
          }

          let surface = column.with_y(surface_y);
          if !chunk.is_active(surface) || !self.surface_replace.contains(chunk.get(surface)) {
            continue;
          }

          let deposit_depth = rng.range(*self.depth.start() as i32..=*self.depth.end() as i32);
          chunk.set(surface, self.place);
          for layer in 1..deposit_depth.min(surface_y) {
            let layer_pos = surface.with_y(surface_y - layer);
            if !self.below_replace.contains(chunk.get(layer_pos)) {
              break;
            }
            chunk.set(layer_pos, self.place);
          }

          // Turn the remaining dirt column beneath the deposit into the supplied
          // stone palette, stopping when the natural dirt layer ends.
          let mut layer = deposit_depth;
          loop {
            if surface_y - layer <= 0 {
              break;
            }
            let layer_pos = surface.with_y(surface_y - layer);
            if layer_pos.y() <= 0 || !self.below_replace.contains(chunk.get(layer_pos)) {
              break;
            }
            chunk.set(layer_pos, *rng.choose(&self.below_mix));
            layer += 1;
          }
        }
      }
    }
  }
}
