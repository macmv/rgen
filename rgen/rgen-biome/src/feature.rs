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
//pub const GEOGRAPHIC_TYPE_OVERRIDE: Option<GeographicType> =
// Some(GeographicType::Standard);

/// Overrides the climate type of the world.
pub const CLIMATE_TYPE_OVERRIDE: Option<ClimateType> = None;

/// Test the two completed climates in broad humidity regions. A single-climate
/// override above takes priority. Disable this to restore the full climate table.
pub const MEDITERRANEAN_BADLANDS_MIX: bool = true;

/// Override the entire world to be a single biome (the blank biome).
/// In table.rs To change the biome, modify the biome_categories macro, and
/// change the blank biome composition
pub const BIOME_OVERRIDE: bool = false;

/// Replaces everything with a superflat world.
pub const SUPERFLAT: bool = false;

/// Optional named biome for testing, without replacing the normal climate mixes.
/// Set to None to restore the table's weighted biome selection.
pub const NAMED_BIOME_OVERRIDE: Option<(&str, crate::biome::BiomeFn)> = None;

/// Geography-specific variants of a named test override.
/// Entries apply only when their parent biome name matches NAMED_BIOME_OVERRIDE.
pub const NAMED_BIOME_VARIANTS: &[(&str, GeographicType, &str, crate::biome::BiomeFn)] = &[
  ("arid_woodland", GeographicType::River, "arid_woodland_river", crate::biome::arid_woodland_river),
  ("arid_woodland", GeographicType::Hills, "painted_mesa", crate::biome::painted_mesa),
  ("arid_woodland", GeographicType::Mountains, "painted_mesa", crate::biome::painted_mesa),
  ("arid_woodland", GeographicType::Beach, "red_rock_shore", crate::biome::red_rock_shore),
  ("arid_woodland", GeographicType::Delta, "red_reed_marsh", crate::biome::red_reed_marsh),
  ("red_scrubland", GeographicType::River, "red_scrubland_river", crate::biome::red_scrubland_river),
  ("red_scrubland", GeographicType::Hills, "painted_mesa", crate::biome::painted_mesa),
  ("red_scrubland", GeographicType::Mountains, "painted_mesa", crate::biome::painted_mesa),
  ("red_scrubland", GeographicType::Beach, "red_rock_shore", crate::biome::red_rock_shore),
  ("red_scrubland", GeographicType::Delta, "red_reed_marsh", crate::biome::red_reed_marsh),
  ("painted_mesa", GeographicType::River, "painted_mesa_river", crate::biome::painted_mesa_river),
  ("painted_mesa", GeographicType::Beach, "painted_mesa_beach", crate::biome::painted_mesa_beach),
  ("painted_mesa", GeographicType::Delta, "painted_mesa_delta", crate::biome::painted_mesa_delta),
];

/// Experimental bounded drainage graph, replacing noise-band rivers.
pub const WATERSHEDS: bool = true;
