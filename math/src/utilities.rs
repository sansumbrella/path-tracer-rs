use super::Vec3;
use rand::distributions::{Distribution, UnitSphereSurface};
use std::ops::{Add, Mul, Sub};

/// mix performs a linear interpolation between two like values
pub fn mix<T, U>(a: T, b: T, t: U) -> T
where
    T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<U, Output = T>,
{
    a + (b - a) * t
}

/// returns a random point on the surface of a unit sphere
pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let sphere = UnitSphereSurface::new();
    Vec3(sphere.sample(&mut rng))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn mixing_built_in_types() {
        assert_eq!(mix(1.0, 2.0, 0.5), 1.5);
        assert_eq!(mix(1.0, -1.0, 0.5), 0.0);
        // Rust doesn't support operations between floats and integers,
        // so we need to cast to the output type at least
        assert_eq!(mix(10 as f32, 20 as f32, 0.5) as i32, 15);
    }

    #[test]
    fn mixing_vector_types() {
        assert_eq!(
            mix(Vec3::new(0.0, 1.0, 2.0), Vec3::new(2.0, 1.0, 0.0), 0.5),
            Vec3::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn sphere_random() {
        assert_relative_eq!(random_in_unit_sphere().length_squared(), 1.0);
    }

    #[test]
    fn trigonometry_functions() {
        assert_relative_eq!(f64::sin(std::f64::consts::PI), 0.0);
    }
}
