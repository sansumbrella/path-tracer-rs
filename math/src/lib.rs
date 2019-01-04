mod ray;
mod vector;
use rand::distributions::{Distribution, UnitSphereSurface};

pub use self::ray::*;
pub use self::vector::*;

/// TODO: implement as a generic function
/// Two types that implement Add, Subtract, and Multiply by a Scalar
/// Want it generic so we can mix(T, &T, f64) or mix(&T, &T, f64) without
/// manually writing out the variants.
pub fn mix(a: Vec3, b: Vec3, t: f64) -> Vec3 {
    a + (b - a) * t
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let sphere = UnitSphereSurface::new();
    Vec3(sphere.sample(&mut rng))
}