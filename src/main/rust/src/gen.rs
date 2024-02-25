use std::sync::Mutex;

pub struct Generator {
  pub blocks: Blocks,
}

static GENERATOR: Mutex<Option<Generator>> = Mutex::new(None);

impl Generator {
  pub fn init<F>(lookup_id: F)
  where
    F: FnMut(&str) -> i32,
  {
    let gen = Generator { blocks: Blocks::init(lookup_id) };

    GENERATOR.lock().unwrap().replace(gen);
  }
}

pub struct Block(i32);

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
          $($id: Block(lookup_id($name)),)*
        }
      }
    }
  };
}

blocks! {
  dirt => "minecraft:dirt",
}
