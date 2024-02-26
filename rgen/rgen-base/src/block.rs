// TODO: If there's a static context set, Debug should print the block name.
#[derive(Debug, Clone, Copy, Eq)]
pub struct Block(pub(crate) u16);

// FIXME: Need to cleanup the block state interactions so this isn't a thing.
impl PartialEq for Block {
  fn eq(&self, other: &Block) -> bool { self.0 >> 4 == other.0 >> 4 }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Biome(pub(crate) u8);

impl Biome {
  // This is hardcoded to make my life easier. In reality it'll always be zero.
  pub const VOID: Biome = Biome(127);

  pub fn from_raw_id(id: i32) -> Biome {
    assert!(id >= 0 && id < 256);
    Biome(id as u8)
  }

  /// The biome ID.
  pub fn raw_id(&self) -> u8 { self.0 }
}

macro_rules! big {
  ($struct_name:ident: $item:ident $($id:ident => $name:expr,)*) => {
    pub struct $struct_name {
      $(pub $id: $item),*
    }

    impl $struct_name {
      pub fn init(mut lookup_id: impl FnMut(&str) -> i32) -> $struct_name {
        $struct_name {
          $($id: $item::from_raw_id(lookup_id($name)),)*
        }
      }

      /// Only public for testing.
      pub fn test_blocks() -> $struct_name {
        let mut id = 0;
        $struct_name {
          $($id: $item::from_raw_id({ id += 1; id }),)*
        }
      }
    }
  };
}

big! { Blocks: Block
  stone => "minecraft:stone",
  dirt => "minecraft:dirt",
  grass => "minecraft:grass",
  snow => "minecraft:snow",
  gravel => "minecraft:gravel",
  log => "minecraft:log",
  leaves => "minecraft:leaves",
}

big! { Biomes: Biome
  cold_taiga => "minecraft:taiga_cold",
  extreme_hills => "minecraft:extreme_hills",
  ice_plains => "minecraft:ice_flats",
  plains => "minecraft:plains",
  roofed_forest => "minecraft:roofed_forest",
  savanna => "minecraft:savanna",
  swamp => "minecraft:swampland",
}
