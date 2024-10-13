use std::collections::HashMap;

/// A realized block state. The least significant 4 bits are the data value, and
/// the most significant 12 bits are the block id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StateId(pub u16);

/// A realized block ID. This increments for 1 for each 16 state ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BlockId(pub u16);

impl StateId {
  pub const AIR: StateId = StateId(0);

  pub fn new(block: BlockId, meta: u8) -> StateId {
    assert!(meta < 16);
    StateId((block.0 << 4) | meta as u16)
  }

  pub fn block(&self) -> BlockId { BlockId(self.0 >> 4) }
  pub fn meta(&self) -> u8 { self.0 as u8 & 0x0f }
}

impl BlockId {
  pub const AIR: BlockId = BlockId(0);
}

/// A block state represents a block with a specific data value (like wool
/// color).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockState {
  pub block: Block,
  pub state: u8,
}

// FIXME: This should probably use the default state.
impl From<Block> for BlockState {
  fn from(val: Block) -> Self { BlockState { block: val, state: 0 } }
}
impl From<BlockInfo> for BlockState {
  fn from(val: BlockInfo) -> Self { val.default_state }
}

/// Stores info about a block, like its default states and properties.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BlockInfo {
  pub name:          String,
  pub block:         Block,
  pub default_state: BlockState,

  prop_map: HashMap<String, HashMap<String, u8>>,
}

impl BlockInfo {
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

impl Block {
  /// Creates a block state with the given data value, from 0 to 15 inclusive.
  pub fn with_data(&self, data: u8) -> BlockState {
    assert!(data < 16);
    BlockState { block: *self, state: data }
  }
}

impl BlockState {
  pub const AIR: BlockState = BlockState { block: Block::Air, state: 0 };

  /// Creates a block state with the given data value, from 0 to 15 inclusive.
  pub fn with_data(&self, data: u8) -> BlockState { self.block.with_data(data) }
}

impl Default for Biome {
  fn default() -> Biome { Biome::Void }
}

// Block Identification Guide
macro_rules! big {
  (
    $enum_name:ident, $macro_name:ident
    $default_id:ident => $default_namespace:ident:$default_name:ident,
    $($id:ident => $namespace:ident:$name:ident,)*
  ) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum $enum_name {
      $default_id,
      $($id,)*
    }

    #[macro_export]
    macro_rules! $macro_name {
      // block![stone[2]]
      ($block_name:ident [$state:expr]) => {
        $crate::BlockState {
          block: block![$block_name],
          state: $state,
        }
      };
      // block![minecraft:stone[2]]
      ($block_namespace:ident:$block_name:ident [$state:expr]) => {
        $crate::BlockState {
          block: block![$block_namespace:$block_name],
          state: $state,
        }
      };

      // block![air]
      ($default_name) => { $crate::$enum_name::$default_id };
      // block![minecraft:air]
      ($default_namespace:$default_name) => { $crate::$enum_name::$default_id };
      $(
        // block![stone]
        ($name) => { $crate::$macro_name![$default_namespace:$name] };
        // block![minecraft:stone]
        ($namespace:$name) => { $crate::$enum_name::$id };
      )*
    }

    impl $enum_name {
      pub fn name_of(&self) -> &'static str {
        match self {
          $(
            Self::$id => stringify!($namespace:$name),
          )*
          _ => stringify!($default_namespace:$default_name),
        }
      }

      pub fn by_name(name: &str) -> Option<Self> {
        match name {
          s if s == stringify!($default_namespace:$default_name) => Some(Self::$default_id),
          $(s if s == stringify!($namespace:$name) => Some(Self::$id),)*
          _ => None
        }
      }
    }
  };
}

big! { Block, block
  Air => minecraft:air,

  Stone => minecraft:stone,
  Dirt => minecraft:dirt,
  Clay => minecraft:clay,
  Grass => minecraft:grass,
  Snow => minecraft:snow,
  SnowLayer => minecraft:snow_layer,
  Sand => minecraft:sand,
  Gravel => minecraft:gravel,
  Log => minecraft:log,
  Leaves => minecraft:leaves,
  Water => minecraft:water,
  Concrete => minecraft:concrete,
  Cobblestone => minecraft:cobblestone,
  MossyCobblestone => minecraft:mossy_cobblestone,
  Ice => minecraft:ice,
  PackedIce => minecraft:packed_ice,
  Tallgrass => minecraft:tallgrass,
  DoublePlant => minecraft:double_plant,
  RedFlower => minecraft:red_flower,
  YellowFlower => minecraft:yellow_flower,
  // 0 - normal    1 - chiseled     2 - smooth
  Sandstone => minecraft:sandstone,
  // 0 - normal    1 - chiseled     2 - smooth (?)
  RedSandstone => minecraft:red_sandstone,
  GoldBlock => minecraft:gold_block,
  HardenedClay => minecraft:hardened_clay,
  StainedHardenedClay => minecraft:stained_hardened_clay,
  Planks => minecraft:planks,
  GlassPane => minecraft:glass_pane,
  Wool => minecraft:wool,
  Lava => minecraft:lava,
  IronOre => minecraft:iron_ore,
  BrownMushroom => minecraft:brown_mushroom,
  Cocoa => minecraft:cocoa,

  RgenLog => rgen:log,
  RgenLog2 => rgen:log2,
  RgenLeaves => rgen:leaves,
  RgenLeaves2 => rgen:leaves2,
  RgenLeaves3 => rgen:leaves3,
  RgenMossyStump => rgen:mossy_stump,
  RgenPolypore => rgen:polypore,
  RgenMossyCarpet => rgen:mossy_carpet,
  RgenFlower => rgen:flower,
  RgenBamboo => rgen:bamboo,
  RgenGlowVine => rgen:glow_vine,
  RgenMossyCobblestone => rgen:mossy_cobblestone_rgen,
  RgenMossyStone => rgen:mossy_stone,
  RgenPlant => rgen:plant,
  RgenMoss => rgen:mossy_block,
  RgenLavender => rgen:lavender_plant,
  RgenTallLavender => rgen:double_tall_lavender_plant,
  RgenJuvenileCactus => rgen:juvenile_cactus,
  RgenCactus => rgen:cactus,
  RgenCactusArm => rgen:cactus_arm,
  RgenBasalt => rgen:basalt,
}

big! { Biome, biome
  Void => minecraft:void,

  ColdTaiga => minecraft:taiga_cold,
  Taiga => minecraft:taiga,
  ExtremeHills => minecraft:extreme_hills,
  IcePlains => minecraft:ice_flats,
  Plains => minecraft:plains,
  Beaches => minecraft:beaches,
  RoofedForest => minecraft:roofed_forest,
  Savanna => minecraft:savanna,
  Swamp => minecraft:swampland,
  StoneBeach => minecraft:stone_beach,
  Jungle => minecraft:jungle,
  BirchForest => minecraft:birch_forest_hills,
  River => minecraft:river,
  Mesa => minecraft:mesa,
  Desert => minecraft:desert,
}
