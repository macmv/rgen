macro_rules! ids {
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

/// A realized biome ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BiomeId(pub u8);

impl BiomeId {
  pub const VOID: BiomeId = BiomeId(127);
}

#[allow(clippy::derivable_impls)]
impl Default for Biome {
  fn default() -> Biome { Biome::Void }
}

ids! { Biome, biome
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
