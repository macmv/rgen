//! Parts of the generator that can be turned off at runtime, so that tools can
//! peel the terrain back one layer at a time. This only exists with the
//! `inspector` feature. Without it, `enabled!` is `true`, and the checks
//! compile away.

use std::sync::atomic::{AtomicBool, Ordering};

macro_rules! toggles {
  ($($id:ident => $name:literal, $parent:expr;)*) => {
    /// One part of the generator. These are in the order the generator applies
    /// them.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Toggle {
      $($id,)*
    }

    impl Toggle {
      pub const ALL: &[Toggle] = &[$(Toggle::$id,)*];

      pub fn name(self) -> &'static str {
        match self {
          $(Toggle::$id => $name,)*
        }
      }

      /// The toggle that this one is a part of. If the parent is off, so is
      /// this.
      pub fn parent(self) -> Option<Toggle> {
        match self {
          $(Toggle::$id => $parent,)*
        }
      }
    }
  };
}

toggles! {
  Noise3d => "3D density noise (overhangs and floating land)", None;
}

static STATE: [AtomicBool; Toggle::ALL.len()] =
  [const { AtomicBool::new(true) }; Toggle::ALL.len()];

/// Whether a toggle, and everything it is a part of, is on.
pub fn enabled(toggle: Toggle) -> bool {
  let mut toggle = toggle;
  loop {
    if !STATE[toggle as usize].load(Ordering::Relaxed) {
      return false;
    }
    match toggle.parent() {
      Some(parent) => toggle = parent,
      None => return true,
    }
  }
}

/// Whether the toggle itself is on, ignoring the toggles it is a part of.
pub fn is_set(toggle: Toggle) -> bool { STATE[toggle as usize].load(Ordering::Relaxed) }

pub fn set(toggle: Toggle, on: bool) { STATE[toggle as usize].store(on, Ordering::Relaxed); }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn toggles_are_in_order_of_their_parents() {
    for (i, toggle) in Toggle::ALL.iter().enumerate() {
      assert_eq!(*toggle as usize, i);
      if let Some(parent) = toggle.parent() {
        assert!((parent as usize) < i, "{toggle:?} comes before its parent");
      }
    }
  }
}
