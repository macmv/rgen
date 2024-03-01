use std::collections::HashMap;

/// A block represents a block type (like dirt, stone, etc).
// TODO: If there's a static context set, Debug should print the block name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block(pub(crate) u16);

/// A block state represents a block with a specific data value (like wool
/// color).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockState {
  pub block: Block,
  pub state: u8,
}

// FIXME: This should probably use the default state.
impl Into<BlockState> for Block {
  fn into(self) -> BlockState { BlockState { block: self, state: 0 } }
}
impl Into<BlockState> for BlockInfo {
  fn into(self) -> BlockState { self.default_state }
}

impl Block {
  /// The raw ID used in the chunk data (air is 0, dirt is 16, etc).
  pub fn raw_id(&self) -> u16 { self.0 << 4 }
}

/// Stores info about a block, like its default states and properties.
#[derive(Debug, PartialEq, Eq)]
pub struct BlockInfo {
  pub name:          String,
  pub block:         Block,
  pub default_state: BlockState,

  prop_map: HashMap<String, HashMap<String, u8>>,
}

impl Block {
  pub const AIR: Block = Block(0);
}

impl BlockInfo {
  pub fn temp_new(name: &str, id: i32) -> BlockInfo {
    let state = BlockState::from_raw_id(id as u16);

    BlockInfo {
      name:          name.to_string(),
      block:         state.block,
      default_state: state,
      prop_map:      HashMap::new(),
    }
  }

  /// Creates a block state with the given data value, from 0 to 15 inclusive.
  /// Prefer `with_property` when possible, as that will use the named
  /// properties, which are almost always clearer.
  pub fn with_data(&self, data: u8) -> BlockState {
    assert!(data < 16);
    BlockState { block: self.block, state: data }
  }

  /// Creates a block state with the given property value.
  ///
  /// For example, you could set the color of wool with `.with_property("color",
  /// "lime")`.
  ///
  /// NOTE: This is note implemented yet, pulling out properties from java is a
  /// pain.
  pub fn with_property(&self, key: &str, value: &str) -> BlockState {
    let values = self
      .prop_map
      .get(key)
      .unwrap_or_else(|| panic!("Block {} does not have a property {}", self.name, key));
    let state = *values.get(value).unwrap_or_else(|| {
      panic!("Block {} property {} does not have key {}", self.name, key, value)
    });

    BlockState { block: self.block, state }
  }
}

impl BlockState {
  pub const AIR: BlockState = BlockState { block: Block::AIR, state: 0 };

  pub(crate) fn from_raw_id(id: u16) -> BlockState {
    BlockState { block: Block(id >> 4), state: (id & 0xf) as u8 }
  }

  /// Returns the state ID used in the chunk data.
  pub fn raw_id(&self) -> u16 {
    assert!(self.state < 16);
    self.block.raw_id() | (self.state as u16)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Biome(pub(crate) u8);

impl Biome {
  pub const VOID: Biome = Biome(127);

  pub fn from_raw_id(id: i32) -> Biome {
    assert!(id >= 0 && id < 256);
    Biome(id as u8)
  }

  /// The biome ID.
  pub fn raw_id(&self) -> u8 { self.0 }
}

macro_rules! big {
  (
    $struct_name:ident: $item:ident
    $default_name:ident => $default_str:literal = $default_id:expr,
    $($id:ident => $name:expr,)*
  ) => {
    pub struct $struct_name {
      $(pub $id: $item),*
    }

    impl $struct_name {
      pub fn init(mut lookup: impl FnMut(&str) -> $item) -> $struct_name {
        $struct_name {
          $($id: lookup($name),)*
        }
      }

      /// Only public for testing.
      pub fn test_blocks() -> $struct_name {
        // let mut id = 0;
        // $struct_name {
        //   $($id: $item::from_raw_id({ id += 1; id }),)*
        // }
        todo!()
      }

      pub fn name_of(&self, _v: $item) -> &'static str {
        todo!()
        // $(
        //   if v == self.$id { return $name }
        // )*
        // if v == $item::$default_name { return $default_str }
        // unreachable!();
      }
    }
  };
}

big! { Blocks: BlockInfo
  AIR => "minecraft:air" = 0,

  stone => "minecraft:stone",
  dirt => "minecraft:dirt",
  grass => "minecraft:grass",
  snow => "minecraft:snow",
  gravel => "minecraft:gravel",
  log => "minecraft:log",
  leaves => "minecraft:leaves",
  water => "minecraft:water",
  wool => "minecraft:wool",
  cobblestone => "minecraft:cobblestone",
  mossy_cobblestone => "minecraft:mossy_cobblestone",
  ice => "minecraft:ice",
}

big! { Biomes: Biome
  VOID => "minecraft:void" = 127,

  cold_taiga => "minecraft:taiga_cold",
  extreme_hills => "minecraft:extreme_hills",
  ice_plains => "minecraft:ice_flats",
  plains => "minecraft:plains",
  roofed_forest => "minecraft:roofed_forest",
  savanna => "minecraft:savanna",
  swamp => "minecraft:swampland",
}
