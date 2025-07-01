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

  // util
  renames.insert("net.minecraft.util.math.AxisAlignedBB", "net.minecraft.util.shape.VoxelShape");
  renames.insert("net.minecraft.util.EnumFacing", "net.minecraft.util.math.Direction");
  renames.insert("net.minecraft.util.EnumHand", "net.minecraft.util.Hand");
  renames.insert("net.minecraft.util.ResourceLocation", "net.minecraft.util.Identifier");
  renames.insert("net.minecraft.util.IStringSerializable", "net.minecraft.util.StringIdentifiable");

  // blocks
  renames.insert("net.minecraft.block.BlockBush", "net.minecraft.block.BushBlock");
  renames.insert("net.minecraft.block.BlockLog", "net.minecraft.block.LogBlock");
  renames.insert("net.minecraft.init.Blocks", "net.minecraft.block.Blocks");
  renames.insert("net.minecraft.block.state.IBlockState", "net.minecraft.block.BlockState");
  renames.insert("net.minecraft.block.BlockPackedIce", "net.minecraft.block.IceBlock");
  renames.insert("net.minecraft.block.BlockRotatedPillar", "net.minecraft.block.PillarBlock");
  renames.insert("net.minecraft.block.BlockVine", "net.minecraft.block.VineBlock");
  renames.insert("net.minecraft.block.material.MapColor", "net.minecraft.block.MapColor");

  renames.insert(
    "net.minecraft.block.properties.PropertyBool",
    "net.minecraft.state.property.BooleanProperty",
  );
  renames.insert(
    "net.minecraft.block.properties.PropertyEnum",
    "net.minecraft.state.property.EnumProperty",
  );
  renames.insert(
    "net.minecraft.block.properties.PropertyInteger",
    "net.minecraft.state.property.IntProperty",
  );
  renames
    .insert("net.minecraft.block.properties.IProperty", "net.minecraft.state.property.Property");

  // items
  renames.insert("net.minecraft.item.ItemBlock", "net.minecraft.item.BlockItem");
  renames.insert("net.minecraft.item.ItemDoor", "net.minecraft.item.TallBlockItem");

  // entities
  renames
    .insert("net.minecraft.entity.player.EntityPlayer", "net.minecraft.entity.player.PlayerEntity");
  renames.insert("net.minecraft.entity.EntityLivingBase", "net.minecraft.entity.LivingEntity");

  // world
  renames.insert("net.minecraft.world.IBlockAccess", "net.minecraft.world.BlockView");

  // client
  renames.insert("net.minecraft.client.Minecraft", "net.minecraft.client.MinecraftClient");

  let config = Config { renames };

  let output = config.process(source);

  print!("{}", output);
}

impl Config {
  fn keep_import(&self, path: &str) -> bool {
    if self.renames.contains_key(path) {
      return true;
    }

    if path.starts_with("net.minecraftforge") || path.starts_with("javax") {
      return false;
    }

    // Known bad imports should get removed.
    match path {
      "net.minecraft.block.state.BlockStateContainer"
      | "net.minecraft.block.BlockPlanks"
      | "net.minecraft.creativetab.CreativeTabs"
      | "net.minecraft.util.BlockRenderLayer"
      | "net.minecraft.util.DamageSource"
      | "net.minecraft.util.NonNullList" => false,
      _ => true,
    }
  }

  fn process(&self, source: String) -> String {
    let mut imports = HashMap::<String, String>::new();

    if source.starts_with("// # only v12 #") {
      return String::new();
    }

    let mut output = Output::new(source.clone());
    let mut parser = Parser::new(&source);
    let mut package = String::new();

    // Eat the package statement
    if parser.next() == Some(Token::Word) && parser.slice() == "package" {
      while parser.slice() != ";" {
        parser.next();
        if parser.slice() != ";" {
          package.push_str(parser.slice());
        }
      }
    }

    while parser.next() == Some(Token::Word) && parser.slice() == "import" {
      let line_start = parser.range().start;

      let (path, span) = parse_path(&mut parser);
      if !self.keep_import(&path) {
        // +2 includes `;` and `\n`.
        output.replace(line_start..span.end + 2, "");
        continue;
      }

      let (_, last_part) = path.rsplit_once('.').unwrap();
      imports.insert(last_part.to_string(), path.clone());

      if let Some(new_path) = self.renames.get(path.as_str()) {
        output.replace(span, new_path);
      }
    }

    while let Some(tok) = parser.next() {
      match tok {
        Token::FabricComment => {
          // The comment is formed like:
          // ```
          // /* #fabric: <...> */
          // ```

          const PREFIX: &str = "/* #fabric:";
          const SUFFIX: &str = "*/";

          let source = parser.slice();
          let source_start = source.find(PREFIX).unwrap() + PREFIX.len();
          let source_end = source.rfind(SUFFIX).unwrap();

          output.replace(parser.range(), source[source_start..source_end].trim());
        }
        Token::V12Comment => {
          // The comment is formed like:
          // ```
          // // #v12-start
          // <...>
          // // #v12-end
          // ```

          const PREFIX: &str = "// #v12-start";
          const SUFFIX: &str = "// #v12-end";

          let source = parser.slice();
          let source_start = source.find(PREFIX).unwrap() + PREFIX.len();
          let source_end = source.rfind(SUFFIX).unwrap();

          let mut indent = 0;
          let mut i = parser.range().start + source_start - PREFIX.len();
          loop {
            let Some(c) = parser.src.get(i - 1..i) else {
              break;
            };

            if c == " " {
              indent += 1;
              i -= 1;
            } else {
              break;
            }
          }

          let indent_str = " ".repeat(indent);

          let mut i = source_start;
          let eat_line = |i: &mut usize| {
            loop {
              let range = parser.range().start + *i..parser.range().start + *i + 1;
              match parser.src.get(range) {
                None => panic!("unexpected end of file"),
                Some("\n") => break,
                Some(_) => *i += 1,
              }
            }
            *i += 1;
          };
          eat_line(&mut i);
          while i < source_end {
            let start = i;
            eat_line(&mut i);

            if i > source_end {
              break;
            }

            let mut line = &parser.src[parser.range().start + start..parser.range().start + i];
            line = line.strip_prefix(&indent_str).unwrap_or(line);
            output.replace(
              parser.range().start + start..parser.range().start + i,
              &format!("{indent_str}// {line}"),
            );
          }
        }

        Token::Word
          if parser.slice() == "BlockStateContainer" && package == "net.macmv.rgen.block" =>
        {
          macro_rules! t {
            ($tok:expr) => {
              if parser.next() != Some($tok) {
                panic!("expected {:?}, found {:?}", $tok, parser.slice());
              }
            };
            ($tok:expr, $slice:expr) => {
              if parser.next() != Some($tok) && parser.slice() != $slice {
                panic!("expected {:?}, found {:?}", $slice, parser.slice());
              }
            };
          }

          let start = parser.range().start;

          t!(Token::Word, "createBlockState");
          t!(Token::Punct, "(");
          t!(Token::Punct, ")");
          t!(Token::Punct, "{");
          t!(Token::Word, "return");
          t!(Token::Word, "new");
          t!(Token::Word, "BlockStateContainer");
          t!(Token::Punct, "(");
          t!(Token::Word, "this");
          t!(Token::Punct, ",");

          let mut props = vec![];
          loop {
            if parser.next() != Some(Token::Word) {
              panic!("expected word, found {:?}", parser.slice());
            }

            props.push(parser.slice());
            match parser.next() {
              Some(Token::Punct) if parser.slice() == "," => {}
              Some(Token::Punct) if parser.slice() == ")" => break,
              _ => panic!("expected ',' or ')', found {:?}", parser.slice()),
            }
          }

          t!(Token::Punct, ";");
          t!(Token::Punct, "}");

          let end = parser.range().end;

          output.replace(
            start..end,
            &format!(
              "void appendProperties(net.minecraft.state.StateManager.Builder<Block, BlockState> builder) {{\n    builder.add({});\n  }}",
              props.join(", ")
            ),
          );
        }
        Token::Word if parser.slice() == "super" && package == "net.macmv.rgen.block" => {
          if parser.next() == Some(Token::Punct) && parser.slice() == "(" {
            if parser.next() == Some(Token::Word) {
              if parser.next() == Some(Token::Punct) && parser.slice() == "." {
                let dot_range = parser.range();
                if parser.next() == Some(Token::Word) && parser.slice() == "material" {
                  output.replace(dot_range, "");
                  output.replace(parser.range(), "");
                }
              }
            }
          }
        }
        Token::Word if parser.slice() == "blockState" && package == "net.macmv.rgen.block" => {
          let start = parser.range().start;
          if parser.next() == Some(Token::Punct) && parser.slice() == "." {
            if parser.next() == Some(Token::Word) && parser.slice() == "getBaseState" {
              output.replace(start..parser.range().end, "getDefaultState");
            }
          }
        }
        Token::Word if parser.slice() == "BlockSettings" && package == "net.macmv.rgen.block" => {
          output.replace(parser.range(), "Settings");
        }
        Token::Word if parser.slice() == "getName" && package == "net.macmv.rgen.block" => {
          output.replace(parser.range(), "asString");
        }
        // Property#create -> Property#of
        Token::Word if parser.slice() == "create" && package == "net.macmv.rgen.block" => {
          output.replace(parser.range(), "of");
        }
        // getValue(Property) -> get(Property)
        Token::Word if parser.slice() == "getValue" && package == "net.macmv.rgen.block" => {
          output.replace(parser.range(), "get");
        }
        Token::Word if parser.slice() == "withProperty" && package == "net.macmv.rgen.block" => {
          output.replace(parser.range(), "with");
        }
        Token::Word if parser.slice() == "NULL_AABB" && package == "net.macmv.rgen.block" => {
          output.replace(parser.range(), "net.minecraft.util.shape.VoxelShapes.empty()");
        }
        Token::Word if parser.slice() == "getBoundingBox" && package == "net.macmv.rgen.block" => {
          output.replace(parser.range(), "getOutlineShape");
        }

        Token::Word if parser.slice() == "new" => {
          let start = parser.range().start;
          if parser.next() == Some(Token::Word) && parser.slice() == "AxisAlignedBB" {
            output
              .replace(start..parser.range().end, "net.minecraft.util.shape.VoxelShapes.cuboid");
          }
        }

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

        if parser.slice() == "static" {
          continue;
        }
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
