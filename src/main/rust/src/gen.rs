use std::sync::RwLock;

pub struct Generator {
  pub blocks: Blocks,
}

static GENERATOR: RwLock<Option<Generator>> = RwLock::new(None);

impl Generator {
  pub fn init(lookup_id: impl FnMut(&str) -> i32) {
    let gen = Generator { blocks: Blocks::init(lookup_id) };

    GENERATOR.write().unwrap().replace(gen);
  }

  pub fn run<R>(f: impl FnOnce(&Generator) -> R) -> R {
    let gen = GENERATOR.read().unwrap();
    let gen = gen.as_ref().expect("Generator not initialized");
    f(gen)
  }
}

#[derive(Clone, Copy)]
pub struct Block(u16);

impl Block {
  pub fn from_raw_id(id: i32) -> Block {
    assert!(id >= 0 && id < 4096);
    Block(id as u16)
  }

  /// The raw ID used in the chunk data (air is 0, dirt is 16, etc).
  pub fn raw_id(&self) -> u16 { self.0 }
}

macro_rules! blocks {
  ($($id:ident => $name:expr,)*) => {
    pub struct Blocks {
      $(pub $id: Block),*
    }

    impl Blocks {
      fn init<F>(mut lookup_id: F) -> Blocks
      where
        F: FnMut(&str) -> i32,
      {
        Blocks {
          $($id: Block::from_raw_id(lookup_id($name)),)*
        }
      }
    }
  };
}

blocks! {
  stone => "minecraft:stone",
  dirt => "minecraft:dirt",
}
