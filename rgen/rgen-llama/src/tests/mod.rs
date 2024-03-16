use rgen_base::{Blocks, Pos};

#[test]
fn parse_house() {
  let blocks = Blocks::test_blocks();

  let house = crate::parse(&blocks, include_str!("./house.ll"));

  assert_eq!(house.get(Pos::new(0, 0, 0)), blocks.stone.default_state);
}
