extern crate rand;
extern crate num;

use std::f32;

use self::rand::Rng;
use self::num::Num;
use self::num::Integer;

// This value determines if the terrain is smooth or mountainous. 0=smooth 1=mountainous
const ROUGHNESS: f32 = 0.75;
// NOTE Look into using the chunks like the image library
type Buffer<T> = Vec<T>;

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
pub struct PixelMap<T: Copy + Num> {
    map: Buffer<T>,
    size: u32,
}

impl<T: Copy + Num> PixelMap<T> {
    fn max(&self) -> i32 {
        (self.size - 1) as i32
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> T {
        let x = x as usize;
        let y = y as usize;
        self.map[x + y * (self.size as usize)]
    }

    fn set_pixel(&mut self, x: u32, y: u32, value: T) {
        let x = x as usize;
        let y = y as usize;
        self.map[x + y * (self.size as usize)] = value;
    }

    pub fn size(&self) -> u32 {
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
pub fn construct(detail: u32) -> PixelMap<f32> {
    let size: u32 = 2u32.pow(detail) + 1;
    let size_vec = size as usize;
    let mut p_map = PixelMap {
        map: vec![0.0f32; size_vec * size_vec],
        size: size,
    };
    // NOTE It appears that with this algorithm this is the largest value we get.
    // Also when the image becomes larger we seem to be skipping certain positions.
    const INITIAL: f32 = 1000.0;
    // Set all corner points to a seed value
    // Then perform steps alternatively until all array values are set
    for x_coord in vec![0, p_map.max()] {
        for y_coord in vec![0, p_map.max()] {
            set_sample(&mut p_map, x_coord, y_coord, INITIAL);
        }
    }
    let initial_size = p_map.max();
    divide(&mut p_map, initial_size);
    p_map
}

fn divide(p_map: &mut PixelMap<f32>, size: i32) {
    let half = size / 2;
    if half < 1 {
        return;
    }
    square_step(p_map, size);
    diamond_step(p_map, size);
    divide(p_map, size / 2);
}

fn diamond_step(p_map: &mut PixelMap<f32>, feature_size: i32) {
    let half_step = feature_size / 2;
    // By multiplying by size we ensure that it shrinks with the feature size
    let size = feature_size as f32;
    let scale = ROUGHNESS * size;
    let mut y: i32 = 0;
    while y <= p_map.max() {
        let mut x: i32 = (y + half_step) % feature_size;
        while x <= p_map.max() {
            diamond_sample(p_map,
                           x,
                           y,
                           rand::thread_rng().next_f32() * scale * 2.0 - scale,
                           half_step);
            x += feature_size;
        }
        y += half_step;
    }
}

fn square_step(p_map: &mut PixelMap<f32>, feature_size: i32) {
    let half_step = feature_size / 2;
    // By multiplying by size we ensure that it shrinks with the feature size
    let size = feature_size as f32;
    let scale = ROUGHNESS * size;
    let mut y = half_step;
    while y < p_map.max() {
        let mut x = half_step;
        while x < p_map.max() {
            square_sample(p_map,
                          x,
                          y,
                          rand::thread_rng().next_f32() * scale * 2.0 - scale,
                          half_step);
            x += feature_size;
        }
        y += feature_size;
    }
}

/// Calculate the centre of the diamond using the average, plus a random value
fn diamond_sample(p_map: &mut PixelMap<f32>, x: i32, y: i32, rand_value: f32, half_size: i32) {
    //   a
    // b x c
    //   d
    let a = sample(p_map, x, (y - half_size));
    let b = sample(p_map, (x - half_size), y);
    let c = sample(p_map, (x + half_size), y);
    let d = sample(p_map, x, (y + half_size));
    let value = (a + b + c + d) / 4.0 + rand_value;
    set_sample(p_map, x, y, value);
}

#[test]
fn diamond_sample_test() {
    // This test will also do the wrapping for us
    //  0  0 10
    // 10 12  X
    //  0  0 10
    let size = 3i32;
    let mut test_sample = PixelMap {
        map: vec![0.0f32, 0.0f32, 10.0f32, 10.0f32, 12.0f32, 0.0f32, 0.0f32, 0.0f32, 10.0f32],
        size: size as u32,
    };
    let (x, y): (i32, i32) = (2, 1);
    let rand_value = 5.0f32;

    diamond_sample(&mut test_sample, x, y, rand_value, size / 2);

    assert_eq!(15.5, test_sample.get_pixel(x as u32, y as u32));

}

fn square_sample(p_map: &mut PixelMap<f32>, x: i32, y: i32, rand_value: f32, half_size: i32) {
    // a  b
    //  x
    // c  d
    let a = sample(p_map, (x - half_size), (y - half_size));
    let b = sample(p_map, (x + half_size), (y - half_size));
    let c = sample(p_map, (x - half_size), (y + half_size));
    let d = sample(p_map, (x + half_size), (y + half_size));
    let value = (a + b + c + d) / 4.0 + rand_value;
    set_sample(p_map, x, y, value);
}

#[test]
fn square_sample_test() {
    // This test will also do the wrapping for us
    // 10 12 0
    // 0  0  X
    // 10 10 0
    let size = 3i32;
    let mut test_sample = PixelMap {
        map: vec![10.0f32, 12.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 10.0f32, 10.0f32, 0.0f32],
        size: size as u32,
    };
    let (x, y): (i32, i32) = (2, 1);
    let rand_value = 5.0f32;

    square_sample(&mut test_sample, x, y, rand_value, size / 2);

    assert_eq!(15.5, test_sample.get_pixel(x as u32, y as u32));

}

fn sample(p_map: &mut PixelMap<f32>, x: i32, y: i32) -> f32 {
    let grid_size = p_map.size as i32;
    let x = x.mod_floor(&grid_size) as u32;
    let y = y.mod_floor(&grid_size) as u32;
    p_map.get_pixel(x, y)
}

/// Set the value of the given element in the map
fn set_sample(p_map: &mut PixelMap<f32>, x: i32, y: i32, value: f32) {
    let x = x as u32;
    let y = y as u32;
    p_map.set_pixel(x, y, value)
}

#[test]
fn set_sample_test() {
    let mut test_sample = PixelMap {
        map: vec![1.0, 2.0, 3.0, 4.0],
        size: 2,
    };
    let (x, y) = (0, 0);
    let expected = 5.0;
    set_sample(&mut test_sample, x, y, expected);
    assert_eq!(expected, test_sample.get_pixel(x as u32, y as u32));
}


pub fn normalize_pixel_map(p_map: PixelMap<f32>) -> PixelMap<u8> {
    let size = p_map.size() as usize;
    let mut new_p_map = PixelMap {
        map: vec![0u8; size* size],
        size: p_map.size(),
    };
    let max_value = max(&p_map);
    let min_value = min(&p_map);


    for (x, y) in p_map.enumerate_pixels() {
        let mut f_value = p_map.get_pixel(x, y);
        // Need to zero align all the values
        f_value -= min_value;
        f_value /= max_value - min_value;
        f_value *= u8::max_value() as f32;
        new_p_map.set_pixel(x, y, f_value as u8);
    }

    new_p_map
}

fn min(p_map: &PixelMap<f32>) -> f32 {
    let mut min_value = f32::MAX;
    for (x, y) in p_map.enumerate_pixels() {
        let f_value = p_map.get_pixel(x, y);
        if f_value < min_value {
            min_value = f_value;
        }
    }
    min_value
}

fn max(p_map: &PixelMap<f32>) -> f32 {
    let mut max_value = -f32::MAX;
    for (x, y) in p_map.enumerate_pixels() {
        let f_value = p_map.get_pixel(x, y);
        if f_value > max_value {
            max_value = f_value;
        }
    }
    max_value
}

#[test]
fn normalize_pixel_map_test() {
    let test_sample = PixelMap {
        map: vec![1.0, 2.0, 3.0, 4.0],
        size: 2,
    };
    let result = normalize_pixel_map(test_sample);
    assertions(result);

    let test_sample = PixelMap {
        map: vec![-1.0, 2.0, 3.0, 4.0],
        size: 2,
    };
    let result = normalize_pixel_map(test_sample);
    assertions(result);

    fn assertions(result: PixelMap<u8>) {
        let mut has_zero = false;
        let mut has_max = false;
        for elem in result.map {
            assert!(elem <= 255, "Element that failed was: {}", elem);
            if elem == 0 {
                has_zero = true;
            }
            if elem == 255 {
                has_max = true;
            }
        }
        assert!(has_zero);
        assert!(has_max);
    }
}
