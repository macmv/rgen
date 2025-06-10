use std::collections::HashMap;

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
  fn process(&self, mut source: String) -> String {
    // Skip the `package` line
    let mut imports = HashMap::<String, String>::new();
    for line in source.lines().skip(1) {
      if let Some(path) = line.strip_prefix("import ") {
        if let Some(path) = path.strip_suffix(";") {
          let parts: Vec<&str> = path.split('.').collect();
          if parts.len() > 1 {
            let last_part = parts.last().unwrap();
            imports.insert(last_part.to_string(), path.to_string());
          }
        }
      }
    }

    source
  }
}
