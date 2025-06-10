use std::collections::HashMap;

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

  renames.insert("net.minecraft.block.BlockPackedIce", "net.minecraft.block.IceBlock");

  renames.insert(
    "net.minecraft.block.properties.PropertyBool",
    "net.minecraft.state.property.BooleanProperty",
  );
  renames.insert(
    "net.minecraft.block.properties.PropertyEnum",
    "net.minecraft.state.property.EnumProperty",
  );

  let config = Config { renames };

  let output = config.process(source);

  print!("{}", output);
}

impl Config {
  fn process(&self, source: String) -> String {
    let mut imports = HashMap::<String, String>::new();

    let mut output = source.clone();
    let mut parser = Parser::new(&source);

    // Eat the package statement
    if parser.next() != Some(Token::Word) || parser.slice() != "package" {
      while parser.slice() != ";" {
        parser.next();
      }
    }

    while parser.next() == Some(Token::Word) && parser.slice() == "import" {
      let path = parse_path(&mut parser);
      let (_, last_part) = path.rsplit_once('.').unwrap();
      imports.insert(last_part.to_string(), path.clone());

      if let Some(new_path) = self.renames.get(path.as_str()) {
        // TODO: Fix range
        output.replace_range(parser.range(), new_path);
      }
    }

    while let Some(tok) = parser.next() {
      match tok {
        Token::Word => {
          let word = parser.slice();
          if let Some(resolved) = imports.get(word) {
            if let Some(new_name) = self.renames.get(resolved.as_str()) {
              let (_, new_imported) = new_name.split_once('.').unwrap();
              let range = parser.range();
              // TODO: Fix range
              output.replace_range(range, new_imported);
            }
          }
        }
        _ => {}
      }
    }

    output
  }
}

fn parse_path(parser: &mut Parser) -> String {
  let mut path = String::new();

  loop {
    match parser.next() {
      Some(Token::Word) => {
        path.push_str(parser.slice());
        assert_eq!(parser.next(), Some(Token::Punct));
        assert_eq!(parser.slice(), ".");
        path.push('.');
      }
      Some(Token::Punct) if parser.slice() == ";" => {
        break;
      }
      _ => panic!(),
    }
  }

  path
}
