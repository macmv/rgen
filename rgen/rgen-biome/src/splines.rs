use rgen_spline::Spline;

use crate::{
  CONTINENTALNESS, EROSION, HEIGHT_IMPACT, PEAKS_VALLEYS,
  terrain::{HeightParts, RawNoise, height_parts},
};

pub type Keys = Spline<Vec<(f64, f64)>>;

/// The splines of the height function.
///
/// Normally the generator reads the `CONTINENTALNESS`, `EROSION`,
/// `PEAKS_VALLEYS` and `HEIGHT_IMPACT` statics. With the `inspector`
/// feature it reads the set given to `set_splines` instead, so that tools can
/// change them while the generator is running.
#[derive(Debug, Clone)]
pub struct Splines {
  pub continentalness: Keys,
  pub erosion:         Keys,
  pub peaks_valleys:   Keys,
  pub height_impact:   Keys,
}

impl Splines {
  /// The splines that the game ships with.
  pub fn game() -> Self {
    Splines {
      continentalness: Spline::from_vec(CONTINENTALNESS.storage.to_vec()),
      erosion:         Spline::from_vec(EROSION.storage.to_vec()),
      peaks_valleys:   Spline::from_vec(PEAKS_VALLEYS.storage.to_vec()),
      height_impact:   Spline::from_vec(HEIGHT_IMPACT.storage.to_vec()),
    }
  }

  pub fn height_parts(&self, noise: RawNoise) -> HeightParts {
    height_parts(
      &self.continentalness,
      &self.erosion,
      &self.peaks_valleys,
      &self.height_impact,
      noise,
    )
  }
}

/// The height function, with the splines that the generator is using.
pub(crate) fn current_height_parts(noise: RawNoise) -> HeightParts {
  #[cfg(feature = "inspector")]
  {
    dynamic::CURRENT.read().unwrap_or_else(|e| e.into_inner()).height_parts(noise)
  }
  #[cfg(not(feature = "inspector"))]
  {
    height_parts(&*CONTINENTALNESS, &*EROSION, &*PEAKS_VALLEYS, &*HEIGHT_IMPACT, noise)
  }
}

#[cfg(feature = "inspector")]
mod dynamic {
  use std::sync::{LazyLock, RwLock};

  use super::Splines;

  pub static CURRENT: LazyLock<RwLock<Splines>> = LazyLock::new(|| RwLock::new(Splines::game()));
}

/// Replaces the splines that every `WorldBiomes` samples height with. Anything
/// that was already generated needs to be generated again.
#[cfg(feature = "inspector")]
pub fn set_splines(splines: Splines) {
  *dynamic::CURRENT.write().unwrap_or_else(|e| e.into_inner()) = splines;
}
