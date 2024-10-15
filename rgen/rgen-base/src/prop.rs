use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PropMap {
  // Garuntee: There cannot be more than 8 properties on a block.
  entries: [(&'static str, PropValue); 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropValue {
  Bool(bool),
  Int(i32),
  Enum(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropType {
  Bool,
  Int(i32, i32),
  Enum(Vec<String>),
}

impl From<bool> for PropValue {
  fn from(value: bool) -> Self { PropValue::Bool(value) }
}
impl From<i32> for PropValue {
  fn from(value: i32) -> Self { PropValue::Int(value) }
}
impl From<&'static str> for PropValue {
  fn from(value: &'static str) -> Self { PropValue::Enum(value) }
}

impl fmt::Debug for PropMap {
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

impl fmt::Display for PropValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PropValue::Bool(value) => write!(f, "{}", value),
      PropValue::Int(value) => write!(f, "{}", value),
      PropValue::Enum(value) => write!(f, "{}", value),
    }
  }
}

impl PropMap {
  pub const fn empty() -> Self { PropMap { entries: [("", PropValue::Bool(false)); 8] } }
  #[track_caller]
  pub fn new(values: &[(&'static str, PropValue)]) -> Self {
    if values.len() > 8 {
      panic!("too many properties");
    }

    let mut entries = [("", PropValue::Bool(false)); 8];
    for (i, (key, value)) in values.iter().enumerate() {
      entries[i] = (*key, *value);
    }

    Self { entries }
  }

  pub fn len(&self) -> usize { self.entries().count() }
  pub fn is_empty(&self) -> bool { self.len() == 0 }

  pub fn entries(&self) -> impl Iterator<Item = (&'static str, PropValue)> + '_ {
    self.entries.iter().copied().filter(|(key, _)| *key != "")
  }

  #[track_caller]
  pub fn set(&mut self, key: &'static str, value: PropValue) {
    for entry in self.entries.iter_mut() {
      if entry.0 == key {
        *entry = (key, value);
        return;
      }
    }

    panic!("key '{key}' not found");
  }
}
