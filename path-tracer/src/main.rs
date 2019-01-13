extern crate image;

use image::RgbImage;
use path_tracing::{trace_scene, Camera, Dielectric, Lambertian, Metallic, Sphere, Vec3, World};
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
    let rendered = trace_scene(&world, &camera, ny, nx, ns);

    if let Some(image) = RgbImage::from_vec(
        nx,
        ny,
        rendered
            .iter()
            .flat_map(|v| v.0.iter())
            .map(|c| (c * 255.99) as u8)
            .collect(),
    ) {
        image.save("mapped-image.png")?;
    }

    Ok(())
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
