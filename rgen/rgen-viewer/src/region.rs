//! Rendering individual chunks in the world ends up being too expensive when
//! zooming out a bunch. So, we render REGION_SIZExREGION_SIZE block regions
//! instead.
//!
//! These regions are unique to the viewier. They are smaller tha minecraft map
//! regions, and are entirely unrelated.

use rgen_base::Pos;
use std::ops::{Add, Div, Sub};

pub const REGION_SIZE: i32 = 128;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionPos {
  pub x: i32,
  pub z: i32,
}

impl RegionPos {
  pub fn new(x: i32, z: i32) -> Self { RegionPos { x, z } }

  pub fn from_pos(pos: Pos) -> Self {
    let x = if pos.x < 0 { (pos.x + 1) / REGION_SIZE - 1 } else { pos.x / REGION_SIZE };
    let z = if pos.z < 0 { (pos.z + 1) / REGION_SIZE - 1 } else { pos.z / REGION_SIZE };
    RegionPos::new(x, z)
  }

  pub fn min_block_pos(&self) -> Pos { Pos::new(self.x * REGION_SIZE, 0, self.z * REGION_SIZE) }
}

impl Add for RegionPos {
  type Output = RegionPos;

  #[track_caller]
  fn add(self, other: RegionPos) -> RegionPos { RegionPos::new(self.x + other.x, self.z + other.z) }
}

impl Sub for RegionPos {
  type Output = RegionPos;

  #[track_caller]
  fn sub(self, other: RegionPos) -> RegionPos { RegionPos::new(self.x - other.x, self.z - other.z) }
}

impl Div<i32> for RegionPos {
  type Output = RegionPos;

  #[track_caller]
  fn div(self, other: i32) -> RegionPos { RegionPos::new(self.x / other, self.z / other) }
}
