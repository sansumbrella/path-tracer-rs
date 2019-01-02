mod ray;
mod vector;

pub use self::ray::*;
pub use self::vector::*;

pub fn mix(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a + (b - a) * t
}
