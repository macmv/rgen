use crate::Pos;

pub struct BlocksIterInclusive {
  min: Pos,
  max: Pos,
  pos: Pos,
}

pub struct BlocksIterExclusive {
  min: Pos,
  max: Pos,
  pos: Pos,
}

impl BlocksIterInclusive {
  pub fn new(min: Pos, max: Pos) -> Self {
    assert!(min.x <= max.x, "{:?} <= {:?}", min, max);
    assert!(min.y <= max.y, "{:?} <= {:?}", min, max);
    assert!(min.z <= max.z, "{:?} <= {:?}", min, max);

    BlocksIterInclusive { min, max, pos: min }
  }
}

impl BlocksIterExclusive {
  pub fn new(min: Pos, max: Pos) -> Self {
    assert!(min.x <= max.x, "{:?} <= {:?}", min, max);
    assert!(min.y <= max.y, "{:?} <= {:?}", min, max);
    assert!(min.z <= max.z, "{:?} <= {:?}", min, max);

    BlocksIterExclusive { min, max, pos: min }
  }
}

impl Iterator for BlocksIterInclusive {
  type Item = Pos;

  fn next(&mut self) -> Option<Self::Item> {
    if self.pos.x > self.max.x {
      self.pos.x = self.min.x;
      self.pos.z += 1;
    }
    if self.pos.z > self.max.z {
      self.pos.z = self.min.z;
      self.pos.y += 1;
    }
    if self.pos.y > self.max.y {
      return None;
    }

    let pos = self.pos;
    self.pos.x += 1;
    Some(pos)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let layers_left = (self.max.y - self.pos.y) as usize;
    let rows_left = (self.max.z - self.pos.z) as usize;
    let cols_left = (self.max.x - self.pos.x + 1) as usize;

    let remaining =
      layers_left * (self.max.z - self.min.z + 1) as usize * (self.max.x - self.min.x + 1) as usize
        + rows_left * (self.max.x - self.min.x + 1) as usize
        + cols_left;

    (remaining, Some(remaining))
  }
}

impl Iterator for BlocksIterExclusive {
  type Item = Pos;

  fn next(&mut self) -> Option<Self::Item> {
    if self.pos.x >= self.max.x {
      self.pos.x = self.min.x;
      self.pos.z += 1;
    }
    if self.pos.z >= self.max.z {
      self.pos.z = self.min.z;
      self.pos.y += 1;
    }
    if self.pos.y >= self.max.y {
      return None;
    }

    let pos = self.pos;
    self.pos.x += 1;
    Some(pos)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let layers_left = ((self.max.y - self.pos.y) as usize).saturating_sub(1);
    let rows_left = ((self.max.z - self.pos.z) as usize).saturating_sub(1);
    let cols_left = (self.max.x - self.pos.x) as usize;

    let remaining =
      layers_left * (self.max.z - self.min.z) as usize * (self.max.x - self.min.x) as usize
        + rows_left * (self.max.x - self.min.x) as usize
        + cols_left;

    (remaining, Some(remaining))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn blocks_iter_inclusive_works() {
    let iter = BlocksIterInclusive::new(Pos::new(0, 0, 0), Pos::new(2, 2, 2));

    assert_eq!(
      iter.collect::<Vec<_>>(),
      vec![
        Pos::new(0, 0, 0),
        Pos::new(1, 0, 0),
        Pos::new(2, 0, 0),
        Pos::new(0, 0, 1),
        Pos::new(1, 0, 1),
        Pos::new(2, 0, 1),
        Pos::new(0, 0, 2),
        Pos::new(1, 0, 2),
        Pos::new(2, 0, 2),
        Pos::new(0, 1, 0),
        Pos::new(1, 1, 0),
        Pos::new(2, 1, 0),
        Pos::new(0, 1, 1),
        Pos::new(1, 1, 1),
        Pos::new(2, 1, 1),
        Pos::new(0, 1, 2),
        Pos::new(1, 1, 2),
        Pos::new(2, 1, 2),
        Pos::new(0, 2, 0),
        Pos::new(1, 2, 0),
        Pos::new(2, 2, 0),
        Pos::new(0, 2, 1),
        Pos::new(1, 2, 1),
        Pos::new(2, 2, 1),
        Pos::new(0, 2, 2),
        Pos::new(1, 2, 2),
        Pos::new(2, 2, 2),
      ]
    );
  }

  #[test]
  fn blocks_iter_inclusive_size_hint() {
    let mut iter = BlocksIterInclusive::new(Pos::new(0, 0, 0), Pos::new(2, 2, 2));

    let mut expected = 27;
    while expected > 0 {
      assert_eq!(iter.size_hint(), (expected, Some(expected)));
      expected -= 1;

      iter.next().unwrap();
    }

    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert!(iter.next().is_none());
  }

  #[test]
  fn blocks_iter_exclusive_works() {
    let iter = BlocksIterExclusive::new(Pos::new(0, 0, 0), Pos::new(3, 3, 3));

    assert_eq!(
      iter.collect::<Vec<_>>(),
      vec![
        Pos::new(0, 0, 0),
        Pos::new(1, 0, 0),
        Pos::new(2, 0, 0),
        Pos::new(0, 0, 1),
        Pos::new(1, 0, 1),
        Pos::new(2, 0, 1),
        Pos::new(0, 0, 2),
        Pos::new(1, 0, 2),
        Pos::new(2, 0, 2),
        Pos::new(0, 1, 0),
        Pos::new(1, 1, 0),
        Pos::new(2, 1, 0),
        Pos::new(0, 1, 1),
        Pos::new(1, 1, 1),
        Pos::new(2, 1, 1),
        Pos::new(0, 1, 2),
        Pos::new(1, 1, 2),
        Pos::new(2, 1, 2),
        Pos::new(0, 2, 0),
        Pos::new(1, 2, 0),
        Pos::new(2, 2, 0),
        Pos::new(0, 2, 1),
        Pos::new(1, 2, 1),
        Pos::new(2, 2, 1),
        Pos::new(0, 2, 2),
        Pos::new(1, 2, 2),
        Pos::new(2, 2, 2),
      ]
    );
  }

  #[test]
  fn blocks_iter_exclusive_size_hint() {
    let mut iter = BlocksIterExclusive::new(Pos::new(0, 0, 0), Pos::new(3, 3, 3));

    let mut expected = 27;
    while expected > 0 {
      assert_eq!(iter.size_hint(), (expected, Some(expected)));
      expected -= 1;

      iter.next().unwrap();
    }

    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert!(iter.next().is_none());
  }

  #[test]
  fn blocks_iter_inclusive_zero() {
    let mut iter = BlocksIterInclusive::new(Pos::new(0, 0, 0), Pos::new(0, 0, 0));
    assert_eq!(iter.next(), Some(Pos::new(0, 0, 0)));
    assert!(iter.next().is_none());
  }

  #[test]
  fn blocks_iter_exclusive_zero() {
    let mut iter = BlocksIterExclusive::new(Pos::new(0, 0, 0), Pos::new(0, 0, 0));
    assert!(iter.next().is_none());

    let mut iter = BlocksIterExclusive::new(Pos::new(0, 0, 0), Pos::new(5, 0, 3));
    assert!(iter.next().is_none());
  }
}
