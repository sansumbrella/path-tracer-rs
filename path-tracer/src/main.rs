extern crate image;

use geometry::{Camera, Dielectric, Hitable, Lambertian, Metallic, Sphere, World};
use image::ImageBuffer;
use math::{mix, normalize, Ray, Vec3};
use rand::prelude::*;

fn main() -> std::io::Result<()> {
    let nx = 300;
    let ny = 150;
    let ns = 100;

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        nx as f64 / ny as f64,
    );

    println!("Camera settings: {:?}", camera);

    let mut world = World::new();
    let sphere_z = -1.5;
    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, sphere_z),
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.1, 0.2, 0.5),
        }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3::new(-1.0, -0.0, sphere_z),
        radius: 0.5,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, sphere_z),
        radius: 0.5,
        material: Box::new(Metallic {
            albedo: Vec3::new(0.8, 0.6, 0.2),
            roughness: 0.0,
        }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, sphere_z),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
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
