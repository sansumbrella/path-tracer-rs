use super::vector::*;
use rand::distributions::{Distribution, UnitSphereSurface};
use rand::prelude::*;
use std::borrow::Borrow;
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

pub fn rand() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

/// Returns a unit vector with the same direction as the input vector.
pub fn normalize<T>(vector: T) -> Vec3
where
    T: Borrow<Vec3>,
{
    let vector = vector.borrow();
    let &[x, y, z] = &vector.0;
    Vec3::new(x, y, z) / vector.length()
}

/// Calculates the dot product of two vectors.
pub fn dot<T, U>(a: T, b: U) -> f64
where
    T: Borrow<Vec3>,
    U: Borrow<Vec3>,
{
    let &[ax, ay, az] = &a.borrow().0;
    let &[bx, by, bz] = &b.borrow().0;

    ax * bx + ay * by + az * bz
}

/// Calculates the cross product of two vectors.
pub fn cross<T, U>(a: T, b: U) -> Vec3
where
    T: Borrow<Vec3>,
    U: Borrow<Vec3>,
{
    let &[ax, ay, az] = &a.borrow().0;
    let &[bx, by, bz] = &b.borrow().0;

    Vec3::new(ay * bz - az * by, -(ax * bz - az * bx), ax * by - ay * bx)
}

/// Reflect a vector about a normal.
pub fn reflect<T, U>(vector: T, normal: U) -> Vec3
where
    T: Borrow<Vec3>,
    U: Borrow<Vec3>,
{
    let vector = vector.borrow();
    let normal = normal.borrow();
    vector - &(normal * 2.0 * dot(vector, normal))
}

/// Returns a refracted vector according to Fresnel's law.
pub fn refract<T, U>(vector: T, normal: U, ni_over_nt: f64) -> Option<Vec3>
where
    T: Borrow<Vec3>,
    U: Borrow<Vec3>,
{
    let vector = vector.borrow();
    let normal = normal.borrow();
    let vector = normalize(vector);
    let dt = dot(&vector, normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        let refracted = (vector - normal * dt) * ni_over_nt - normal * f64::sqrt(discriminant);
        // let refracted = vector * ni_over_nt + normal * (ni_over_nt * dt - discriminant.sqrt());
        return Some(refracted);
    }

    None
}

/// Schlick polynomial approximation for reflection probability.
pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
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

    #[test]
    fn dot_product() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.5, 0.5, 0.5);

        assert_eq!(
            dot(&a, &b),
            0.5,
            "Dot product returns a scalar measuring similarity of two vectors"
        );

        assert_eq!(
            dot(&Vec3::new(0.0, -1.0, 0.0), &Vec3::new(0.0, 1.0, 0.0)),
            -1.0,
            "Dot product returns a scalar measuring similarity of two vectors"
        );
    }

    #[test]
    fn cross_product() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let z = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(
            cross(&x, &y),
            z,
            "Cross product returns a vector orthogonal to both inputs"
        );

        assert_eq!(
            cross(&y, &x),
            Vec3::new(0.0, 0.0, -1.0),
            "Cross product is not commutative"
        );

        assert_eq!(
            cross(&y, &z),
            x,
            "Cross product returns a vector orthogonal to both inputs"
        );

        assert_eq!(
            cross(&z, &x),
            y,
            "Cross product returns a vector orthogonal to both inputs"
        );
    }

    #[test]
    fn normalize_vectors() {
        let b = Vec3::new(1.0, 1.0, 0.0);

        normalize(b);
        normalize(&b);
    }
}
