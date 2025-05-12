use image::{ImageBuffer, RgbImage};
use rayon::prelude::*;

pub struct PixelCoordinates {
    pub x: f32,
    pub y: f32
}

pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

pub fn render_pixels<F>(function: F, width: u32, height: u32) -> RgbImage
where
    F: Sync + Send + Fn(PixelCoordinates) -> Colour,
{
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let buffer = img.as_mut();

    buffer
        .par_chunks_mut(3)
        .enumerate()
        .for_each(|(i, pixel)| {
            let px = (i as u32) % width;
            let py = (i as u32) / width;

            let x = (px as f32 / (width - 1) as f32) * 2.0 - 1.0;
            let y = (py as f32 / (height - 1) as f32) * 2.0 - 1.0;

            let Colour { r, g, b } = function(PixelCoordinates { x, y });

            pixel[0] = ((r + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
            pixel[1] = ((g + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
            pixel[2] = ((b + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
        });

    img
}

pub fn fnv1a(input: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325; 
    let prime: u64 = 0x100000001b3;

    for byte in input.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(prime); 
    }

    hash
}

pub struct LinearCongruentialGenerator {
    state: u64, 
    a: u64,    
    c: u64,   
    m: u64,    
}

impl LinearCongruentialGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed,
            a: 1664525,
            c: 1013904223,
            m: 2_u64.pow(32), 
        }
    }

    pub fn next(&mut self) -> u64 {
        self.state = (self.a.wrapping_mul(self.state).wrapping_add(self.c)) % self.m;
        self.state
    }

    pub fn next_float(&mut self) -> f32 {
        (self.next() as f32) / (self.m as f32)
    }

    pub fn next_range(&mut self, min: u64, max: u64) -> u64 {
        min + (self.next() % (max - min))
    }
}