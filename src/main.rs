extern crate rusty_flow;
extern crate image;

use std::path::Path;
use std::fs::File;

const IMAGE_SIZE: u32 = 51;
fn main() {
    let buffer = rusty_flow::diamond_square::construct(IMAGE_SIZE);
    let buffer = rusty_flow::diamond_square::normalize_pixel_map(buffer);

    let mut img_buf = image::ImageBuffer::new(IMAGE_SIZE as u32, IMAGE_SIZE as u32);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let value: u8 = buffer.get_pixel(x, y);
        *pixel = image::Luma([value]);
    }

    // let mut i = 0;
    // for elem in &buffer {
    //     print!("{}", elem);
    //     i += 1;
    //     if i == IMAGE_SIZE {
    //         println!("", );
    //         i = 0;
    //     }
    // }

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageLuma8(img_buf).save(fout, image::PNG);
}
