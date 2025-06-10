fn main() {
  let filename = std::env::args().nth(1).expect("No filename provided");

  let mut source = std::fs::read_to_string(filename).expect("Failed to read file");
  source = source.replace("IBlockAccess", "BlockAccess");

  print!("{}", source);
}
