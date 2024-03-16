mod parser;
mod structure;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use rgen_base::Blocks;
pub use structure::Structure;

#[derive(Default, Debug)]
struct AST {
  names: HashMap<char, BlockName>,

  layers:  HashMap<String, Layer>,
  ordered: Vec<String>,
}

#[derive(Debug)]
struct Layer {
  name:   String,
  width:  u32,
  height: u32,

  blocks: Vec<char>,
}

#[derive(Debug, Clone)]
struct BlockName {
  category: String,
  block:    String,
  state:    Option<u32>,
}

pub fn parse(blocks: &Blocks, input: &str) -> Structure {
  let mut parser = parser::Parser::new(input);
  let mut ast = AST::default();

  parser.parse(&mut ast);

  println!("{:?}", ast);

  Structure::empty()
}

impl BlockName {
  pub fn air() -> Self {
    BlockName { category: "minecraft".to_string(), block: "air".to_string(), state: None }
  }
}
