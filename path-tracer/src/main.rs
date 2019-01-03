extern crate image;

use geometry::{HitRecord, Hitable, Sphere};
use image::ImageBuffer;
use math::Ray;
use math::Vec3;
use math::{mix, normalize};
use rand::prelude::*;

fn main() -> std::io::Result<()> {
    let nx = 400;
    let ny = 200;
    let ns = 50;

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
    }));

    world.collection().push(Box::new(Sphere {
        center: Vec3::new(-0.3, 0.1, -0.8),
        radius: 0.3,
    }));

    world.collection().push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    let mut rng = rand::thread_rng();

    let buffer = ImageBuffer::from_fn(nx, ny, |x, y| {
        let mut rgb = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let u = (x as f32 + rng.gen::<f32>()) / nx as f32;
            let v = ((ny - y) as f32 + rng.gen::<f32>()) / ny as f32;
            let ray = camera.make_ray(u, v);
            rgb += color(&world, ray);
        }
        rgb /= ns as f32;
        let ir: u8 = (255.99 * rgb.r()) as u8;
        let ig: u8 = (255.99 * rgb.g()) as u8;
        let ib: u8 = (255.99 * rgb.b()) as u8;

        image::Rgb([ir, ig, ib])
    });

    buffer.save("image.png")?;

    Ok(())
}

fn color(world: &World, ray: Ray) -> Vec3 {
    if let Some(hit) = world.hit(&ray, 0.0, 1.0) {
        return (hit.normal + 1.0) * 0.5;
    }

    let unit_direction = normalize(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    mix(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
}

struct World(Vec<Box<Hitable>>);

impl Hitable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
    pub fn make_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
