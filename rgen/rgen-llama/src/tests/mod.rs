use rgen_base::{Pos, block};

#[test]
fn parse_house() {
  let house = crate::parse(include_str!("./house.ll"));

  assert_eq!(house.get(Pos::new(0, 0, 0)), block![stone[0]]);
  assert_eq!(house.get(Pos::new(1, 0, 0)), block![stone[0]]);
  assert_eq!(house.get(Pos::new(0, 0, 1)), block![stone[0]]);
  assert_eq!(house.get(Pos::new(0, 1, 0)), block![cobblestone[0]]);
}

#[test]
fn parse_house_vertical() {
  let house = crate::parse(include_str!("./house.ll"));
  let vertical = crate::parse(include_str!("./house_vertical.ll"));

  assert_eq!(house.get(Pos::new(0, 0, 0)), block![stone[0]]);
  assert_eq!(house.get(Pos::new(1, 0, 0)), block![stone[0]]);
  assert_eq!(house.get(Pos::new(0, 0, 1)), block![stone[0]]);
  assert_eq!(house.get(Pos::new(0, 1, 0)), block![cobblestone[0]]);

  for y in 0..house.height() {
    for x in 0..house.width() {
      for z in 0..house.depth() {
        let pos = Pos::new(x as i32, y as i32, z as i32);
        assert_eq!(house.get(pos), vertical.get(pos));
      }
    }
  }
}
