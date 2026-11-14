use rgen_base::{BlockState, ChunkPos, ChunkRelPos, block};

use super::util::blobs_near_radius;
use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};

/// Makes a small, irregular rock outcrop with a gravel cap and a shallow
/// mixed-rock layer beneath it.
pub struct RockSurface {
  pub avg_per_chunk: f64,
  /// Width and length of the patch footprint in blocks.
  pub size:          u8,
  /// Materials mixed into the surface and the stone layer below it.
  pub mix_in:        Vec<BlockState>,
}

impl ChunkPlacer for RockSurface {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    if self.mix_in.is_empty() {
      return;
    }

    let mut stone_materials = Vec::with_capacity(self.mix_in.len() + 1);
    stone_materials.push(block![stone]);
    stone_materials.extend(self.mix_in.iter().copied());
    let min = chunk_pos.min_block_pos();
    for blob in
      blobs_near_radius(rng.next(), chunk_pos, self.avg_per_chunk, i32::from(self.size / 2) + 1)
    {
      let mut rng = Rng::new(blob.seed);
      let half_size = (self.size / 2) as i32;
      let center_limit = (half_size / 3).max(1);
      let lobe_radius = (half_size - 1).max(2);

      // A short, wandering chain of overlapping circles makes a connected,
      // lumpy footprint close to the configured width and length.
      let mut lobes = Vec::with_capacity(6);
      let mut cx = rng.range(-center_limit..=center_limit);
      let mut cz = rng.range(-center_limit..=center_limit);
      for _ in 0..6 {
        lobes.push((cx, cz, rng.range(2..=lobe_radius)));
        cx = (cx + rng.range(-2..=2)).clamp(-center_limit, center_limit);
        cz = (cz + rng.range(-2..=2)).clamp(-center_limit, center_limit);
      }

      for dx in (-half_size).max(min.x - blob.x)
        ..(half_size + i32::from(self.size % 2)).min(min.x + 16 - blob.x)
      {
        for dz in (-half_size).max(min.z - blob.z)
          ..(half_size + i32::from(self.size % 2)).min(min.z + 16 - blob.z)
        {
          let inside = lobes.iter().any(|&(x, z, radius)| {
            let x = dx - x;
            let z = dz - z;
            x * x + z * z <= radius * radius
          });
          if !inside {
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
          let surface_y = chunk.chunk.surfaces(column).iter().copied().max().map(i32::from);
          let Some(surface_y) = surface_y else { continue };
          if surface_y < 2 || surface_y > 252 {
            continue;
          }

          let surface = column.with_y(surface_y);
          if !chunk.is_active(surface) {
            continue;
          }
          let current = chunk.get(surface);
          if current == block![air] || current == block![water] {
            continue;
          }

          // Surface plus one or two blocks beneath it.
          let gravel_depth = rng.range(2..=3);
          let stone_depth = rng.range(2..=3);

          // Gravel remains the dominant surface material, with occasional
          // exposed inclusions selected from the biome-provided mix.
          let surface_material =
            if rng.range(0..100) < 60 { block![gravel] } else { *rng.choose(&self.mix_in) };

          for depth in 0..gravel_depth.min(surface_y) {
            let block = if depth == 0 || surface_material != block![gravel] {
              surface_material
            } else {
              block![gravel]
            };
            chunk.set(surface.with_y(surface_y - depth), block);
          }

          // Continue the same material palette below the gravel cap. Natural
          // stone is included alongside every configured surface inclusion.
          for depth in 0..stone_depth {
            let y = surface_y - gravel_depth - depth;
            if y <= 0 {
              break;
            }
            chunk.set(column.with_y(y), *rng.choose(&stone_materials));
          }
        }
      }
    }
  }
}
