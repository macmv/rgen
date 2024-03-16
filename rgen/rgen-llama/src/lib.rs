mod parser;
mod structure;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use rgen_base::{Blocks, Pos};
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

  let width = ast.layers.values().map(|layer| layer.width).max().unwrap_or(0);
  let height = ast.layers.values().map(|layer| layer.height).max().unwrap_or(0);
  let depth = ast.ordered.len() as u32;

  let mut structure = Structure::new(width, height, depth);

  for (z, name) in ast.ordered.iter().enumerate() {
    let layer = ast.layers.get(name).unwrap();
    for y in 0..layer.height {
      for x in 0..layer.width {
        let block = layer.blocks[(y * layer.width + x) as usize];
        if block == ' ' {
          continue;
        }

        let name = ast.names.get(&block).unwrap();
        let name = format!("{}:{}", name.category, name.block);

        structure.set(
          Pos::new(x as i32, y as i32, z as i32),
          blocks.by_name(&name).unwrap_or_else(|| panic!("no such block {name}")).default_state,
        );
      }
    }
  }

  structure
}

impl BlockName {
  pub fn air() -> Self {
    BlockName { category: "minecraft".to_string(), block: "air".to_string(), state: None }
  }
}
