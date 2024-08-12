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

  pub fn opposite(&self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::East => Direction::West,
      Direction::South => Direction::North,
      Direction::West => Direction::East,
    }
  }

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
