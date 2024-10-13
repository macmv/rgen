mod parser;
mod structure;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use rgen_base::{Block, Pos};
pub use structure::Structure;

#[derive(Default, Debug)]
struct Ast {
  names: HashMap<char, BlockName>,

  layers:      HashMap<LayerKey, Layer>,
  ordered:     Vec<LayerKey>,
  orientation: Orientation,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum LayerKey {
  Name(String),
  Ord(u64),
}

#[derive(Default, Debug, Clone, Copy)]
enum Orientation {
  #[default]
  Horizontal,
  Vertical,
}

#[derive(Debug)]
struct Layer {
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

pub fn parse(input: &str) -> Structure {
  let mut parser = parser::Parser::new(input);
  let mut ast = Ast::default();

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
        let block_name = format!("{}:{}", name.category, name.block);

        let block =
          Block::by_name(&block_name).unwrap_or_else(|| panic!("no such block {block_name}"));

        let state = match name.state {
          Some(state) => block.with_data(state as u8),
          None => block.with_data(0),
        };

        let pos = match ast.orientation {
          Orientation::Horizontal => Pos::new(x as i32, y as i32, z as i32),
          Orientation::Vertical => Pos::new(x as i32, z as i32, y as i32),
        };

        structure.set(pos, state);
      }
    }
  }

  structure
}
