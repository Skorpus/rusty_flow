extern crate rand;

use self::rand::Rng;

// This value determines if the terrain is smooth or mountainous. 0=smooth 1=mountainous
const ROUGHNESS: f32 = 0.8;
// NOTE Look into using the chunks like the image library
type Buffer = Vec<f32>;

struct EnumeratePixelMap {
    // pixels: Buffer,
    x: u32,
    y: u32,
    size: u32,
}

impl Iterator for EnumeratePixelMap {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<(u32, u32)> {
        if self.x >= self.size {
            self.x = 0;
            self.y += 1;
        }
        let (x, y) = (self.x, self.y);
        self.x += 1;
        if self.y >= self.size {
            None
        } else {
            Some((x, y))
        }
    }
}

/// Structure containing the essential elements to perform the Diamond-Square algorithm
pub struct PixelMap {
    map: Buffer,
    size: u32,
}

impl PixelMap {
    fn max(&self) -> i32 {
        (self.size - 1) as i32
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> f32 {
        let x = x as usize;
        let y = y as usize;
        self.map[x + y * (self.size as usize)]
    }

    fn set_pixel(&mut self, x: u32, y: u32, value: f32) {
        let x = x as usize;
        let y = y as usize;
        self.map[x + y * (self.size as usize)] = value;
    }

    fn size(&self) -> u32 {
        self.size
    }

    fn enumerate_pixels(&self) -> EnumeratePixelMap {
        EnumeratePixelMap {
            x: 0,
            y: 0,
            size: self.size, // pixels: self.pixels(),
        }
    }
}

/// Constructs a Buffer of with dimensions size * size
pub fn construct(size: u32) -> PixelMap {
    let size_vec = size as usize;
    let mut ds = PixelMap {
        map: vec![0.0f32; size_vec * size_vec],
        size: size,
    };
    const INITIAL: f32 = 100.0;
    // Set all corner points to a seed value
    // Then perform steps alternatively until all array values are set
    for x_coord in vec![0, ds.max()] {
        for y_coord in vec![0, ds.max()] {
            set_sample(&mut ds, x_coord, y_coord, INITIAL);
        }
    }
    let initial_size = ds.max();
    divide(&mut ds, initial_size);
    ds
}

fn divide(ds: &mut PixelMap, size: i32) {
    let half = size / 2;
    if half < 1 {
        return;
    }
    square_step(ds, size);
    diamond_step(ds, size);
    divide(ds, size / 2);
}

fn diamond_step(ds: &mut PixelMap, feature_size: i32) {
    let half_step = feature_size / 2;
    // By multiplying by size we ensure that it shrinks with the feature size
    let size = feature_size as f32;
    let scale = ROUGHNESS * size;
    let mut y: i32 = 0;
    while y <= ds.max() {
        let mut x: i32 = (y + half_step) % feature_size;
        while x <= ds.max() {
            diamond_sample(ds,
                           x,
                           y,
                           rand::thread_rng().gen::<f32>() * scale,
                           feature_size);
            x += feature_size;
        }
        y += half_step;
    }
}

fn square_step(ds: &mut PixelMap, feature_size: i32) {
    let half_step = feature_size / 2;
    // By multiplying by size we ensure that it shrinks with the feature size
    let size = feature_size as f32;
    let scale = ROUGHNESS * size;
    let mut y = half_step;
    while y < ds.max() {
        let mut x = half_step;
        while x < ds.max() {
            square_sample(ds,
                          x,
                          y,
                          rand::thread_rng().gen::<f32>() * scale,
                          feature_size);
            x += feature_size;
        }
        y += feature_size;
    }
}

/// Calculate the centre of the diamond using the average, plus a random value
fn diamond_sample(ds: &mut PixelMap, x: i32, y: i32, rand_value: f32, size: i32) {
    //   a
    // b x c
    //   d
    let a = sample(ds, x, (y - size));
    let b = sample(ds, (x - size), y);
    let c = sample(ds, (x + size), y);
    let d = sample(ds, x, (y + size));
    let value = (a + b + c + d) / 4.0 + rand_value;
    set_sample(ds, x, y, value);
}

fn square_sample(ds: &mut PixelMap, x: i32, y: i32, rand_value: f32, size: i32) {
    // a  b
    //  x
    // c  d
    let a = sample(ds, (x - size), (y - size));
    let b = sample(ds, (x + size), (y - size));
    let c = sample(ds, (x - size), (y + size));
    let d = sample(ds, (x + size), (y + size));
    let value = (a + b + c + d) / 4.0 + rand_value;
    set_sample(ds, x, y, value);
}

fn sample(ds: &mut PixelMap, x: i32, y: i32) -> f32 {
    let grid_size = ds.size as i32;
    let x = modulo(x, grid_size) as u32;
    let y = modulo(y, grid_size) as u32;
    ds.get_pixel(x, y)
}

fn modulo(x: i32, modulo: i32) -> i32 {
    ((x % modulo) + modulo) % modulo
}

/// Set the value of the given element in the map
fn set_sample(ds: &mut PixelMap, x: i32, y: i32, value: f32) {
    let x = x as u32;
    let y = y as u32;
    ds.set_pixel(x, y, value)
}


pub fn normalize_pixel_map(p_map: PixelMap) -> PixelMap {
    let size = p_map.size() as usize;
    let mut new_p_map = PixelMap {
        map: vec![0.0f32; size* size],
        size: p_map.size(),
    };
    // TODO Check that we are always setting value to positive
    let mut max_value = 0.0;
    for (x, y) in p_map.enumerate_pixels() {
        let f_value = p_map.get_pixel(x, y);
        if f_value > max_value {
            max_value = f_value;
        }
    }

    for (x, y) in p_map.enumerate_pixels() {
        let mut f_value = p_map.get_pixel(x, y);
        f_value /= max_value;
        f_value *= 255.0;
        new_p_map.set_pixel(x, y, f_value);
    }

    new_p_map
}
