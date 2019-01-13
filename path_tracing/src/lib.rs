//! # Ray Tracing
//!
//! Provides traits for defining geometric objects and materials that
//! govern how rays interact when hitting them.
//!
//! Vector math routines and convenience functions for graphics.
//!

mod camera;
mod hitable;
mod ray;
mod scattering;
mod sphere;
mod utilities;
mod vector;
mod world;

pub use self::camera::*;
pub use self::hitable::*;
pub use self::ray::*;
pub use self::scattering::*;
pub use self::sphere::*;
pub use self::utilities::*;
pub use self::vector::*;
pub use self::world::*;
use rand::prelude::*;
use rayon::prelude::*;

fn color(world: &World, ray: Ray, depth: u8) -> Vec3 {
    if let Some(hit) = world.hit(&ray, 0.001, std::f64::MAX) {
        // return (hit.normal + 1.0) * 0.5;
        // recurse until you bounce off into the sky
        if depth < 50 {
            if let Some(reflection) = hit.material.scatter(&ray, &hit) {
                // return (*reflection.ray.direction() + 1.0) * 0.5;
                return color(world, reflection.ray, depth + 1) * reflection.attenuation;
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }
    }

    let unit_direction = normalize(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    mix(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
}

pub fn trace_scene(
    world: &World,
    camera: &Camera,
    rows: u32,
    columns: u32,
    num_samples: u32,
) -> Vec<Vec3> {
    let colors: Vec<Vec3> = (0..(rows * columns))
        .into_par_iter()
        .map(|index| {
            let x = index % columns;
            let y = index / columns;
            (x as i32, y as i32)
        })
        .map(|(x, y)| {
            let mut rng = rand::thread_rng();
            (0..num_samples)
                .map(|_| (x as f64 + rng.gen::<f64>(), y as f64 + rng.gen::<f64>()))
                .map(|(x, y)| (x / columns as f64, 1.0 - y / rows as f64))
                .collect::<Vec<(f64, f64)>>()
        }) // generate N normalized samples per pixel coordinate
        .map(|samples| {
            samples
                .iter()
                .map(|(u, v)| {
                    let ray = camera.make_ray(*u, *v);
                    return color(world, ray, 0);
                })
                .fold(Vec3::fill(0.0), |acc, x| acc + x)
                / num_samples as f64
        }) // accumulate colors for pixels
        .map(|linear| Vec3::new(linear.r().sqrt(), linear.g().sqrt(), linear.b().sqrt())) // gamma adjust
        .collect();

    colors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_a_collection() {
        let columns = 4;
        let rows = 2;
    }
}
