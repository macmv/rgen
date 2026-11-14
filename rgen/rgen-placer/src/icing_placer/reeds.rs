use crate::{BiomeCachedChunk, IcingPlacer, Random, Rng, chunk_placer::util::for_each_clump_point};
use rgen_base::{BlockInfo, ChunkPos, ChunkRelPos, block};

/// Places fully grown, two-block reeds rooted in water.
pub struct Reeds {
  pub avg_per_chunk:          f64,
  pub attempts:               u32,
  /// Default rule: reeds sit above one water block, with ground directly
  /// below that water (reeds / water / ground). Deeper water is opt-in.
  pub must_be_one_block_deep: bool,
}

impl Default for Reeds {
  fn default() -> Self { Reeds { avg_per_chunk: 1.0, attempts: 64, must_be_one_block_deep: true } }
}

impl IcingPlacer for Reeds {
  fn place(&self, chunk: &mut BiomeCachedChunk, rng: &mut Rng, chunk_pos: ChunkPos) {
    // Check each finished column once, even when multiple clump walks hit it.
    let mut visited = [false; 256];
    for_each_clump_point(
      rng.next(),
      chunk_pos,
      self.avg_per_chunk,
      &(2..=6),
      self.attempts,
      |column, seed| {
        // Old candidates jittered vertically across nine heights. Keep roughly
        // that success rate now that we target the actual water surface.
        if Rng::new(seed).range(0..9) != 0 {
          return;
        }
        let index = column.z() as usize * 16 + column.x() as usize;
        if visited[index] {
          return;
        }
        visited[index] = true;
        self.place_column(chunk, column);
      },
    );
  }
}

impl Reeds {
  fn place_column(&self, chunk: &mut BiomeCachedChunk, column: ChunkRelPos) {
    // Surface caches describe the original terrain, not the finished water.
    // Read the actual column so late ground features are respected.
    let Some(y) = (1..=255).rev().find(|&y| chunk.get(column.with_y(y)) == block![water]) else {
      return;
    };
    if y + 2 >= 255 {
      return;
    }
    let water = column.with_y(y);
    if !chunk.is_active(water) {
      return;
    }
    let bottom_y = if self.must_be_one_block_deep {
      y
    } else {
      let mut bottom = y;
      while bottom > 0 && chunk.get(column.with_y(bottom - 1)) == block![water] {
        bottom -= 1;
      }
      bottom
    };
    if bottom_y <= 0 || !is_reed_soil(chunk.get(column.with_y(bottom_y - 1))) {
      return;
    }
    let lower = column.with_y(y + 1);
    let upper = column.with_y(y + 2);
    if chunk.get(lower) != block![air] || chunk.get(upper) != block![air] {
      return;
    }
    // Preserve the mature lower/upper states and leave their water untouched.
    chunk.set(lower, block![rgen:reeds].with_data(3));
    chunk.set(upper, block![rgen:reeds].with_data(7));
  }
}

fn is_reed_soil(soil: BlockInfo<'_>) -> bool {
  soil == block![dirt]
    || soil == block![grass]
    || soil == block![clay]
    || soil == block![gravel]
    || soil == block![sand]
    || soil == block![rgen:mud]
}
