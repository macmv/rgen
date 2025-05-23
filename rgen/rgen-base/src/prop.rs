use std::{fmt, ops::RangeInclusive};

#[derive(Clone, Copy, Eq)]
pub struct PropMap {
  // Garuntee: There cannot be more than 8 properties on a block.
  //
  // Also, this will always be sorted by key.
  entries: [(Option<PropName>, PropValueCompact); 8],
}

/// Stores a property value. The bits are:
/// - 0 and 1: bools.
/// - 2 through 17: ints (vanilla only uses integers from 0 to 16).
/// - 18 through 166: enums.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PropValueCompact(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropValue<'a> {
  Bool(bool),
  Int(i32),
  Enum(&'a str),
}

#[derive(Clone, Eq)]
pub struct PropMapOwned {
  // Garuntee: There cannot be more than 8 properties on a block.
  //
  // Also, this will always be sorted by key.
  pub entries: [(String, PropValueOwned); 8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropValueOwned {
  Bool(bool),
  Int(i32),
  Enum(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropType {
  Bool,
  Int(i32, i32),
  Enum(Vec<String>),
}

impl From<bool> for PropValue<'_> {
  fn from(value: bool) -> Self { PropValue::Bool(value) }
}
impl From<i32> for PropValue<'_> {
  fn from(value: i32) -> Self { PropValue::Int(value) }
}
impl From<&'static str> for PropValue<'_> {
  fn from(value: &'static str) -> Self { PropValue::Enum(value) }
}

impl From<RangeInclusive<i32>> for PropType {
  fn from(range: RangeInclusive<i32>) -> Self { PropType::Int(*range.start(), *range.end()) }
}
impl<const N: usize> From<[&str; N]> for PropType {
  fn from(values: [&str; N]) -> Self {
    PropType::Enum(values.into_iter().map(|s| s.to_string()).collect())
  }
}

impl PropType {
  pub fn matches(&self, value: &PropValue) -> bool {
    match (self, value) {
      (PropType::Bool, PropValue::Bool(_)) => true,
      (PropType::Int(min, max), PropValue::Int(value)) => min <= value && value <= max,
      (PropType::Enum(allowed), PropValue::Enum(value)) => allowed.contains(&value.to_string()),
      _ => false,
    }
  }
}

impl fmt::Debug for PropMap {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_map().entries(self.entries()).finish()
  }
}

impl fmt::Debug for PropMapOwned {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_map().entries(self.entries()).finish()
  }
}

impl fmt::Display for PropMap {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, (key, value)) in self.entries().enumerate() {
      if i != 0 {
        write!(f, ",")?;
      }

      write!(f, "{key}={value},")?;
    }
    Ok(())
  }
}

impl fmt::Display for PropMapOwned {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, (key, value)) in self.entries().enumerate() {
      if i != 0 {
        write!(f, ",")?;
      }

      write!(f, "{key}={value},")?;
    }
    Ok(())
  }
}

impl fmt::Display for PropValue<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PropValue::Bool(value) => write!(f, "{}", value),
      PropValue::Int(value) => write!(f, "{}", value),
      PropValue::Enum(value) => write!(f, "{}", value),
    }
  }
}

impl fmt::Display for PropValueOwned {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PropValueOwned::Bool(value) => write!(f, "{}", value),
      PropValueOwned::Int(value) => write!(f, "{}", value),
      PropValueOwned::Enum(value) => write!(f, "{}", value),
    }
  }
}

impl PropMap {
  pub const fn empty() -> Self { PropMap { entries: [(None, PropValueCompact(0)); 8] } }

  // Should be constructed with the `block![]` macro.
  #[doc(hidden)]
  #[track_caller]
  pub fn new(values: &[(PropName, PropValue<'static>)]) -> Self {
    if values.len() > 8 {
      panic!("too many properties");
    }

    let mut entries = [(None, PropValueCompact(0)); 8];
    for (i, (key, value)) in values.iter().enumerate() {
      entries[i] = (Some(*key), PropValueCompact::for_value_or_panic(*value));
    }

    entries[0..values.len()].sort_unstable_by(|a, b| a.0.cmp(&b.0));

    Self { entries }
  }

  pub fn len(&self) -> usize { self.entries.partition_point(|(e, _)| e.is_some()) }
  pub fn is_empty(&self) -> bool { self.len() == 0 }

  pub fn entries(&self) -> impl Iterator<Item = (&'_ str, PropValue)> + '_ {
    self.entries[0..self.len()]
      .iter()
      .copied()
      .map(|(key, value)| (key.unwrap().name(), value.as_value()))
  }

  fn partition_point(&self, name: &PropName) -> usize {
    self.entries.partition_point(|(e, _)| match e {
      Some(e) => e < name,
      None => false, // Empty entries at the end.
    })
  }

  // Precondition: `key` must be in the map.
  #[track_caller]
  fn set(&mut self, name: PropName, value: PropValue) {
    for entry in self.entries.iter_mut() {
      if entry.0 == Some(name) {
        entry.1 = PropValueCompact::for_value_or_panic(value);
        return;
      }
    }

    panic!("property '{name}' not found");
  }

  // Precondition: `key` must not be in the map.
  #[track_caller]
  fn add(&mut self, name: PropName, value: PropValue) {
    if self.len() == 8 {
      panic!("no more space for key '{name}'");
    }

    let index = self.partition_point(&name);

    // Given:
    // A A B B _ _ _ _
    //
    // We want to insert our element between `A` and `B`. So we have a quick and
    // dirty `sort` impl here.
    self.entries[index..].rotate_right(1); // A A _ B B _ _ _
    self.entries[index] = (Some(name), PropValueCompact::for_value_or_panic(value));
  }

  #[track_caller]
  pub fn insert(&mut self, key: &str, value: PropValue) {
    let name = PropName::for_name_or_panic(key);

    if self.contains_key(&name) {
      self.set(name, value);
    } else {
      self.add(name, value);
    }
  }

  #[track_caller]
  pub fn insert_if_unset(&mut self, key: &str, value: PropValue) {
    let name = PropName::for_name_or_panic(key);

    if self.contains_key(&name) {
      return;
    }

    self.add(name, value);
  }

  fn contains_key(&self, name: &PropName) -> bool {
    let index = self.partition_point(name);
    index != 8 && self.entries[index].0 == Some(*name)
  }
}

impl PropMapOwned {
  pub const fn empty() -> Self {
    PropMapOwned { entries: [const { (String::new(), PropValueOwned::Bool(false)) }; 8] }
  }

  pub fn entries(&self) -> impl Iterator<Item = (&str, PropValue)> + '_ {
    self.entries.iter().filter_map(|(key, value)| {
      if key.is_empty() { None } else { Some((&**key, value.as_value())) }
    })
  }

  #[track_caller]
  pub fn set(&mut self, key: String, value: PropValueOwned) {
    for entry in self.entries.iter_mut() {
      if entry.0 == key {
        *entry = (key, value);
        return;
      }
    }

    panic!("key '{key}' not found");
  }

  pub fn insert_if_unset(&mut self, key: String, value: PropValueOwned) {
    for entry in self.entries() {
      if entry.0 == key {
        return;
      }
    }

    for entry in self.entries.iter_mut() {
      if entry.0.is_empty() {
        *entry = (key, value);
        // FIXME: Insert this key in the right spot, instead of just sorting. This is a
        // somewhat hot path, so probably with optimizing at some point.
        self.entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        return;
      }
    }

    panic!("no more space for key '{key}'");
  }
}

impl PartialEq<PropMap> for PropMapOwned {
  fn eq(&self, other: &PropMap) -> bool { self.entries().eq(other.entries()) }
}
impl PartialEq<PropMapOwned> for PropMap {
  fn eq(&self, other: &PropMapOwned) -> bool { self.entries().eq(other.entries()) }
}

impl PartialEq<PropMap> for PropMap {
  fn eq(&self, other: &PropMap) -> bool { self.entries().eq(other.entries()) }
}
impl PartialEq<PropMapOwned> for PropMapOwned {
  fn eq(&self, other: &PropMapOwned) -> bool { self.entries().eq(other.entries()) }
}

impl PropValueOwned {
  pub fn as_value(&self) -> PropValue {
    match self {
      PropValueOwned::Bool(value) => PropValue::Bool(*value),
      PropValueOwned::Int(value) => PropValue::Int(*value),
      PropValueOwned::Enum(value) => PropValue::Enum(value),
    }
  }
}

// Properties are stored on the stack in `BlockState` directly, so we intern
// them to make that struct reasonably sized.
macro_rules! intern {
  (
    $enum_name:ident, $macro_name:ident
    $($id:ident => $name:ident,)*
  ) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(u8)]
    pub enum $enum_name {
      $($id,)*
    }

    #[macro_export]
    macro_rules! $macro_name {
      $(
        // prop_name![x]
        ($name) => { $crate::$enum_name::$id };
      )*

      ($other:ident) => {
        compile_error!(concat!("unknown property ", stringify!($other)))
      };
    }

    impl $enum_name {
      pub fn name(&self) -> &'static str {
        match self {
          $(
            Self::$id => stringify!($name),
          )*
        }
      }

      pub fn for_name(name: &str) -> Option<Self> {
        match name {
          $(stringify!($name) => Some(Self::$id),)*
          _ => None
        }
      }

      #[allow(dead_code)]
      pub const ALL: &[Self] = &[
        $(Self::$id,)*
      ];
    }
  };
}

intern! { PropName, prop_name
  Age         => age,
  Attached    => attached,
  Axis        => axis,
  Bites       => bites,
  CheckDecay  => check_decay,
  Color       => color,
  Conditional => conditional,
  Contents    => contents,
  Count       => count,
  Damage      => damage,
  Decayable   => decayable,
  Delay       => delay,
  Disarmed    => disarmed,
  Down        => down,
  East        => east,
  Enabled     => enabled,
  Explode     => explode,
  Extended    => extended,
  Eye         => eye,
  Face        => face,
  Facing      => facing,
  Half        => half,
  HasBottle0  => has_bottle_0,
  HasBottle1  => has_bottle_1,
  HasBottle2  => has_bottle_2,
  HasLeaves   => has_leaves,
  HasRecord   => has_record,
  Hinge       => hinge,
  InWall      => in_wall,
  Layers      => layers,
  LegacyData  => legacy_data,
  Level       => level,
  Locked      => locked,
  Mode        => mode,
  Moisture    => moisture,
  Nodrop      => nodrop,
  North       => north,
  Occupied    => occupied,
  Open        => open,
  Part        => part,
  Placement   => placement,
  Power       => power,
  Powered     => powered,
  Rotation    => rotation,
  Seamless    => seamless,
  Shape       => shape,
  Short       => short,
  Size        => size,
  Snowy       => snowy,
  South       => south,
  Stage       => stage,
  Triggered   => triggered,
  Type        => type,
  Up          => up,
  Variant     => variant,
  West        => west,
  Wet         => wet,
}

intern! { PropEnum, prop_enum
  Acacia               => acacia,
  AllInside            => all_inside,
  Allium               => allium,
  AllOutside           => all_outside,
  AllStem              => all_stem,
  Andesite             => andesite,
  AscendingEast        => ascending_east,
  AscendingNorth       => ascending_north,
  AscendingSouth       => ascending_south,
  AscendingWest        => ascending_west,
  Aspen                => aspen,
  Birch                => birch,
  Black                => black,
  Blue                 => blue,
  BlueOrchid           => blue_orchid,
  Bottom               => bottom,
  Brick                => brick,
  Brown                => brown,
  Cedar                => cedar,
  Center               => center,
  ChiseledBrick        => chiseled_brick,
  Chiseled             => chiseled,
  ChiseledRedSandstone => chiseled_red_sandstone,
  ChiseledSandstone    => chiseled_sandstone,
  ChiseledStonebrick   => chiseled_stonebrick,
  CoarseDirt           => coarse_dirt,
  Cobblestone          => cobblestone,
  Compare              => compare,
  Corner               => corner,
  CrackedBrick         => cracked_brick,
  CrackedStonebrick    => cracked_stonebrick,
  Cyan                 => cyan,
  Dandelion            => dandelion,
  DarkOak              => dark_oak,
  DarkPrismarine       => dark_prismarine,
  Data                 => data,
  DeadBush             => dead_bush,
  Dead                 => dead,
  Default              => default,
  Diorite              => diorite,
  Dirt                 => dirt,
  DoubleFern           => double_fern,
  DoubleGrass          => double_grass,
  DoubleRose           => double_rose,
  Down                 => down,
  DownX                => down_x,
  DownZ                => down_z,
  East                 => east,
  EastWest             => east_west,
  Empty                => empty,
  Fern                 => fern,
  Fir                  => fir,
  Foot                 => foot,
  ForgetMeNot          => forgetmenot,
  Granite              => granite,
  Gray                 => gray,
  Green                => green,
  Head                 => head,
  Houstonia            => houstonia,
  Jungle               => jungle,
  Large                => large,
  Lavender             => lavender,
  Left                 => left,
  LightBlue            => lightblue,
  Lime                 => lime,
  Lines                => lines,
  Load                 => load,
  Lower                => lower,
  Magenta              => magenta,
  Mangrove             => mangrove,
  Medium               => medium,
  MossyBrick           => mossy_brick,
  MossyCobblestone     => mossy_cobblestone,
  MossyStonebrick      => mossy_stonebrick,
  NetherBrick          => nether_brick,
  None                 => none,
  Normal               => normal,
  NorthEast            => north_east,
  North                => north,
  NorthSouth           => north_south,
  NorthWest            => north_west,
  Oak                  => oak,
  One                  => one,
  Orange               => orange,
  OrangeTulip          => orange_tulip,
  OxeyeDaisy           => oxeye_daisy,
  Paeonia              => paeonia,
  Palm                 => palm,
  Pink                 => pink,
  PinkTulip            => pink_tulip,
  Podzol               => podzol,
  Poppy                => poppy,
  PrismarineBricks     => prismarine_bricks,
  Prismarine           => prismarine,
  Purple               => purple,
  Quartz               => quartz,
  Red                  => red,
  RedSand              => red_sand,
  RedSandstone         => red_sandstone,
  RedTulip             => red_tulip,
  Right                => right,
  Sakura               => sakura,
  Sand                 => sand,
  Sandstone            => sandstone,
  Save                 => save,
  Seasonal             => seasonal,
  Silver               => silver,
  Small                => small,
  SmoothAndesite       => smooth_andesite,
  SmoothDiorite        => smooth_diorite,
  SmoothGranite        => smooth_granite,
  SmoothRedSandstone   => smooth_red_sandstone,
  SmoothSandstone      => smooth_sandstone,
  SouthEast            => south_east,
  South                => south,
  SouthWest            => south_west,
  Spruce               => spruce,
  Standard             => standard,
  Stem                 => stem,
  Sticky               => sticky,
  StoneBrick           => stone_brick,
  Stonebrick           => stonebrick,
  Stone                => stone,
  Straight             => straight,
  Subtract             => subtract,
  Sunflower            => sunflower,
  Syringa              => syringa,
  TallGrass            => tall_grass,
  Three                => three,
  Top                  => top,
  Two                  => two,
  Upper                => upper,
  Up                   => up,
  UpX                  => up_x,
  UpZ                  => up_z,
  Variant1             => variant_1,
  Variant2             => variant_2,
  Variant3             => variant_3,
  Variant4             => variant_4,
  West                 => west,
  WhiteTulip           => white_tulip,
  White                => white,
  WoodOld              => wood_old,
  X                    => x,
  Xz                   => xz,
  Yellow               => yellow,
  Y                    => y,
  Zero                 => zero,
  Z                    => z,
}

impl PropName {
  pub fn for_name_or_panic(name: &str) -> Self {
    match Self::for_name(name) {
      Some(prop) => prop,
      None => panic!("unknown property '{}'", name),
    }
  }
}

impl fmt::Display for PropName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.name()) }
}

impl PropValueCompact {
  pub fn for_value_or_panic(value: PropValue) -> Self {
    match value {
      PropValue::Bool(value) => PropValueCompact(value as u8),
      PropValue::Int(value) => {
        if !(0..=15).contains(&value) {
          panic!("int value out of range: {}", value);
        }
        PropValueCompact(value as u8 + 2)
      }
      PropValue::Enum(value) => {
        let value =
          PropEnum::for_name(value).unwrap_or_else(|| panic!("unknown enum value: {}", value));
        PropValueCompact(18 + value as u8)
      }
    }
  }

  pub fn as_value(&self) -> PropValue<'static> {
    match self.0 {
      0..=1 => PropValue::Bool(self.0 != 0),
      2..=17 => PropValue::Int(self.0 as i32 - 2),
      18..=166 => PropValue::Enum(PropEnum::ALL[(self.0 - 18) as usize].name()),
      _ => panic!("invalid compact prop value: {}", self.0),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn size_of_prop_map() {
    assert_eq!(std::mem::size_of::<PropMap>(), 16);
  }

  #[test]
  fn prop_map_new() {
    let map = PropMap::new(&[(prop_name![age], true.into()), (prop_name![axis], "x".into())]);

    assert_eq!(map.len(), 2);
    assert_eq!(map.entries().collect::<Vec<_>>(), [("age", true.into()), ("axis", "x".into())]);
  }

  #[test]
  fn prop_map_insert() {
    let mut map = PropMap::empty();

    map.insert("age", true.into());
    map.insert("axis", "x".into());

    assert_eq!(map.len(), 2);
    assert_eq!(map.entries().collect::<Vec<_>>(), [("age", true.into()), ("axis", "x".into())]);

    map.insert("age", true.into());

    assert_eq!(map.len(), 2);
    assert_eq!(map.entries().collect::<Vec<_>>(), [("age", true.into()), ("axis", "x".into())]);

    map.insert("age", false.into());

    assert_eq!(map.len(), 2);
    assert_eq!(map.entries().collect::<Vec<_>>(), [("age", false.into()), ("axis", "x".into())]);
  }

  #[test]
  fn prop_map_insert_if_unset() {
    let mut map = PropMap::empty();

    map.insert_if_unset("age", true.into());
    map.insert_if_unset("axis", "x".into());

    assert_eq!(map.len(), 2);
    assert_eq!(map.entries().collect::<Vec<_>>(), [("age", true.into()), ("axis", "x".into())]);

    map.insert_if_unset("age", false.into());

    assert_eq!(map.len(), 2);
    assert_eq!(map.entries().collect::<Vec<_>>(), [("age", true.into()), ("axis", "x".into())]);
  }
}
