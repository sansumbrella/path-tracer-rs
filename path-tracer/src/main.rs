extern crate image;

use image::ImageBuffer;
use math::Ray;
use math::Vec3;
use math::{cross, dot, mix, normalize};

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let buffer = ImageBuffer::from_fn(nx, ny, |x, y| {
        let u = x as f32 / nx as f32;
        let v = (ny - y) as f32 / ny as f32;

        let ray = Ray::new(
            origin,
            lower_left_corner + (horizontal * u) + (vertical * v),
        );
        let rgb = color(ray);
        let ir: u8 = (255.99 * rgb.r()) as u8;
        let ig: u8 = (255.99 * rgb.g()) as u8;
        let ib: u8 = (255.99 * rgb.b()) as u8;

        image::Rgb([ir, ig, ib])
    });

    buffer.save("image.png")?;

    Ok(())
}

fn color(ray: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let unit_direction = normalize(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    mix(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - &center;
    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(&oc, ray.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
