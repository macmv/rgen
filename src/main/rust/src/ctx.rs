use std::sync::RwLock;

use crate::generator::Generator;

pub struct Context {
  pub generator: Generator,
  pub blocks:    Blocks,
}

static CONTEXT: RwLock<Option<Context>> = RwLock::new(None);

impl Context {
  pub fn init(lookup_id: impl FnMut(&str) -> i32, seed: i64) {
    let ctx =
      Context { generator: Generator::new(seed as u64), blocks: Blocks::init(lookup_id) };

    CONTEXT.write().unwrap().replace(ctx);
  }

  pub fn run<R>(f: impl FnOnce(&Context) -> R) -> R {
    let ctx = CONTEXT.read().unwrap();
    let ctx = ctx.as_ref().expect("Context not initialized");
    f(ctx)
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
