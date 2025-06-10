use std::{collections::HashMap, ops::Range};

use parser::{Parser, Token};

mod parser;

struct Config {
  renames: HashMap<&'static str, &'static str>,
}

fn main() {
  let filename = std::env::args().nth(1).expect("No filename provided");

  let source = std::fs::read_to_string(filename).expect("Failed to read file");
  let mut renames = HashMap::new();

  renames.insert("net.minecraft.util.math.AxisAlignedBB", "net.minecraft.util.shape.VoxelShape");
  renames.insert("net.minecraft.util.EnumFacing", "net.minecraft.util.math.Direction");
  renames.insert("net.minecraft.util.EnumHand", "net.minecraft.util.Hand");
  renames.insert("net.minecraft.util.IStringSerializable", "net.minecraft.util.StringIdentifiable");

  renames.insert("net.minecraft.init.Blocks", "net.minecraft.block.Blocks");
  renames.insert("net.minecraft.block.state.IBlockState", "net.minecraft.block.BlockState");
  renames.insert("net.minecraft.block.BlockPackedIce", "net.minecraft.block.IceBlock");

  renames.insert(
    "net.minecraft.block.properties.PropertyBool",
    "net.minecraft.state.property.BooleanProperty",
  );
  renames.insert(
    "net.minecraft.block.properties.PropertyEnum",
    "net.minecraft.state.property.EnumProperty",
  );

  renames.insert("net.minecraft.world.IBlockAccess", "net.minecraft.world.BlockView");

  let config = Config { renames };

  let output = config.process(source);

  print!("{}", output);
}

impl Config {
  fn process(&self, source: String) -> String {
    let mut imports = HashMap::<String, String>::new();

    let mut output = Output::new(source.clone());
    let mut parser = Parser::new(&source);

    // Eat the package statement
    if parser.next() == Some(Token::Word) && parser.slice() == "package" {
      while parser.slice() != ";" {
        parser.next();
      }
    }

    while parser.next() == Some(Token::Word) && parser.slice() == "import" {
      let (path, span) = parse_path(&mut parser);
      let (_, last_part) = path.rsplit_once('.').unwrap();
      imports.insert(last_part.to_string(), path.clone());

      if let Some(new_path) = self.renames.get(path.as_str()) {
        output.replace(span, new_path);
      }
    }

    while let Some(tok) = parser.next() {
      match tok {
        Token::Word => {
          let word = parser.slice();
          if let Some(resolved) = imports.get(word) {
            if let Some(new_path) = self.renames.get(resolved.as_str()) {
              let (_, new_imported) = new_path.rsplit_once('.').unwrap();
              let range = parser.range();
              output.replace(range, new_imported);
            }
          }
        }
        _ => {}
      }
    }

    output.output
  }
}

fn parse_path(parser: &mut Parser) -> (String, Range<usize>) {
  let mut path = String::new();
  let mut start = None;

  loop {
    if let Some(Token::Word) = parser.next() {
      if start.is_none() {
        start = Some(parser.range().start);
      }
      path.push_str(parser.slice());
      assert_eq!(parser.next(), Some(Token::Punct));
      match parser.slice() {
        "." => {
          path.push('.');
        }
        ";" => break (path, start.unwrap()..parser.range().start),
        _ => panic!("expected '.' or ';', found '{}'", parser.slice()),
      }
    } else if parser.slice() == "*" {
      assert_eq!(parser.next(), Some(Token::Punct));
      match parser.slice() {
        ";" => break (path, start.unwrap()..parser.range().start),
        _ => panic!("expected ';', found '{}'", parser.slice()),
      }
    } else {
      panic!();
    }
  }
}

// This tracks a moving offset, as we replace words in a string from start to
// end.
struct Output {
  output: String,
  offset: isize,
}

impl Output {
  pub fn new(output: String) -> Self { Self { output, offset: 0 } }

  pub fn replace(&mut self, range: Range<usize>, str: &str) {
    let start = range.start.wrapping_add_signed(self.offset);
    let end = range.end.wrapping_add_signed(self.offset);

    self.output.replace_range(start..end, str);

    self.offset += str.len() as isize - (end - start) as isize;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn output_works() {
    let mut output = Output::new("foo bar".into());

    output.replace(0..3, "bazzz");
    assert_eq!(output.output, "bazzz bar");

    output.replace(4..7, "quuux");
    assert_eq!(output.output, "bazzz quuux");
  }
}
