extern crate image;

use geometry::{Camera, Dielectric, Hitable, Lambertian, Metallic, Sphere, World};
use image::ImageBuffer;
use math::{mix, normalize, Ray, Vec3};
use rand::prelude::*;

fn main() -> std::io::Result<()> {
    let nx = 900;
    let ny = 600;
    let ns = 100;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
    );

    println!("Camera settings: {:?}", camera);

    let world = build_book_scene();
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

fn build_book_scene() -> World {
    let mut world = World::new();
    let mut rng = rand::thread_rng();
    let mut rand = || rng.gen::<f64>();

    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Vec3::fill(0.5),
        }),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let material_choice: f64 = rand();
            let center = Vec3::new(a + 0.9 * rand(), 0.2, b + 0.9 * rand());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if material_choice < 0.8 {
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            albedo: Vec3::new(rand() * rand(), rand() * rand(), rand() * rand()),
                        }),
                    }))
                } else if material_choice < 0.95 {
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metallic {
                            albedo: Vec3::new(
                                0.5 * rand() * rand(),
                                0.5 * rand() * rand(),
                                0.5 * rand() * rand(),
                            ),
                            roughness: 0.5 * rand(),
                        }),
                    }));
                } else {
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric {
                            refractive_index: 1.5,
                        }),
                    }));
                }
            }
        }
    }

    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric {
            refractive_index: 1.5,
        }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }),
    }));

    world.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metallic {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            roughness: 0.0,
        }),
    }));

    world
}
