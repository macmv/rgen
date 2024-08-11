use rgen_base::{Blocks, Pos};

#[test]
fn parse_house() {
  let blocks = Blocks::test_blocks();

  let house = crate::parse(&blocks, include_str!("./house.ll"));

  assert_eq!(house.get(Pos::new(0, 0, 0)), blocks.stone.default_state);
  assert_eq!(house.get(Pos::new(1, 0, 0)), blocks.stone.default_state);
  assert_eq!(house.get(Pos::new(0, 0, 1)), blocks.stone.default_state);
  assert_eq!(house.get(Pos::new(0, 1, 0)), blocks.cobblestone.default_state);
}

#[test]
fn parse_house_vertical() {
  let blocks = Blocks::test_blocks();

  let house = crate::parse(&blocks, include_str!("./house.ll"));
  let vertical = crate::parse(&blocks, include_str!("./house_vertical.ll"));

  assert_eq!(house.get(Pos::new(0, 0, 0)), blocks.stone.default_state);
  assert_eq!(house.get(Pos::new(1, 0, 0)), blocks.stone.default_state);
  assert_eq!(house.get(Pos::new(0, 0, 1)), blocks.stone.default_state);
  assert_eq!(house.get(Pos::new(0, 1, 0)), blocks.cobblestone.default_state);

  for y in 0..house.height() {
    for x in 0..house.width() {
      for z in 0..house.depth() {
        let pos = Pos::new(x as i32, y as i32, z as i32);
        assert_eq!(house.get(pos), vertical.get(pos));
      }
    }
  }
}
