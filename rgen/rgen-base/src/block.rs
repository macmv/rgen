// TODO: If there's a static context set, Debug should print the block name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block(pub(crate) u16);

impl Block {
  // This is hardcoded to make my life easier. In reality it'll always be zero.
  pub const AIR: Block = Block(0);

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
      pub fn init<F>(mut lookup_id: F) -> Blocks
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
