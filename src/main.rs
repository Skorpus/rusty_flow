extern crate rusty_flow;
extern crate image;

use std::path::Path;
use std::fs::File;

const DETAIL: u32 = 10;
fn main() {
    let buffer = rusty_flow::diamond_square::construct(DETAIL);
    let buffer = rusty_flow::diamond_square::normalize_pixel_map(buffer);
    let image_size: u32 = buffer.size();

    let mut img_buf = image::ImageBuffer::new(image_size as u32, image_size as u32);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let value: u8 = buffer.get_pixel(x, y);
        *pixel = image::Luma([value]);
    }

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageLuma8(img_buf).save(fout, image::PNG);
}
