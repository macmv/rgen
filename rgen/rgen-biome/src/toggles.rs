//! Parts of the generator that can be turned off at runtime, so that tools can
//! peel the terrain back one layer at a time. This only exists with the
//! `inspector` feature. Without it, `enabled!` is `true`, and the checks
//! compile away.

use std::sync::{
  RwLock,
  atomic::{AtomicBool, Ordering},
};

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
  Caves => "Caves", None;
  NoodleCaves => "Noodle caves (winding tunnels)", Some(Toggle::Caves);
  NoodleWaterGuard => "Stop at water", Some(Toggle::NoodleCaves);
  CheeseCaves => "Cheese caves (large caverns)", Some(Toggle::Caves);
  CheeseSurfaceTaper => "Narrow entrances near the surface", Some(Toggle::CheeseCaves);
  CheeseBedrockTaper => "Shrink towards bedrock", Some(Toggle::CheeseCaves);
  CheeseRivers => "Rivers shrink caves", Some(Toggle::CheeseCaves);
  CheeseWaterGuard => "Don't carve next to water", Some(Toggle::CheeseCaves);
  Biomes => "Biome variety (off: one biome everywhere)", None;
  SurfaceLayers => "Surface layers (grass, dirt, sand...)", None;
  BiomeChunkPlacers => "Biome patches (sand, gravel, clay...)", None;
  Ores => "Ores", None;
  Decorations => "Decorations (trees, plants, rocks...)", None;
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

/// The decorations that are turned off. These are the names that biomes give
/// their placers, so a decoration is off in every biome that has it.
static OFF_PLACERS: RwLock<Vec<&'static str>> = RwLock::new(Vec::new());

/// Whether a decoration is on. This doesn't check `Toggle::Decorations`.
pub fn placer_enabled(name: &str) -> bool {
  !OFF_PLACERS.read().unwrap_or_else(|e| e.into_inner()).contains(&name)
}

pub fn set_placer(name: &'static str, on: bool) {
  let mut off = OFF_PLACERS.write().unwrap_or_else(|e| e.into_inner());
  off.retain(|n| *n != name);
  if !on {
    off.push(name);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn placers_can_be_turned_off_and_on() {
    assert!(placer_enabled("a test placer"));
    set_placer("a test placer", false);
    assert!(!placer_enabled("a test placer"));
    set_placer("a test placer", true);
    assert!(placer_enabled("a test placer"));
  }

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
