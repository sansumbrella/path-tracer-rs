//! # Geometry
//!
//! Provides traits for defining geometric objects and materials that
//! govern how rays interact when hitting them.

mod camera;
mod hitable;
mod scattering;
mod sphere;
mod world;

pub use self::camera::*;
pub use self::hitable::*;
pub use self::scattering::*;
pub use self::world::*;
pub use self::sphere::*;