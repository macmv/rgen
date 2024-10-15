use std::fmt;

#[derive(Clone, Copy, Eq)]
pub struct PropMap {
  // Garuntee: There cannot be more than 8 properties on a block.
  //
  // Also, this will always be sorted by key.
  entries: [(Option<PropName>, PropValueCompact); 8],
}

/// Stores a property value. The bits are:
/// - 0 and 1: bools.
/// - 1 through 127: ints.
/// - 128 and 255: enums.
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

    entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    Self { entries }
  }

  pub fn len(&self) -> usize { self.entries().count() }
  pub fn is_empty(&self) -> bool { self.len() == 0 }

  pub fn entries(&self) -> impl Iterator<Item = (&'_ str, PropValue)> + '_ {
    self
      .entries
      .iter()
      .copied()
      .filter_map(|(key, value)| key.map(|key| (key.name(), value.as_value())))
  }

  #[track_caller]
  pub fn set(&mut self, key: &str, value: PropValue) {
    let name = PropName::for_name_or_panic(key);

    for entry in self.entries.iter_mut() {
      if entry.0 == Some(name) {
        entry.1 = PropValueCompact::for_value_or_panic(value);
        return;
      }
    }

    panic!("key '{key}' not found");
  }

  pub fn insert_if_unset(&mut self, key: &str, value: PropValue) {
    let name = PropName::for_name_or_panic(key);

    for entry in self.entries() {
      if entry.0 == key {
        return;
      }
    }

    for entry in self.entries.iter_mut() {
      if entry.0.is_none() {
        *entry = (Some(name), PropValueCompact::for_value_or_panic(value));
        // FIXME: Insert this key in the right spot, instead of just sorting. This is a
        // somewhat hot path, so probably with optimizing at some point.
        self.entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        return;
      }
    }

    panic!("no more space for key '{key}'");
  }
}

impl PropMapOwned {
  pub const fn empty() -> Self {
    PropMapOwned { entries: [const { (String::new(), PropValueOwned::Bool(false)) }; 8] }
  }

  pub fn entries(&self) -> impl Iterator<Item = (&str, PropValue)> + '_ {
    self
      .entries
      .iter()
      .filter_map(|(key, value)| if *key != "" { Some((&**key, value.as_value())) } else { None })
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
      if entry.0 == "" {
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
  Axis => axis,
  Variant => variant,
}

intern! { PropEnum, prop_enum
  Andesite => andesite,

  Oak => oak,

  X => x,
  Y => y,
  Z => z,
}

impl PropName {
  pub fn for_name_or_panic(name: &str) -> Self {
    match Self::for_name(name) {
      Some(prop) => prop,
      None => panic!("unknown property '{}'", name),
    }
  }
}

impl PropValueCompact {
  pub fn for_value_or_panic(value: PropValue) -> Self {
    match value {
      PropValue::Bool(value) => PropValueCompact(value as u8),
      PropValue::Int(value) => {
        if value < 0 || value > 125 {
          panic!("int value out of range: {}", value);
        }
        PropValueCompact(value as u8 + 2)
      }
      PropValue::Enum(value) => {
        let value =
          PropEnum::for_name(value).unwrap_or_else(|| panic!("unknown enum value: {}", value));
        PropValueCompact(128 + value as u8)
      }
    }
  }

  pub fn as_value(&self) -> PropValue<'static> {
    match self.0 {
      0..=1 => PropValue::Bool(self.0 != 0),
      2..=127 => PropValue::Int(self.0 as i32 - 2),
      128..=255 => PropValue::Enum(PropEnum::ALL[(self.0 - 128) as usize].name()),
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
}
