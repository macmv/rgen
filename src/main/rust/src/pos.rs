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
