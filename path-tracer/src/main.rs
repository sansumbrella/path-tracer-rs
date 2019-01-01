extern crate image;

use image::ImageBuffer;
use math::Vec3;

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;

    let buffer = ImageBuffer::from_fn(nx, ny, |x, y| {
        let rgb = Vec3::new(x as f32 / nx as f32, (ny - y) as f32 / ny as f32, 0.2);
        let ir: u8 = (255.99 * rgb.r()) as u8;
        let ig: u8 = (255.99 * rgb.g()) as u8;
        let ib: u8 = (255.99 * rgb.b()) as u8;

        image::Rgb([ir, ig, ib])
    });

    buffer.save("image.png")?;

    println!("Size of a Vec3 is {}", std::mem::size_of::<Vec3>());

    Ok(())
}
