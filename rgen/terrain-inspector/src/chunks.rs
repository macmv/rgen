//! The terrain as 3D chunks of blocks, for the ray marcher.
//!
//! Only chunks that cross the surface store any blocks. Every other chunk is
//! either all air or all solid, and is a single value in an indirection table.
//! The bricks of the chunks that do store blocks live in slots, which are the
//! same slots as in the GPU's atlas. This all lives on the CPU too, so that
//! hovering can ray cast without reading anything back.
//!
//! The world wraps around in x and z, like a torus: the chunk at `(cx, cz)` in
//! the table is the chunk at `(cx - shift) mod dims` in the window that the
//! camera sees. Moving the window only changes the shift, and the chunks that
//! leave one side are cleared to make room for the ones that enter the other.

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
/// Table values with this bit set are a chunk that is all one block, which is
/// `UNIFORM | palette index`. The real terrain uses these for the chunks of
/// solid stone and water, which would otherwise each take up a slot.
pub const UNIFORM: u32 = 1 << 31;

/// The slot that a table value points at, if it points at one.
fn slot_of(value: u32) -> Option<u32> {
  (FIRST_SLOT..UNIFORM).contains(&value).then(|| value - FIRST_SLOT)
}

/// The contents of one chunk of the real terrain.
pub enum Brick {
  Empty,
  /// Every block is this palette index.
  Uniform(u8),
  /// The palette index of every block, in `brick_index` order.
  Blocks(Arc<[u8]>),
}

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

  /// The chunk of the table that the window starts at, along x and z.
  shift: [usize; 2],

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
      shift: [0, 0],
      overflowed: false,
    }
  }

  fn table_index(&self, c: [usize; 3]) -> usize {
    c[0] + self.dims[0] * (c[1] + self.dims[1] * c[2])
  }

  /// Sets which chunk of the table is at the window's minimum corner.
  pub fn set_shift(&mut self, shift: [usize; 2]) { self.shift = shift; }

  /// The table index of a chunk of the window.
  fn window_index(&self, c: [usize; 3]) -> usize {
    self.table_index([
      (c[0] + self.shift[0]) % self.dims[0],
      c[1],
      (c[2] + self.shift[1]) % self.dims[2],
    ])
  }

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

  /// Empties the chunks above one square of columns, and pushes the changes.
  pub fn clear_column(&mut self, cx: usize, cz: usize, out: &mut Vec<ChunkUpdate>) {
    for cy in 0..self.dims[1] {
      let chunk = [cx, cy, cz];
      let idx = self.table_index(chunk);
      let old = self.table[idx];
      if let Some(slot) = slot_of(old) {
        self.free.push(slot);
      }
      if old != EMPTY {
        self.table[idx] = EMPTY;
        out.push(ChunkUpdate { chunk, value: EMPTY, brick: None });
      }
    }
  }

  /// Replaces the chunks above one square of columns with chunks of the real
  /// terrain, and pushes the changes. `column` has one brick for each chunk
  /// going up.
  pub fn set_column(
    &mut self,
    cx: usize,
    cz: usize,
    column: Vec<Brick>,
    out: &mut Vec<ChunkUpdate>,
  ) {
    for (cy, brick) in column.into_iter().enumerate() {
      let chunk = [cx, cy, cz];
      let idx = self.table_index(chunk);
      let old = self.table[idx];
      let old_slot = slot_of(old);

      let (value, blocks) = match brick {
        Brick::Empty => (EMPTY, None),
        Brick::Uniform(palette) => (UNIFORM | u32::from(palette), None),
        Brick::Blocks(blocks) => match old_slot.or_else(|| self.alloc()) {
          Some(slot) => (FIRST_SLOT + slot, Some(blocks)),
          None => {
            self.overflowed = true;
            (EMPTY, None)
          }
        },
      };

      if let Some(slot) = old_slot {
        if value != FIRST_SLOT + slot {
          self.free.push(slot);
        }
      }

      if let Some(blocks) = blocks {
        let slot = (value - FIRST_SLOT) as usize;
        if old == value && *self.bricks[slot] == *blocks {
          continue;
        }
        self.bricks[slot] = blocks.clone();
        self.table[idx] = value;
        out.push(ChunkUpdate { chunk, value, brick: Some(blocks) });
      } else if old != value {
        self.table[idx] = value;
        out.push(ChunkUpdate { chunk, value, brick: None });
      }
    }
  }

  /// Rebuilds the chunks above one square of columns from their heights, and
  /// pushes the changes. `heights` is indexed with `width` columns per row, and
  /// the square starts at `start` in it. A column's blocks are the ones below
  /// its height.
  pub fn rebuild_column(
    &mut self,
    cx: usize,
    cz: usize,
    heights: &[f32],
    width: usize,
    start: [usize; 2],
    out: &mut Vec<ChunkUpdate>,
  ) {
    let mut column = [0i32; CHUNK * CHUNK];
    let (mut lowest, mut highest) = (i32::MAX, 0);
    for lz in 0..CHUNK {
      for lx in 0..CHUNK {
        let h = heights[(start[1] + lz) * width + start[0] + lx] as i32;
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
        if let Some(slot) = slot_of(old) {
          self.free.push(slot);
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
          let first = brick_index(lx, 0, lz);
          scratch[first..first + local].fill(1);
        }
      }

      let slot = slot_of(old).or_else(|| self.alloc());
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
      let state = self.table[self.window_index(cell.map(|v| v as usize))];

      if state != EMPTY {
        let lo = cell.map(|v| v * CHUNK as i32);
        let p = [0, 1, 2].map(|a| origin[a] + safe[a] * (t_cur + 1e-3));
        let v = [0, 1, 2].map(|a| (p[a].floor() as i32).clamp(lo[a], lo[a] + CHUNK as i32 - 1));

        if state == SOLID || state >= UNIFORM {
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
    let mut t_max = [0, 1, 2].map(|a| ((v[a] + (step[a] + 1) / 2) as f32 - origin[a]) / safe[a]);
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
    let heights: Vec<f32> = (0..size * size).map(|i| f(i % size, i / size)).collect();
    let mut chunks = Chunks::new([size, size], 1024);
    let mut out = vec![];
    for cz in 0..size / CHUNK {
      for cx in 0..size / CHUNK {
        chunks.rebuild_column(cx, cz, &heights, size, [cx * CHUNK, cz * CHUNK], &mut out);
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
    chunks.rebuild_column(0, 0, &heights, 32, [0, 0], &mut out);
    assert!(out.is_empty());
  }

  #[test]
  fn shifted_window_wraps() {
    // Two chunks along x are solid in the table, then the window moves over.
    let (mut chunks, _) = world(64, |x, _| if x >= 32 { 100.0 } else { 20.0 });
    chunks.set_shift([1, 0]);
    // The window's first chunk is the table's second, which is the tall one.
    let hit = chunks.raycast([0.5, 200.0, 0.5], [0.0, -1.0, 0.0]);
    assert_eq!(hit, Some([0, 99, 0]));
    let hit = chunks.raycast([32.5, 200.0, 0.5], [0.0, -1.0, 0.0]);
    assert_eq!(hit, Some([32, 19, 0]));
  }

  #[test]
  fn clearing_frees_slots() {
    let (mut chunks, _) = world(32, |_, _| 40.0);
    let mut out = vec![];
    chunks.clear_column(0, 0, &mut out);
    assert!(!out.is_empty());
    assert_eq!(chunks.raycast([0.5, 200.0, 0.5], [0.0, -1.0, 0.0]), None);
    assert_eq!(chunks.free.len(), 1);
  }

  #[test]
  fn real_columns_use_slots_and_uniform_chunks() {
    let mut chunks = Chunks::new([32, 32], 8);
    let mut out = vec![];

    let mut blocks = vec![0u8; VOLUME];
    blocks[brick_index(3, 4, 5)] = 7;
    let blocks: Arc<[u8]> = Arc::from(blocks);
    let column = |blocks: &Arc<[u8]>| {
      vec![
        Brick::Uniform(2),
        Brick::Blocks(blocks.clone()),
        Brick::Empty,
        Brick::Empty,
        Brick::Empty,
        Brick::Empty,
        Brick::Empty,
        Brick::Empty,
      ]
    };

    chunks.set_column(0, 0, column(&blocks), &mut out);
    assert_eq!(out.len(), 2);
    assert_eq!(chunks.table[chunks.table_index([0, 0, 0])], UNIFORM | 2);

    // Straight down hits the only block, in the second chunk.
    assert_eq!(chunks.raycast([3.5, 200.0, 5.5], [0.0, -1.0, 0.0]), Some([3, 36, 5]));
    // A ray that misses it lands on the uniform chunk below.
    assert_eq!(chunks.raycast([10.5, 200.0, 10.5], [0.0, -1.0, 0.0]), Some([10, 31, 10]));

    // The same column again changes nothing.
    out.clear();
    chunks.set_column(0, 0, column(&blocks), &mut out);
    assert!(out.is_empty());

    // Emptying it frees the slot.
    chunks.set_column(0, 0, (0..8).map(|_| Brick::Empty).collect(), &mut out);
    assert_eq!(chunks.free.len(), 1);
    assert_eq!(chunks.raycast([3.5, 200.0, 5.5], [0.0, -1.0, 0.0]), None);
  }

  #[test]
  fn running_out_of_slots_is_reported() {
    let mut chunks = Chunks::new([32, 32], 0);
    let heights = vec![40.0; 32 * 32];
    chunks.rebuild_column(0, 0, &heights, 32, [0, 0], &mut vec![]);
    assert!(chunks.overflowed);
  }
}
