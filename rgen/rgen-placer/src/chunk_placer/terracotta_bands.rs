use rgen_base::{BlockFilter, BlockState, ChunkPos, ChunkRelPos};

use crate::{BiomeCachedChunk, ChunkPlacer, Random, Rng};
use crate::noise::{NoiseGenerator, OpenSimplexNoise, SeededNoise};

/// World-aligned strata warped vertically by two scales of seeded noise.
/// Only recolors the supplied substrate; it never fills air or water.
pub struct TerracottaBands {
  pub replace: BlockFilter,
  /// One entry per vertical block; repeated entries make thicker bands.
  pub palette: Vec<BlockState>,
  /// Maximum terrain shell depth to inspect, including the surface.
  pub depth: i32,
  broad: OpenSimplexNoise,
  detail: OpenSimplexNoise,
  phase: i32,
}

impl TerracottaBands {
  /// Vanilla-style seeded bands adapted to a 64-block cycle for our shorter world.
  /// Built once per biome, not per chunk. Keep thin seams at block resolution;
  /// scale the number of colored bands down from the 192-entry vanilla palette.
  pub fn short_world(seed: u64, depth: i32) -> Self {
    Self::new(seed, block![hardened_clay].into(), Self::short_palette(seed), depth)
  }

  fn short_palette(seed: u64) -> Vec<BlockState> {
    let mut rng = Rng::new(seed ^ 0x636C61795F62616E);
    let mut palette = vec![block![hardened_clay]; 64];

    let mut y = rng.range(1..=5) as usize;
    while y < palette.len() {
      palette[y] = block![stained_hardened_clay[color = "orange"]];
      y += rng.range(2..=6) as usize;
    }

    // As in vanilla, later colors can overlap and replace earlier bands.
    for (state, min_width) in [
      (block![stained_hardened_clay[color = "yellow"]], 1),
      (block![stained_hardened_clay[color = "brown"]], 2),
      (block![stained_hardened_clay[color = "red"]], 1),
    ] {
      for _ in 0..rng.range(2..=5) {
        let width = (min_width + rng.range(0..=2)) as usize;
        let start = rng.range(0..palette.len() as i32) as usize;
        let end = (start + width).min(palette.len());
        palette[start..end].fill(state);
      }
    }

    let mut y = 0;
    for _ in 0..rng.range(3..=5) {
      if y >= palette.len() {
        break;
      }
      palette[y] = block![stained_hardened_clay[color = "white"]];
      if y > 0 && rng.range(0..2) == 0 {
        palette[y - 1] = block![stained_hardened_clay[color = "silver"]];
      }
      if y + 1 < palette.len() && rng.range(0..2) == 0 {
        palette[y + 1] = block![stained_hardened_clay[color = "silver"]];
      }
      y += rng.range(4..=19) as usize;
    }
    palette
  }

  pub fn new(seed: u64, replace: BlockFilter, palette: Vec<BlockState>, depth: i32) -> Self {
    assert!(!palette.is_empty(), "terracotta band palette must not be empty");
    let phase = (seed % palette.len() as u64) as i32;
    Self {
      replace, palette, depth,
      broad: OpenSimplexNoise::new(seed ^ 0x6D657361),
      detail: OpenSimplexNoise::new(seed ^ 0x62616E6473),
      phase,
    }
  }

  fn offset(&self, x: i32, z: i32) -> i32 {
    // Two noise samples per column, reused throughout its terracotta shell.
    (self.broad.generate(x as f64 / 96.0, z as f64 / 96.0) * 3.0
      + self.detail.generate(x as f64 / 24.0, z as f64 / 24.0) * 0.75).round() as i32
  }

  fn color(&self, y: i32, offset: i32) -> BlockState {
    self.palette[(y + offset + self.phase).rem_euclid(self.palette.len() as i32) as usize]
  }
}

impl ChunkPlacer for TerracottaBands {
  fn place(&self, chunk: &mut BiomeCachedChunk, _rng: &mut Rng, chunk_pos: ChunkPos) {
    let min = chunk_pos.min_block_pos();
    for x in 0..16 {
      for z in 0..16 {
        let column = ChunkRelPos::new(x, 0, z);
        let Some(top) = chunk.chunk.surfaces(column).iter().copied().max().map(i32::from)
        else { continue; };
        if !chunk.is_active(column.with_y(top)) {
          continue;
        }
        let offset = self.offset(min.x + i32::from(x), min.z + i32::from(z));
        for y in (top - self.depth + 1).max(1)..=top {
          let pos = column.with_y(y);
          if self.replace.contains(chunk.get(pos)) {
            chunk.set(pos, self.color(y, offset));
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::ground_tests::{registry, terrain};
  use crate::TemporaryBiome;

  #[test]
  fn short_palette_is_seeded_and_uses_only_the_terracotta_palette() {
    let allowed = [
      block![hardened_clay],
      block![stained_hardened_clay[color = "orange"]],
      block![stained_hardened_clay[color = "yellow"]],
      block![stained_hardened_clay[color = "brown"]],
      block![stained_hardened_clay[color = "red"]],
      block![stained_hardened_clay[color = "white"]],
      block![stained_hardened_clay[color = "silver"]],
    ];
    let mut previous = None;
    for seed in 0..100 {
      let palette = TerracottaBands::short_palette(seed);
      assert_eq!(palette.len(), 64);
      assert_eq!(palette, TerracottaBands::short_palette(seed));
      assert!(palette.iter().all(|state| allowed.contains(state)));
      assert!(palette.contains(&block![stained_hardened_clay[color = "white"]]));
      if let Some(previous) = previous {
        assert_ne!(palette, previous);
      }
      previous = Some(palette);
    }
  }

  #[test]
  fn bands_match_world_coordinates_preserve_cavities_and_respect_biomes() {
    let info = registry();
    let placer = TerracottaBands::new(42, block![dirt].into(), vec![block![clay], block![sand], block![gravel]], 32);
    for cx in [-1, 0, 1] {
      let cp = ChunkPos::new(cx, -1);
      let mut original = terrain(&info, 80);
      original.set(ChunkRelPos::new(3, 78, 4), info.encode(block![air]));
      original.set(ChunkRelPos::new(4, 78, 4), info.encode(block![water]));
      let mut a = original.clone();
      let mut b = original.clone();
      for (chunk, seed) in [(&mut a, 1), (&mut b, 99)] {
        placer.place(&mut BiomeCachedChunk::new(&info, chunk), &mut Rng::new(seed), cp);
      }
      assert_eq!(a.data(), b.data());
      let chunk = BiomeCachedChunk::new(&info, &mut a);
      for x in 0..16 {
        for z in 0..16 {
          let offset = placer.offset(cx * 16 + i32::from(x), -16 + i32::from(z));
          for y in 0..=80 {
            let pos = ChunkRelPos::new(x, y, z);
            let before = info.decode(original.get(pos));
            if before == block![dirt] {
              assert_eq!(chunk.get(pos), placer.color(y, offset));
            } else {
              assert_eq!(chunk.chunk.get(pos), original.get(pos));
            }
          }
        }
      }
      let mut inactive = original.clone();
      let mut chunk = BiomeCachedChunk::new(&info, &mut inactive);
      chunk.set_active(TemporaryBiome(1));
      placer.place(&mut chunk, &mut Rng::new(1), cp);
      assert_eq!(inactive.data(), original.data());
    }
  }
}
