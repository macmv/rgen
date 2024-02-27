use std::ops::Add;

/// A position in a chunk.
///
/// The x and z coordinates are in the range 0..16, and the y coordinate is in
/// the range 0..256.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkRelPos {
  x: u8,
  y: u8,
  z: u8,
}

impl ChunkRelPos {
  #[track_caller]
  pub fn new(x: u8, y: u8, z: u8) -> ChunkRelPos {
    assert!(x < 16);
    assert!(z < 16);
    ChunkRelPos { x, y, z }
  }

  pub fn x(&self) -> u8 { self.x }
  pub fn y(&self) -> u8 { self.y }
  pub fn z(&self) -> u8 { self.z }

  /// Returns the current position, with the Y set to the given value.
  ///
  /// ```
  /// # use rgen_base::ChunkRelPos;
  /// let pos = ChunkRelPos::new(3, 4, 5);
  ///
  /// assert_eq!(pos.with_y(6), ChunkRelPos::new(3, 6, 5));
  /// ```
  pub fn with_y(&self, y: u8) -> ChunkRelPos { ChunkRelPos { x: self.x, y, z: self.z } }
}

/// A position in the world.
///
/// The X and Z coordinates are unbounded, and the Y coordinate is in the range
/// 0..256.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
  pub x: i32,
  pub y: u8,
  pub z: i32,
}

impl Pos {
  pub fn new(x: i32, y: u8, z: i32) -> Pos { Pos { x, y, z } }

  pub fn x(&self) -> i32 { self.x }
  pub fn y(&self) -> u8 { self.y }
  pub fn z(&self) -> i32 { self.z }

  /// Returns `true` if the position is in the given chunk position.
  ///
  /// ```
  /// # use rgen_base::{Pos, ChunkPos};
  /// let pos = Pos::new(13, 0, 19);
  ///
  /// assert!(pos.in_chunk(ChunkPos::new(0, 1)));
  /// assert!(!pos.in_chunk(ChunkPos::new(0, 0)));
  /// assert!(!pos.in_chunk(ChunkPos::new(0, 2)));
  /// ```
  pub fn in_chunk(&self, chunk_pos: ChunkPos) -> bool {
    self.x >= chunk_pos.x * 16
      && self.x < (chunk_pos.x + 1) * 16
      && self.z >= chunk_pos.z * 16
      && self.z < (chunk_pos.z + 1) * 16
  }

  pub fn chunk(&self) -> ChunkPos {
    let chunk_x = if self.x < 0 { (self.x + 1) / 16 - 1 } else { self.x / 16 };
    let chunk_z = if self.x < 0 { (self.z + 1) / 16 - 1 } else { self.z / 16 };
    ChunkPos::new(chunk_x, chunk_z)
  }

  /// Returns the position of this block in the chunk it is in.
  ///
  /// ```
  /// # use rgen_base::Pos;
  /// let pos = Pos::new(13, 0, 19);
  /// let chunk_pos = pos.chunk_rel();
  ///
  /// assert_eq!(chunk_pos.x(), 13);
  /// assert_eq!(chunk_pos.z(), 3);
  /// ```
  pub fn chunk_rel(&self) -> ChunkRelPos {
    ChunkRelPos::new(((self.x % 16 + 16) % 16) as u8, self.y, ((self.z % 16 + 16) % 16) as u8)
  }

  /// Returns the current position, with the Y set to the given value.
  ///
  /// ```
  /// # use rgen_base::Pos;
  /// let pos = Pos::new(3, 4, 5);
  ///
  /// assert_eq!(pos.with_y(6), Pos::new(3, 6, 5));
  /// ```
  pub fn with_y(&self, y: u8) -> Pos { Pos { x: self.x, y, z: self.z } }
}

/// The position of a chunk in the world.
///
/// The X and Z coordinates are unbounded, and they are 16 times smaller than a
/// block position. To get block position of this chunk, use `min_block_pos`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
  pub x: i32,
  pub z: i32,
}

impl ChunkPos {
  pub fn new(x: i32, z: i32) -> ChunkPos { ChunkPos { x, z } }

  pub fn x(&self) -> i32 { self.x }
  pub fn z(&self) -> i32 { self.z }

  pub fn min_block_pos(&self) -> Pos { Pos::new(self.x * 16, 0, self.z * 16) }
}

impl Add for Pos {
  type Output = Pos;

  #[track_caller]
  fn add(self, other: Pos) -> Pos { Pos::new(self.x + other.x, self.y + other.y, self.z + other.z) }
}

impl Add for ChunkPos {
  type Output = ChunkPos;

  #[track_caller]
  fn add(self, other: ChunkPos) -> ChunkPos { ChunkPos::new(self.x + other.x, self.z + other.z) }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn chunk_rel_pos() {
    let pos = Pos::new(13, 0, 19).chunk_rel();

    assert_eq!(pos.x(), 13);
    assert_eq!(pos.y(), 0);
    assert_eq!(pos.z(), 3);

    // Make sure negatives work
    let pos = Pos::new(-1, 0, -13).chunk_rel();

    assert_eq!(pos.x(), 15);
    assert_eq!(pos.y(), 0);
    assert_eq!(pos.z(), 3);
  }
}
