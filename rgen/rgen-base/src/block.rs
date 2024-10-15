use std::collections::HashMap;

use crate::{PropMap, PropMapOwned, PropType};

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

/// A realized biome ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BiomeId(pub u8);

impl BiomeId {
  pub const VOID: BiomeId = BiomeId(127);
}

/// A block state represents a block with a specific data value (like wool
/// color).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BlockState {
  pub block: BlockKind,
  pub state: StateOrProps,
}

/// A compressed enum. The states 0-15 are for placing with an explicit data,
/// whereas the state 16 is to place the default state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateOrProps {
  Default,
  Meta(u8),
  Props(PropMap),
}

impl StateOrProps {
  pub const fn meta(state: u8) -> StateOrProps {
    assert!(state < 16);
    StateOrProps::Meta(state)
  }

  pub fn is_default(&self) -> bool { matches!(self, StateOrProps::Default) }
  pub fn state(&self) -> Option<u8> {
    match self {
      StateOrProps::Default => None,
      StateOrProps::Props(_) => None,
      StateOrProps::Meta(m) => Some(*m),
    }
  }
}

/// Stores data about a block, like its default states and properties.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BlockData {
  pub name:         String,
  pub block:        Option<BlockKind>,
  pub default_meta: u8,

  pub prop_types:  HashMap<String, PropType>,
  pub prop_values: [PropMapOwned; 16],
}

impl BlockData {
  /// Creates a block state with the given data value, from 0 to 15 inclusive.
  /// Prefer `with_property` when possible, as that will use the named
  /// properties, which are almost always clearer.
  pub fn with_data(&self, data: u8) -> BlockState {
    assert!(data < 16);
    match self.block {
      Some(block) => BlockState { block, state: StateOrProps::meta(data) },
      None => panic!("cannot construct a block state without a constant block definition"),
    }
  }

  /// Creates a block state with the given property value.
  ///
  /// For example, you could set the color of wool with `.with_property("color",
  /// "lime")`.
  ///
  /// NOTE: This is note implemented yet, pulling out properties from java is a
  /// pain.
  pub fn with_property(&self, _key: &str, _value: &str) -> BlockState { todo!() }
}

/// A block read from the world. This is a specific state of a block data, that
/// can be compared against `BlockState`s, and used to get the block's
/// properties.
#[derive(Debug, Clone, Copy)]
pub struct BlockInfo<'a> {
  pub(crate) data:  &'a BlockData,
  pub(crate) state: StateId,
}

impl BlockInfo<'_> {
  // NB: Do not use! Only meant for `rgen-world` to construct.
  pub fn new(data: &BlockData, state: StateId) -> BlockInfo { BlockInfo { data, state } }

  pub fn block_kind(&self) -> BlockKind { self.data.block.unwrap_or(BlockKind::Air) }
  pub fn meta(&self) -> u8 { self.state.meta() }
}

impl BlockKind {
  /// Creates a block state with the given data value, from 0 to 15 inclusive.
  pub fn with_data(&self, data: u8) -> BlockState {
    assert!(data < 16);
    BlockState { block: *self, state: StateOrProps::meta(data) }
  }
}

impl BlockState {
  pub const AIR: BlockState = BlockState { block: BlockKind::Air, state: StateOrProps::meta(0) };

  /// Creates a block state with the given data value, from 0 to 15 inclusive.
  pub fn with_data(&self, data: u8) -> BlockState { self.block.with_data(data) }
}

#[allow(clippy::derivable_impls)]
impl Default for Biome {
  fn default() -> Biome { Biome::Void }
}

impl PartialEq<BlockKind> for BlockState {
  fn eq(&self, other: &BlockKind) -> bool { self.block == *other }
}
impl PartialEq<BlockState> for BlockKind {
  fn eq(&self, other: &BlockState) -> bool { *self == other.block && other.state.is_default() }
}

impl PartialEq<BlockKind> for BlockInfo<'_> {
  fn eq(&self, other: &BlockKind) -> bool { self.data.block == Some(*other) }
}

// NB: Default meta on `other` is considered a match-all.
impl PartialEq<BlockState> for BlockInfo<'_> {
  fn eq(&self, other: &BlockState) -> bool {
    self.data.block == Some(other.block)
      && match other.state.state() {
        Some(m) => self.state.meta() == m,
        None => true,
      }
  }
}

#[macro_export]
macro_rules! block {
  // block![stone[variant = andesite]]
  ($b1:ident $(:$b2:ident)? [$($key:ident = $value:expr),*]) => {
    $crate::BlockState {
      block: $crate::block_kind![$b1 $(:$b2)?],
      state: $crate::StateOrProps::Props($crate::PropMap::new(&[
        $(($crate::prop_name![$key], $crate::PropValue::from($value)),)*
      ])),
    }
  };

  // block![minecraft:stone[2]]
  ($b1:ident $(:$b2:ident)? [$state:expr]) => {
    $crate::BlockState {
      block: $crate::block_kind![$b1 $(:$b2)?],
      state: $crate::StateOrProps::meta($state),
    }
  };

  // block![minecraft:stone]
  ($b1:ident $(:$b2:ident)?) => {
    $crate::BlockState {
      block: $crate::block_kind![$b1 $(:$b2)?],
      state: $crate::StateOrProps::Default,
    }
  };
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
      // block_kind![air]
      ($default_name) => { $crate::$enum_name::$default_id };
      // block_kind![minecraft:air]
      ($default_namespace:$default_name) => { $crate::$enum_name::$default_id };
      // block_kind![stone] -> block_kind![minecraft:stone]
      ($block_name:ident) => { $crate::$macro_name![$default_namespace:$block_name] };
      $(
        // block_kind![rgen:log]
        ($namespace:$name) => { $crate::$enum_name::$id };
      )*

      ($other_namespace:ident:$other:ident) => {
        compile_error!(concat!("unknown block ", stringify!($other_namespace), ":", stringify!($other)))
      };
    }

    impl $enum_name {
      pub fn name(&self) -> &'static str {
        match self {
          $(
            Self::$id => concat!(stringify!($namespace), ":", stringify!($name)),
          )*
          _ => concat!(stringify!($default_namespace), ":", stringify!($default_name)),
        }
      }

      pub fn by_name(name: &str) -> Option<Self> {
        match name {
          s if s == concat!(stringify!($default_namespace), ":", stringify!($default_name)) => Some(Self::$default_id),
          $(s if s == concat!(stringify!($namespace), ":", stringify!($name)) => Some(Self::$id),)*
          _ => None
        }
      }

      pub const ALL: &[Self] = &[
        Self::$default_id,
        $(Self::$id,)*
      ];
    }
  };
}

big! { BlockKind, block_kind
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
  GrassPath => minecraft:grass_path,

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn block_by_name_works() {
    assert_eq!(BlockKind::by_name("minecraft:stone"), Some(BlockKind::Stone));
  }

  #[test]
  fn block_name_works() {
    assert_eq!(BlockKind::Stone.name(), "minecraft:stone");
  }
}
