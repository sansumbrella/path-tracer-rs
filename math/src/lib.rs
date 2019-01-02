mod ray;
mod vector;

pub use self::ray::*;
pub use self::vector::*;

/// TODO: implement as a generic function
/// Two types that implement Add, Subtract, and Multiply by a Scalar
/// Want it generic so we can mix(T, &T, f32) or mix(&T, &T, f32) without
/// manually writing out the variants.
pub fn mix(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a + (b - a) * t
}
