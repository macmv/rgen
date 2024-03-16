mod parser;
mod structure;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use rgen_base::Blocks;
pub use structure::Structure;

#[derive(Default)]
struct AST {
  names: HashMap<String, BlockName>,

  layers:  HashMap<String, Layer>,
  ordered: Vec<String>,
}

struct Layer {
  name:   String,
  width:  u32,
  height: u32,

  blocks: Vec<String>,
}

struct BlockName {
  name:  String,
  state: Option<u32>,
}

pub fn parse(blocks: &Blocks, input: &str) -> Structure {
  let mut parser = parser::Parser::new(input);
  let mut ast = AST::default();

  parser.parse(&mut ast);

  Structure::empty()
}
