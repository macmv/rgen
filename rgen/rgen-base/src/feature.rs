//! Constant time features for enabling/disabling things in the world generation.
//!
//! These are mostly for testing, or for new features that are in progress.

/// Removes all chunks from 0-8 on the X axis, which gives a side view of all
/// the ores in a chunk.
pub const DEBUG_ORES: bool = false;

/// Enables village generation.
pub const VILLAGES: bool = false;
