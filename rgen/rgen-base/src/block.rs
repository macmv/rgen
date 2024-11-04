use std::collections::HashMap;

use crate::{PropMap, PropMapOwned, PropType, PropValue};

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
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BlockState {
  pub block: BlockKind,
  pub state: StateOrProps,
}

impl BlockState {
  // Use `block![]` instead.
  #[doc(hidden)]
  #[track_caller]
  pub fn new(block: BlockKind, state: StateOrProps) -> Self {
    let state = BlockState { block, state };
    state.check();
    state
  }

  /// Validates `self.state` against the properties defined for `self.block`.
  #[track_caller]
  fn check(&self) {
    match self.state {
      StateOrProps::Default => {}
      StateOrProps::Meta(m) => assert!(m < 16),
      StateOrProps::Props(p) => {
        let expected = self.block.expected_props();

        for (k, v) in p.entries() {
          assert!(
            expected.contains_key(k),
            "unexpected property for block {}: {}",
            self.block.name(),
            k
          );
          assert!(
            expected[k].matches(&v),
            "invalid property value for block {}: {} = {:?} (expected: {:?})",
            self.block.name(),
            k,
            v,
            expected[k]
          );
        }
      }
    }
  }

  /// Sets the property `key` to `value`.
  ///
  /// # Panics
  ///
  /// If the property is not defined on this block, this will panic. If the
  /// value is not allowed for this block, this will panic.
  #[track_caller]
  pub fn set_prop<'a>(&mut self, key: &str, value: impl Into<PropValue<'a>>) {
    match self.state {
      StateOrProps::Default => {
        let mut m = PropMap::new(&[]);
        m.insert(key, value.into());
        self.state = StateOrProps::Props(m);
      }
      StateOrProps::Meta(_) => panic!("cannot set properties on a block with a data value"),
      StateOrProps::Props(ref mut props) => {
        props.insert(key, value.into());
      }
    }

    self.check();
  }

  /// Sets the property `key` to `value`, returning the updated block state.
  ///
  /// # Panics
  ///
  /// If the property is not defined on this block, this will panic. If the
  /// value is not allowed for this block, this will panic.
  #[track_caller]
  pub fn with_prop<'a>(mut self, key: &str, value: impl Into<PropValue<'a>>) -> BlockState {
    self.set_prop(key, value);
    self
  }
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
    $crate::BlockState::new(
      $crate::block_kind![$b1 $(:$b2)?],
      $crate::StateOrProps::Props($crate::PropMap::new(&[
        $(($crate::prop_name![$key], $crate::PropValue::from($value)),)*
      ])),
    )
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

macro_rules! blocks {
  (
    $default_id:ident => $default_namespace:ident:$default_name:ident,
    $($id:ident => $namespace:ident:$name:ident $([$($prop_key:ident: $prop_value:expr),* $(,)?])?,)*
  ) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BlockKind {
      $default_id,
      $($id,)*
    }

    #[macro_export]
    macro_rules! block_kind {
      // block_kind![air]
      ($default_name) => { $crate::BlockKind::$default_id };
      // block_kind![minecraft:air]
      ($default_namespace:$default_name) => { $crate::BlockKind::$default_id };
      // block_kind![stone] -> block_kind![minecraft:stone]
      ($block_name:ident) => { $crate::block_kind![$default_namespace:$block_name] };
      $(
        // block_kind![rgen:log]
        ($namespace:$name) => { $crate::BlockKind::$id };
      )*

      ($other_namespace:ident:$other:ident) => {
        compile_error!(concat!("unknown block ", stringify!($other_namespace), ":", stringify!($other)))
      };
    }

    impl BlockKind {
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

      pub fn expected_props(&self) -> HashMap<String, PropType> {
        match self {
          $(
            Self::$id => HashMap::from_iter([$($(
              (String::from(stringify!($prop_key)), $crate::PropType::from($prop_value)),
            )*)?]),
          )*
          _ => HashMap::new(),
        }
      }
    }
  };
}

const WOOD_4: [&str; 4] = ["oak", "spruce", "birch", "jungle"];
const WOOD_6: [&str; 6] = ["oak", "spruce", "birch", "jungle", "acacia", "dark_oak"];

const RGEN_WOOD_4_1: [&str; 4] = ["fir", "palm", "sakura", "cedar"];
const RGEN_WOOD_4_2_LOG: [&str; 2] = ["mangrove", "dead"];
const RGEN_WOOD_4_2_LEAVES: [&str; 3] = ["mangrove", "lavender", "seasonal"];
const RGEN_WOOD_4_3: [&str; 1] = ["aspen"];

const COLOR: [&str; 16] = [
  "white",
  "orange",
  "magenta",
  "lightblue",
  "yellow",
  "lime",
  "pink",
  "gray",
  "silver",
  "cyan",
  "purple",
  "blue",
  "brown",
  "green",
  "red",
  "black",
];

blocks! {
  Air => minecraft:air,

  Stone => minecraft:stone[
    variant: ["stone", "granite", "smooth_granite", "diorite", "smooth_diorite", "andesite", "smooth_andesite"],
  ],
  Dirt => minecraft:dirt[
    snowy: PropType::Bool,
    variant: ["dirt", "coarse_dirt", "podzol"],
  ],
  Clay => minecraft:clay,
  Grass => minecraft:grass[snowy: PropType::Bool],
  Snow => minecraft:snow,
  SnowLayer => minecraft:snow_layer[layers: 1..=8],
  Sand => minecraft:sand[variant: ["sand", "red_sand"]],
  Gravel => minecraft:gravel,
  Log => minecraft:log[axis: ["x", "y", "z", "none"], variant: WOOD_4],
  Leaves => minecraft:leaves[
    check_decay: PropType::Bool,
    decayable: PropType::Bool,
    variant: WOOD_4,
  ],
  Water => minecraft:water[level: 0..=15],
  Concrete => minecraft:concrete[color: COLOR],
  Cobblestone => minecraft:cobblestone,
  MossyCobblestone => minecraft:mossy_cobblestone,
  Ice => minecraft:ice,
  PackedIce => minecraft:packed_ice,
  Tallgrass => minecraft:tallgrass[type: ["dead_bush", "tall_grass", "fern"]],
  DoublePlant => minecraft:double_plant[
    facing: ["north", "south", "west", "east"],
    half: ["upper", "lower"],
    variant: ["sunflower", "syringa", "double_grass", "double_fern", "double_rose", "paeonia"],
  ],
  RedFlower => minecraft:red_flower[
    type: ["poppy", "blue_orchid", "allium", "houstonia", "red_tulip", "orange_tulip", "white_tulip", "pink_tulip", "oxeye_daisy"],
  ],
  YellowFlower => minecraft:yellow_flower[type: ["dandelion"]],
  Sandstone => minecraft:sandstone[
    type: ["sandstone", "chiseled_sandstone", "smooth_sandstone"],
  ],
  RedSandstone => minecraft:red_sandstone[
    type: ["red_sandstone", "chiseled_red_sandstone", "smooth_red_sandstone"],
  ],
  GoldBlock => minecraft:gold_block,
  HardenedClay => minecraft:hardened_clay,
  StainedHardenedClay => minecraft:stained_hardened_clay[color: COLOR],
  Planks => minecraft:planks[variant: WOOD_6],
  GlassPane => minecraft:glass_pane[
    east: PropType::Bool,
    north: PropType::Bool,
    south: PropType::Bool,
    west: PropType::Bool,
  ],
  Wool => minecraft:wool[color: COLOR],
  Lava => minecraft:lava[level: 0..=15],
  BrownMushroom => minecraft:brown_mushroom,
  Cocoa => minecraft:cocoa[
    age: 0..=2,
    facing: ["north", "south", "west", "east"],
  ],
  GrassPath => minecraft:grass_path,
  CoalOre => minecraft:coal_ore,
  IronOre => minecraft:iron_ore,
  GoldOre => minecraft:gold_ore,
  LapisOre => minecraft:lapis_ore,
  RedstoneOre => minecraft:redstone_ore,
  DiamondOre => minecraft:diamond_ore,
  EmeraldOre => minecraft:emerald_ore,
  IronBlock => minecraft:iron_block,

  RgenLog => rgen:log[axis: ["x", "y", "z", "none"], variant: RGEN_WOOD_4_1],
  RgenLog2 => rgen:log2[axis: ["x", "y", "z", "none"], variant: RGEN_WOOD_4_2_LOG],
  RgenLeaves => rgen:leaves[
    check_decay: PropType::Bool,
    decayable: PropType::Bool,
    variant: RGEN_WOOD_4_1,
  ],
  RgenLeaves2 => rgen:leaves2[
    check_decay: PropType::Bool,
    decayable: PropType::Bool,
    variant: RGEN_WOOD_4_2_LEAVES,
  ],
  RgenLeaves3 => rgen:leaves3[
    check_decay: PropType::Bool,
    decayable: PropType::Bool,
    variant: RGEN_WOOD_4_3,
  ],
  RgenMossyStump => rgen:mossy_stump[
    axis: ["x", "y", "z", "none"],
    variant: ["oak", "birch"],
  ],
  RgenCoveredJungleLog => rgen:covered_jungle_log[
    axis:["x", "y", "z", "none"],
  ],
  RgenPolypore => rgen:polypore[
    facing: ["north", "south", "west", "east"],
    type: ["one", "two", "three"],
  ],
  RgenMossyCarpet => rgen:mossy_carpet,
  RgenFlower => rgen:flower[type: ["forgetmenot"]],
  RgenBamboo => rgen:bamboo[has_leaves: PropType::Bool, placement: ["standard", "x", "z", "xz"]],
  RgenGlowVine => rgen:glow_vine[
    east: PropType::Bool,
    west: PropType::Bool,
    north: PropType::Bool,
    south: PropType::Bool,
    up: PropType::Bool,
  ],
  RgenHangingVines => rgen:hanging_vines[
    type:["bottom","standard"],
  ],
  RgenMossyCobblestone => rgen:mossy_cobblestone_rgen,
  RgenMossyStone => rgen:mossy_stone,
  RgenPlant => rgen:plant,
  RgenMoss => rgen:mossy_block,
  RgenLavender => rgen:lavender_plant[
    variant: ["variant_1", "variant_2", "variant_3", "variant_4"],
  ],
  RgenTallLavender => rgen:double_tall_lavender_plant[
    half: ["upper", "lower"],
    variant: ["variant_1", "variant_2", "variant_3", "variant_4"],
  ],
  RgenJuvenileCactus => rgen:juvenile_cactus[
    age: ["zero", "one", "two", "three"],
    color: ["green", "blue", "yellow", "orange"],
  ],
  RgenCactus => rgen:cactus[color: ["green", "blue", "yellow", "orange"]],
  RgenCactusArm => rgen:cactus_arm[face: ["north", "east", "south", "west"]],
  RgenBasalt => rgen:basalt[axis: ["x", "y", "z"]],
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
