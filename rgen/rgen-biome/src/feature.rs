//! Constant time features for enabling/disabling things in the world
//! generation.
//!
//! These are mostly for testing, or for new features that are in progress.

use crate::table::{ClimateType, GeographicType};

/// Removes all chunks from 0-8 on the X axis, which gives a side view of all
/// the ores in a chunk.
pub const DEBUG_ORES: bool = false;

/// Enables village generation.
pub const VILLAGES: bool = false;

/// Overrides the geographic type of the world.
pub const GEOGRAPHIC_TYPE_OVERRIDE: Option<GeographicType> = None;

/// Overrides the climate type of the world.
pub const CLIMATE_TYPE_OVERRIDE: Option<ClimateType> = None;

/// Override the entire world to be a single biome (the blank biome).
/// table.rs line 37 to set new biome
pub const BIOME_OVERRIDE: bool = false;

/// Replaces everything with a superflat world.
pub const SUPERFLAT: bool = false;
