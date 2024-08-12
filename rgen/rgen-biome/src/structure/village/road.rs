use rgen_base::Pos;

use super::math::Axis;

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
}
