extern crate rand;

use self::rand::Rng;

// This value determines if the terrain is smooth or mountainous. 0=smooth 1=mountainous
const ROUGHNESS: f32 = 0.8;
type Buffer = Vec<f32>;

/// Structure containing the essential elements to perform the Diamond-Square algorithm
struct DSAlgorithm {
    map: Buffer,
    size: usize,
    max: i32,
}

/// Constructs a Buffer of with dimensions size * size
pub fn construct(size: usize) -> Buffer {
    let mut ds = DSAlgorithm {
        map: vec![0.0f32; size * size],
        size: size,
        max: (size - 1) as i32,
    };
    const INITIAL: f32 = 100.0;
    // Set all corner points to a seed value
    // Then perform steps alternatively until all array values are set
    for x_coord in vec![0, ds.max] {
        for y_coord in vec![0, ds.max] {
            set_sample(&mut ds, x_coord, y_coord, INITIAL);
        }
    }
    let initial_size = ds.max;
    divide(&mut ds, initial_size);
    ds.map
}

fn divide(ds: &mut DSAlgorithm, size: i32) {
    let half = size / 2;
    if half < 1 {
        return;
    }
    square_step(ds, size);
    diamond_step(ds, size);
    divide(ds, size / 2);
}

fn diamond_step(ds: &mut DSAlgorithm, feature_size: i32) {
    let half_step = feature_size / 2;
    // By multiplying by size we ensure that it shrinks with the feature size
    let size = feature_size as f32;
    let scale = ROUGHNESS * size;
    let mut y: i32 = 0;
    while y <= ds.max {
        let mut x: i32 = (y + half_step) % feature_size;
        while x <= ds.max {
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

fn square_step(ds: &mut DSAlgorithm, feature_size: i32) {
    let half_step = feature_size / 2;
    // By multiplying by size we ensure that it shrinks with the feature size
    let size = feature_size as f32;
    let scale = ROUGHNESS * size;
    let mut y = half_step;
    while y < ds.max {
        let mut x = half_step;
        while x < ds.max {
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
fn diamond_sample(ds: &mut DSAlgorithm, x: i32, y: i32, rand_value: f32, size: i32) {
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

fn square_sample(ds: &mut DSAlgorithm, x: i32, y: i32, rand_value: f32, size: i32) {
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

fn sample(ds: &mut DSAlgorithm, x: i32, y: i32) -> f32 {
    let grid_size = ds.size as i32;
    let x = modulo(x, grid_size) as usize;
    let y = modulo(y, grid_size) as usize;
    ds.map[x + y * ds.size]
}

fn modulo(x: i32, modulo: i32) -> i32 {
    ((x % modulo) + modulo) % modulo
}

/// Set the value of the given element in the map
fn set_sample(ds: &mut DSAlgorithm, x: i32, y: i32, value: f32) {
    let x = x as usize;
    let y = y as usize;
    ds.map[x + y * ds.size] = value;
}
