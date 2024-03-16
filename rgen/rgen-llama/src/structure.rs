use rgen_base::{BlockState, Pos};

pub struct Structure {
  // Width on the X-axis
  width:  u32,
  // Height on the Y-axis
  height: u32,
  // Depth on the Z-axis
  depth:  u32,

  // Block storage, indexed by Y, then Z, then X
  storage: Vec<BlockState>,
}

impl Structure {
  pub(crate) fn empty() -> Self { Structure { width: 0, height: 0, depth: 0, storage: Vec::new() } }

  #[cfg(test)]
  fn new_test(width: u32, height: u32, depth: u32, storage: Vec<BlockState>) -> Self {
    Structure { width, height, depth, storage }
  }

  /// Returns the width of the structure, or the number of blocks on the
  /// X-axis.
  pub fn width(&self) -> u32 { self.width }
  /// Returns the height of the structure, or the number of blocks on the
  /// Y-axis.
  pub fn height(&self) -> u32 { self.height }
  /// Returns the depth of the structure, or the number of blocks on the Z-axis.
  pub fn depth(&self) -> u32 { self.depth }

  /// Returns `true` if the structure contains the given position.
  pub fn contains(&self, pos: Pos) -> bool {
    pos.x >= 0
      && pos.x < self.width as i32
      && pos.y >= 0
      && pos.y < self.height as i32
      && pos.z >= 0
      && pos.z < self.depth as i32
  }

  /// Returns the block in the structure at the given relative position. Returns
  /// `AIR` if the given position is not within the structure.
  pub fn get(&self, pos: Pos) -> BlockState {
    if self.contains(pos) {
      self.storage[(pos.y as u32 * self.depth * self.width
        + pos.z as u32 * self.width
        + pos.x as u32) as usize]
    } else {
      BlockState::AIR
    }
  }

  /// Rotates the structure on the Y axist by the given delta, in multiples of
  /// 90 degrees. Positive delta means clockwise.
  pub fn rotate(&mut self, delta: i32) {
    let delta = (delta % 4 + 4) % 4;
    if delta == 0 {
      return;
    }

    fn idx(structure: &Structure, x: u32, y: u32, z: u32) -> usize {
      (y * structure.depth * structure.width + z * structure.width + x) as usize
    }

    for y in 0..self.height {
      for z in 0..self.depth / 2 {
        for x in 0..=self.width / 2 {
          let q1 = idx(self, x, y, z);
          let q2 = idx(self, z, y, self.width - 1 - x);
          let q3 = idx(self, self.width - 1 - x, y, self.depth - 1 - z);
          let q4 = idx(self, self.depth - 1 - z, y, x);

          match delta {
            1 => {
              self.storage.swap(q1, q4);
              self.storage.swap(q3, q2);
              self.storage.swap(q1, q3);
            }
            2 => {
              self.storage.swap(q1, q3);
              self.storage.swap(q2, q4);
            }
            3 => {
              self.storage.swap(q1, q2);
              self.storage.swap(q3, q4);
              self.storage.swap(q1, q3);
            }
            _ => {}
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use rgen_base::Blocks;

  use super::*;

  #[test]
  fn structure_rotate() {
    let blocks = Blocks::test_blocks();

    let s = blocks.stone.default_state;
    let w = blocks.wool.default_state;
    let l = blocks.log.default_state;

    #[rustfmt::skip]
    let original = vec![
      s, s, w,
      w, w, l,
      s, s, s,
    ];
    #[rustfmt::skip]
    let rotated_1 = vec![
      s, w, s,
      s, w, s,
      s, l, w,
    ];
    #[rustfmt::skip]
    let rotated_2 = vec![
      s, s, s,
      l, w, w,
      w, s, s,
    ];
    #[rustfmt::skip]
    let rotated_3 = vec![
      w, l, s,
      s, w, s,
      s, w, s,
    ];

    let mut structure = Structure::new_test(3, 1, 3, original.clone());

    structure.rotate(1);
    assert_eq!(structure.storage, rotated_1);
    structure.rotate(1);
    assert_eq!(structure.storage, rotated_2);
    structure.rotate(1);
    assert_eq!(structure.storage, rotated_3);
    structure.rotate(1);
    assert_eq!(structure.storage, original);

    structure.rotate(2);
    assert_eq!(structure.storage, rotated_2);
    structure.rotate(1);
    assert_eq!(structure.storage, rotated_3);
    structure.rotate(1);
    assert_eq!(structure.storage, original);

    structure.rotate(3);
    assert_eq!(structure.storage, rotated_3);
    structure.rotate(1);
    assert_eq!(structure.storage, original);

    structure.rotate(4);
    assert_eq!(structure.storage, original);

    structure.rotate(-1);
    assert_eq!(structure.storage, rotated_3);
    structure.rotate(-1);
    assert_eq!(structure.storage, rotated_2);
    structure.rotate(-1);
    assert_eq!(structure.storage, rotated_1);
    structure.rotate(-1);
    assert_eq!(structure.storage, original);
  }
}
