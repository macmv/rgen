//! The terrain as 3D chunks of blocks, for the ray marcher.
//!
//! Only chunks that cross the surface store any blocks. Every other chunk is
//! either all air or all solid, and is a single value in an indirection table.
//! The bricks of the chunks that do store blocks live in slots, which are the
//! same slots as in the GPU's atlas. This all lives on the CPU too, so that
//! hovering can ray cast without reading anything back.

use std::sync::Arc;

/// The width of a chunk, in blocks.
pub const CHUNK: usize = 32;

/// The height of the world, in blocks.
pub const WORLD_HEIGHT: usize = 256;

const VOLUME: usize = CHUNK * CHUNK * CHUNK;

/// A chunk with nothing in it.
pub const EMPTY: u32 = 0;
/// A chunk that is all solid.
pub const SOLID: u32 = 1;
/// Table values from here up are `FIRST_SLOT + slot`.
pub const FIRST_SLOT: u32 = 2;

/// The index of a block in a brick. This is the order the bricks are laid out
/// in the atlas, where y is the fastest axis in the texture. That makes the
/// blocks of a column contiguous, so columns can be filled with one write.
pub fn brick_index(x: usize, y: usize, z: usize) -> usize { y + CHUNK * (x + CHUNK * z) }

/// A change to one chunk, to copy into the GPU's textures.
pub struct ChunkUpdate {
  /// The chunk's position, in chunks.
  pub chunk: [usize; 3],
  /// The new table value.
  pub value: u32,
  /// The new contents of the chunk's slot, if they changed.
  pub brick: Option<Arc<[u8]>>,
}

pub struct Chunks {
  /// The number of chunks along x, y and z.
  pub dims: [usize; 3],

  /// What each chunk is. This is laid out like the table texture, so x is
  /// fastest.
  table:    Vec<u32>,
  /// The contents of each slot. These are shared with the pending uploads.
  bricks:   Vec<Arc<[u8]>>,
  free:     Vec<u32>,
  capacity: usize,

  /// Set when there weren't enough slots, and some chunks are missing.
  pub overflowed: bool,
}

impl Chunks {
  /// Makes an empty world that covers `size` columns. `capacity` is the number
  /// of slots in the atlas.
  pub fn new(size: [usize; 2], capacity: usize) -> Self {
    let dims = [size[0].div_ceil(CHUNK), WORLD_HEIGHT / CHUNK, size[1].div_ceil(CHUNK)];
    Chunks {
      dims,
      table: vec![EMPTY; dims[0] * dims[1] * dims[2]],
      bricks: vec![],
      free: vec![],
      capacity,
      overflowed: false,
    }
  }

  fn table_index(&self, c: [usize; 3]) -> usize { c[0] + self.dims[0] * (c[1] + self.dims[1] * c[2]) }

  fn alloc(&mut self) -> Option<u32> {
    if let Some(slot) = self.free.pop() {
      return Some(slot);
    }
    if self.bricks.len() >= self.capacity {
      return None;
    }
    self.bricks.push(Arc::from(vec![0u8; VOLUME]));
    Some(self.bricks.len() as u32 - 1)
  }

  /// Rebuilds the chunks above one square of columns from their heights, and
  /// pushes the changes. `heights` is indexed with `width` columns per row. A
  /// column's blocks are the ones below its height.
  pub fn rebuild_column(
    &mut self,
    cx: usize,
    cz: usize,
    heights: &[f32],
    width: usize,
    out: &mut Vec<ChunkUpdate>,
  ) {
    let mut column = [0i32; CHUNK * CHUNK];
    let (mut lowest, mut highest) = (i32::MAX, 0);
    for lz in 0..CHUNK {
      for lx in 0..CHUNK {
        let h = heights[(cz * CHUNK + lz) * width + cx * CHUNK + lx] as i32;
        let h = h.clamp(0, WORLD_HEIGHT as i32);
        column[lz * CHUNK + lx] = h;
        lowest = lowest.min(h);
        highest = highest.max(h);
      }
    }

    let mut scratch = vec![0u8; VOLUME];

    for cy in 0..self.dims[1] {
      let chunk = [cx, cy, cz];
      let idx = self.table_index(chunk);
      let old = self.table[idx];
      let base = (cy * CHUNK) as i32;

      if base >= highest || base + CHUNK as i32 <= lowest {
        let value = if base >= highest { EMPTY } else { SOLID };
        if old >= FIRST_SLOT {
          self.free.push(old - FIRST_SLOT);
        }
        if old != value {
          self.table[idx] = value;
          out.push(ChunkUpdate { chunk, value, brick: None });
        }
        continue;
      }

      scratch.fill(0);
      for lz in 0..CHUNK {
        for lx in 0..CHUNK {
          let local = (column[lz * CHUNK + lx] - base).clamp(0, CHUNK as i32) as usize;
          let start = brick_index(lx, 0, lz);
          scratch[start..start + local].fill(1);
        }
      }

      let slot = if old >= FIRST_SLOT { Some(old - FIRST_SLOT) } else { self.alloc() };
      let Some(slot) = slot else {
        self.overflowed = true;
        if old != EMPTY {
          self.table[idx] = EMPTY;
          out.push(ChunkUpdate { chunk, value: EMPTY, brick: None });
        }
        continue;
      };

      let value = FIRST_SLOT + slot;
      let same = old == value && *self.bricks[slot as usize] == scratch[..];
      if same {
        continue;
      }

      let brick: Arc<[u8]> = Arc::from(&scratch[..]);
      self.bricks[slot as usize] = brick.clone();
      self.table[idx] = value;
      out.push(ChunkUpdate { chunk, value, brick: Some(brick) });
    }
  }

  /// Casts a ray through the blocks, and returns the first block it hits. The
  /// ray is in grid space: one unit is one block. This is the same walk the
  /// shader does: step through the chunks, and through the blocks of the ones
  /// that have any.
  pub fn raycast(&self, origin: [f32; 3], dir: [f32; 3]) -> Option<[usize; 3]> {
    let c = CHUNK as f32;
    let bounds = [0, 1, 2].map(|a| self.dims[a] as f32 * c);
    let safe = dir.map(|d| if d.abs() < 1e-9 { 1e-9f32.copysign(d) } else { d });

    // Clip the ray to the box around the world.
    let (mut t_in, mut t_out) = (0.0f32, f32::INFINITY);
    for a in 0..3 {
      let t0 = -origin[a] / safe[a];
      let t1 = (bounds[a] - origin[a]) / safe[a];
      t_in = t_in.max(t0.min(t1));
      t_out = t_out.min(t0.max(t1));
    }
    if t_in >= t_out {
      return None;
    }

    let step: [i32; 3] = safe.map(|d| if d > 0.0 { 1 } else { -1 });
    let start = [0, 1, 2].map(|a| origin[a] + safe[a] * (t_in + 1e-3));
    let mut cell: [i32; 3] =
      [0, 1, 2].map(|a| ((start[a] / c).floor() as i32).clamp(0, self.dims[a] as i32 - 1));
    let mut t_max =
      [0, 1, 2].map(|a| ((cell[a] + (step[a] + 1) / 2) as f32 * c - origin[a]) / safe[a]);
    let t_delta = safe.map(|d| c / d.abs());

    let mut t_cur = t_in;
    for _ in 0..self.dims.iter().sum::<usize>() + 4 {
      let state = self.table[self.table_index(cell.map(|v| v as usize))];

      if state != EMPTY {
        let lo = cell.map(|v| v * CHUNK as i32);
        let p = [0, 1, 2].map(|a| origin[a] + safe[a] * (t_cur + 1e-3));
        let v = [0, 1, 2].map(|a| (p[a].floor() as i32).clamp(lo[a], lo[a] + CHUNK as i32 - 1));

        if state == SOLID {
          return Some(v.map(|v| v as usize));
        }
        if let Some(hit) = self.march_brick(origin, safe, step, lo, v, state - FIRST_SLOT) {
          return Some(hit);
        }
      }

      let a = argmin(t_max);
      if t_max[a] > t_out {
        return None;
      }
      t_cur = t_max[a];
      t_max[a] += t_delta[a];
      cell[a] += step[a];
      if cell[a] < 0 || cell[a] >= self.dims[a] as i32 {
        return None;
      }
    }
    None
  }

  /// Walks the blocks of one chunk, starting at `v`, which is the block the ray
  /// enters through.
  fn march_brick(
    &self,
    origin: [f32; 3],
    safe: [f32; 3],
    step: [i32; 3],
    lo: [i32; 3],
    mut v: [i32; 3],
    slot: u32,
  ) -> Option<[usize; 3]> {
    let brick = &self.bricks[slot as usize];
    let mut t_max =
      [0, 1, 2].map(|a| ((v[a] + (step[a] + 1) / 2) as f32 - origin[a]) / safe[a]);
    let t_delta = safe.map(|d| 1.0 / d.abs());

    for _ in 0..3 * CHUNK + 2 {
      let l = [0, 1, 2].map(|a| v[a] - lo[a]);
      if l.iter().any(|&l| l < 0 || l >= CHUNK as i32) {
        return None;
      }
      if brick[brick_index(l[0] as usize, l[1] as usize, l[2] as usize)] != 0 {
        return Some(v.map(|v| v as usize));
      }

      let a = argmin(t_max);
      v[a] += step[a];
      t_max[a] += t_delta[a];
    }
    None
  }
}

fn argmin(v: [f32; 3]) -> usize {
  if v[0] < v[1] {
    if v[0] < v[2] { 0 } else { 2 }
  } else if v[1] < v[2] {
    1
  } else {
    2
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// A world of `size` columns, where the height of each is `f(x, z)`.
  fn world(size: usize, f: impl Fn(usize, usize) -> f32) -> (Chunks, Vec<f32>) {
    let heights: Vec<f32> =
      (0..size * size).map(|i| f(i % size, i / size)).collect();
    let mut chunks = Chunks::new([size, size], 1024);
    let mut out = vec![];
    for cz in 0..size / CHUNK {
      for cx in 0..size / CHUNK {
        chunks.rebuild_column(cx, cz, &heights, size, &mut out);
      }
    }
    (chunks, heights)
  }

  #[test]
  fn classifies_chunks() {
    let (chunks, _) = world(64, |_, _| 40.0);
    // y 0..32 is solid, and y 32..64 crosses the surface at 40.
    assert_eq!(chunks.table[chunks.table_index([0, 0, 0])], SOLID);
    assert!(chunks.table[chunks.table_index([0, 1, 0])] >= FIRST_SLOT);
    assert_eq!(chunks.table[chunks.table_index([0, 2, 0])], EMPTY);
  }

  #[test]
  fn straight_down_hits_the_top_block() {
    let (chunks, heights) = world(64, |x, z| 30.0 + ((x * 7 + z * 3) % 20) as f32);
    for (x, z) in [(0, 0), (5, 9), (33, 40), (63, 63)] {
      let hit = chunks.raycast([x as f32 + 0.5, 200.0, z as f32 + 0.5], [0.0, -1.0, 0.0]);
      let top = heights[z * 64 + x] as usize - 1;
      assert_eq!(hit, Some([x, top, z]));
    }
  }

  #[test]
  fn sideways_ray_hits_a_cliff() {
    // A wall of columns at x >= 40.
    let (chunks, _) = world(64, |x, _| if x >= 40 { 100.0 } else { 20.0 });
    let hit = chunks.raycast([0.5, 50.0, 10.5], [1.0, 0.0, 0.0]);
    assert_eq!(hit, Some([40, 50, 10]));
  }

  #[test]
  fn misses_above_the_terrain() {
    let (chunks, _) = world(64, |_, _| 20.0);
    assert_eq!(chunks.raycast([0.5, 100.0, 0.5], [1.0, 0.0, 0.2]), None);
  }

  #[test]
  fn rebuild_only_reports_changes() {
    let (mut chunks, heights) = world(32, |_, _| 40.0);
    let mut out = vec![];
    chunks.rebuild_column(0, 0, &heights, 32, &mut out);
    assert!(out.is_empty());
  }

  #[test]
  fn running_out_of_slots_is_reported() {
    let mut chunks = Chunks::new([32, 32], 0);
    let heights = vec![40.0; 32 * 32];
    chunks.rebuild_column(0, 0, &heights, 32, &mut vec![]);
    assert!(chunks.overflowed);
  }
}
