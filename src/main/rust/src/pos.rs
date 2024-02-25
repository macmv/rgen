use std::ops::Add;

/// A position in a chunk.
///
/// The x and z coordinates are in the range 0..16, and the y coordinate is in
/// the range 0..256.
pub struct ChunkRelPos {
  x: u8,
  y: u8,
  z: u8,
}

impl ChunkRelPos {
  pub fn new(x: u8, y: u8, z: u8) -> ChunkRelPos {
    assert!(x < 16);
    assert!(z < 16);
    ChunkRelPos { x, y, z }
  }

  pub fn x(&self) -> u8 { self.x }
  pub fn y(&self) -> u8 { self.y }
  pub fn z(&self) -> u8 { self.z }
}

/// A position in the world.
///
/// The X and Z coordinates are unbounded, and the Y coordinate is in the range
/// 0..256.
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
}

/// The position of a chunk in the world.
///
/// The X and Z coordinates are unbounded, and they are 16 times smaller than a
/// block position. To get block position of this chunk, use `min_block_pos`.
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

  fn add(self, other: Pos) -> Pos { Pos::new(self.x + other.x, self.y + other.y, self.z + other.z) }
}
