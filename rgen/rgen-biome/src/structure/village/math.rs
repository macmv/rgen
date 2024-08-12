// TODO: Most of this should go into `rgen_base`.

use rgen_base::Pos;

#[derive(Clone, Copy)]
pub enum Direction {
  North,
  East,
  South,
  West,
}

#[derive(Clone, Copy)]
pub enum Axis {
  X,
  Z,
}

pub struct Rectangle {
  pub min: Pos,
  pub max: Pos,
}

impl Direction {
  pub fn dir(&self) -> Pos {
    match self {
      Direction::North => Pos::new(0, 0, -1),
      Direction::East => Pos::new(1, 0, 0),
      Direction::South => Pos::new(0, 0, 1),
      Direction::West => Pos::new(-1, 0, 0),
    }
  }

  pub fn right(&self) -> Direction {
    match self {
      Direction::North => Direction::East,
      Direction::East => Direction::South,
      Direction::South => Direction::West,
      Direction::West => Direction::North,
    }
  }

  #[allow(unused)]
  pub fn opposite(&self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::East => Direction::West,
      Direction::South => Direction::North,
      Direction::West => Direction::East,
    }
  }

  #[allow(unused)]
  pub fn axis(&self) -> Axis {
    match self {
      Direction::North | Direction::South => Axis::Z,
      Direction::East | Direction::West => Axis::X,
    }
  }
}

impl Axis {
  pub fn positive_dir(&self) -> Direction {
    match self {
      Axis::X => Direction::East,
      Axis::Z => Direction::South,
    }
  }

  pub fn negative_dir(&self) -> Direction {
    match self {
      Axis::X => Direction::West,
      Axis::Z => Direction::North,
    }
  }

  pub fn orthogonal(&self) -> Axis {
    match self {
      Axis::X => Axis::Z,
      Axis::Z => Axis::X,
    }
  }
}

impl Rectangle {
  pub fn intersects(&self, other: &Rectangle) -> bool {
    self.min.x <= other.max.x
      && self.max.x >= other.min.x
      && self.min.z <= other.max.z
      && self.max.z >= other.min.z
  }
}
