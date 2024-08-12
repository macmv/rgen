use rgen_base::Pos;

use super::math::{Axis, Rectangle};

#[derive(Clone, Copy)]
pub struct Road {
  pub start: Pos,
  pub end:   Pos,
}

impl Road {
  pub fn axis(&self) -> Axis {
    if self.start.x == self.end.x {
      Axis::Z
    } else {
      Axis::X
    }
  }

  /// Returns the minimum center of this road. Note that the bounding box
  /// extends beyond this position.
  pub fn min(&self) -> Pos {
    Pos::new(self.start.x.min(self.end.x), 0, self.start.z.min(self.end.z))
  }

  /// Returns the maximum center of this road. Note that the bounding box
  /// extends beyond this position.
  pub fn max(&self) -> Pos {
    Pos::new(self.start.x.max(self.end.x), 0, self.start.z.max(self.end.z))
  }

  /// Returns the space this road takes up.
  pub fn bounding_box(&self) -> Rectangle {
    Rectangle { min: self.min() - Pos::new(1, 0, 1), max: self.max() + Pos::new(1, 0, 1) }
  }
}
