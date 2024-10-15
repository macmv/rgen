use std::fmt;

#[derive(Clone, Copy, Eq)]
pub struct PropMap<'a> {
  // Garuntee: There cannot be more than 8 properties on a block.
  //
  // Also, this will always be sorted by key.
  entries: [(&'a str, PropValue<'a>); 8],
}

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

impl fmt::Debug for PropMap<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_map().entries(self.entries()).finish()
  }
}

impl fmt::Debug for PropMapOwned {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_map().entries(self.entries()).finish()
  }
}

impl fmt::Display for PropMap<'_> {
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

impl<'a> PropMap<'a> {
  pub const fn empty() -> Self { PropMap { entries: [("", PropValue::Bool(false)); 8] } }
  #[track_caller]
  pub fn new(values: &[(&'a str, PropValue<'a>)]) -> Self {
    if values.len() > 8 {
      panic!("too many properties");
    }

    let mut entries = [("", PropValue::Bool(false)); 8];
    for (i, (key, value)) in values.iter().enumerate() {
      entries[i] = (*key, *value);
    }

    entries.sort_unstable_by(|a, b| a.0.cmp(b.0));

    Self { entries }
  }

  pub fn len(&self) -> usize { self.entries().count() }
  pub fn is_empty(&self) -> bool { self.len() == 0 }

  pub fn entries(&self) -> impl Iterator<Item = (&'a str, PropValue)> + '_ {
    self.entries.iter().copied().filter(|(key, _)| *key != "")
  }

  #[track_caller]
  pub fn set(&mut self, key: &'static str, value: PropValue<'a>) {
    for entry in self.entries.iter_mut() {
      if entry.0 == key {
        *entry = (key, value);
        return;
      }
    }

    panic!("key '{key}' not found");
  }

  pub fn insert_if_unset(&mut self, key: &'a str, value: PropValue<'a>) {
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
        self.entries.sort_unstable_by(|a, b| a.0.cmp(b.0));
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

impl PartialEq<PropMap<'_>> for PropMapOwned {
  fn eq(&self, other: &PropMap) -> bool { self.entries().eq(other.entries()) }
}
impl PartialEq<PropMapOwned> for PropMap<'_> {
  fn eq(&self, other: &PropMapOwned) -> bool { self.entries().eq(other.entries()) }
}

impl PartialEq<PropMap<'_>> for PropMap<'_> {
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
