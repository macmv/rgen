//! Icing placers: the last layer of decoration, placed once a chunk is final.
//!
//! See `IcingPlacer` for what that means.

mod bamboo;

pub use bamboo::{Bamboo, BambooClump};

mod clump;
mod scatter;
mod spread;
pub use clump::{Clumps, DriedGrassClumps, GrassClumps};
pub use scatter::{DoubleScatter, Scatter};
pub use spread::Spread;

#[cfg(test)]
mod tests;

mod reeds;
pub use reeds::Reeds;
