extern crate image;

use geometry::{HitRecord, Hitable, Lambertian, Metallic, Sphere};
use image::ImageBuffer;
use math::Ray;
use math::Vec3;
use math::{mix, normalize};
use rand::prelude::*;
use std::rc::Rc;

fn main() -> std::io::Result<()> {
    let nx = 300;
    let ny = 150;
    let ns = 100;

    let camera = Camera {
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0),
    };

    let mut world = World(vec![]);
    world.collection().push(Box::new(Sphere {
        center: Vec3::new(0.2, 0.0, -1.0),
        radius: 0.6,
        material: Rc::new(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
    }));

    world.collection().push(Box::new(Sphere {
        center: Vec3::new(-0.3, 0.1, -0.8),
        radius: 0.3,
        material: Rc::new(Metallic {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
    }));

    world.collection().push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(Metallic {
            albedo: Vec3::new(0.9, 0.9, 0.1),
        }),
    }));

    let mut rng = rand::thread_rng();

    let buffer = ImageBuffer::from_fn(nx, ny, |x, y| {
        let mut rgb = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let u = (x as f64 + rng.gen::<f64>()) / nx as f64;
            let v = ((ny - y) as f64 + rng.gen::<f64>()) / ny as f64;
            let ray = camera.make_ray(u, v);
            rgb += color(&world, ray, 0);
        }
        rgb /= ns as f64;
        let rgb = Vec3::new(rgb.r().sqrt(), rgb.g().sqrt(), rgb.b().sqrt());
        let ir: u8 = (255.99 * rgb.r()) as u8;
        let ig: u8 = (255.99 * rgb.g()) as u8;
        let ib: u8 = (255.99 * rgb.b()) as u8;

        image::Rgb([ir, ig, ib])
    });

    buffer.save("image.png")?;

    Ok(())
}

fn color(world: &World, ray: Ray, depth: u8) -> Vec3 {
    if let Some(hit) = world.hit(&ray, 0.001, 1.0) {
        // recurse until you bounce off into the sky
        if depth < 50 {
            if let Some(reflection) = hit.material.scatter(&ray, &hit) {
                return color(world, reflection.ray, depth + 1) * reflection.attenuation;
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }
        // return (hit.normal + 1.0) * 0.5;
    }

    let unit_direction = normalize(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    mix(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
}

struct World(Vec<Box<Hitable>>);

impl Hitable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut found: Option<HitRecord> = None;
        for hitable in &self.0 {
            if let Some(hit) = hitable.hit(ray, t_min, closest) {
                closest = hit.t;
                found = Some(hit);
            }
        }
        found
    }
}

impl World {
    pub fn collection(&mut self) -> &mut Vec<Box<Hitable>> {
        &mut self.0
    }
}

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn make_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
