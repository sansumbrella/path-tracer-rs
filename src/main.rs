extern crate image;

use image::ImageBuffer;

fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;

    let buffer = ImageBuffer::from_fn(nx, ny, |x, y| {
        let r = x as f32 / nx as f32;
        let g = (ny - y) as f32 / ny as f32;
        let b = 0.2;
        let ir: u8 = (255.99 * r) as u8;
        let ig: u8 = (255.99 * g) as u8;
        let ib: u8 = (255.99 * b) as u8;

        image::Rgb([ir, ig, ib])
    });

    buffer.save("image.png")?;

    Ok(())
}
