use rgen_base::Pos;
use rgen_llama::Structure;

use super::math::{Direction, Rectangle};

#[derive(Clone)]
pub struct Building {
  // The door position, or the front center of the building. The Y value will be zero, as the
  // height of each building must be determined later, in the decoration phase.
  pub pos: Pos,

  // The direction the front door is facing. This is perpendicular to the road.
  pub forward: Direction,

  pub building_id: u32,

  // Width, or the parallel distance to the road.
  pub width: u32,

  // Depth, or the distance the building extends from the road.
  pub depth: u32,
}

impl Building {
  fn forward_dir(&self) -> Pos { self.forward.dir() }
  fn right_dir(&self) -> Pos { self.forward.right().dir() }

  pub fn front_right(&self) -> Pos { self.pos + self.right_dir() * (self.width as i32 / 2) }
  pub fn front_left(&self) -> Pos { self.pos - self.right_dir() * (self.width as i32 / 2) }
  pub fn back_right(&self) -> Pos {
    self.pos - self.forward_dir() * (self.depth as i32) + self.right_dir() * (self.width as i32 / 2)
  }
  pub fn back_left(&self) -> Pos {
    self.pos - self.forward_dir() * (self.depth as i32) - self.right_dir() * (self.width as i32 / 2)
  }

  pub fn bounding_box(&self) -> Rectangle {
    // Add in some offsets around the edges.
    let front_left = self.front_left() - self.right_dir();
    let front_right = self.front_right() + self.right_dir();
    let back_left = self.back_left() - self.right_dir() - self.forward_dir();
    let back_right = self.back_right() + self.right_dir() - self.forward_dir();

    let min_x = front_left.x.min(front_right.x).min(back_left.x).min(back_right.x);
    let min_z = front_left.z.min(front_right.z).min(back_left.z).min(back_right.z);

    let max_x = front_left.x.max(front_right.x).max(back_left.x).max(back_right.x);
    let max_z = front_left.z.max(front_right.z).max(back_left.z).max(back_right.z);

    Rectangle { min: Pos::new(min_x, self.pos.y, min_z), max: Pos::new(max_x, self.pos.y, max_z) }
  }

  pub fn transform_to_world(&self, structure: &Structure, pos: Pos) -> Pos {
    // This is the axis of rotation for the building.
    let front_center = Pos::new(structure.width() as i32 / 2, 0, 0);

    self.pos + rotate_around(pos - front_center, Pos::new(0, 0, 0), self.forward)
  }
}

// Rotate `pos` about `around`.
fn rotate_around(pos: Pos, around: Pos, dir: Direction) -> Pos {
  let rotated_x = pos.x - around.x;
  let rotated_z = pos.z - around.z;
  let (rotated_x, rotated_z) = match dir {
    Direction::North => (rotated_x, rotated_z),
    Direction::East => (-rotated_z, rotated_x),
    Direction::South => (-rotated_x, -rotated_z),
    Direction::West => (rotated_z, -rotated_x),
  };

  Pos::new(around.x + rotated_x, pos.y, around.z + rotated_z)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn min_max() {
    // North is -Z
    let pos = Pos::new(3, 4, 5);
    let building = Building { pos, building_id: 0, forward: Direction::North, width: 3, depth: 4 };

    assert_eq!(building.front_left(), pos + Pos::new(-1, 0, 0));
    assert_eq!(building.front_right(), pos + Pos::new(1, 0, 0));
    assert_eq!(building.back_left(), pos + Pos::new(-1, 0, 4));
    assert_eq!(building.back_right(), pos + Pos::new(1, 0, 4));
  }

  #[test]
  fn rotate_around_works() {
    assert_eq!(
      rotate_around(Pos::new(1, 0, 2), Pos::new(1, 0, 1), Direction::North),
      Pos::new(1, 0, 2)
    );
    assert_eq!(
      rotate_around(Pos::new(1, 0, 2), Pos::new(1, 0, 1), Direction::East),
      Pos::new(0, 0, 1)
    );
    assert_eq!(
      rotate_around(Pos::new(1, 0, 2), Pos::new(1, 0, 1), Direction::South),
      Pos::new(1, 0, 0)
    );
    assert_eq!(
      rotate_around(Pos::new(1, 0, 2), Pos::new(1, 0, 1), Direction::West),
      Pos::new(2, 0, 1)
    );
  }
}
