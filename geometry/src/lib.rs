//! # Geometry
//!
//! Provides traits for defining geometric objects and materials that
//! govern how rays interact when hitting them.

mod hitable;
mod scattering;

pub use self::hitable::*;
pub use self::scattering::*;
