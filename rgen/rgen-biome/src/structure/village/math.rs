// TODO: Most of this should go into `rgen_base`.

use rgen_base::Pos;

#[derive(Clone, Copy)]
pub enum Direction {
  North,
  East,
  South,
  West,
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
}
