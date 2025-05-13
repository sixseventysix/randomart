use image::{RgbImage, ImageBuffer};
use rayon::prelude::*;
use crate::closure_tree::ClosureTree;

pub struct PixelCoordinates {
    pub x: f32,
    pub y: f32,
    pub xi: u32,
    pub yi: u32,
}

pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

pub fn render_pixels(tree: &ClosureTree, width: u32, height: u32) -> RgbImage {
    let width_usize = width as usize;
    let height_usize = height as usize;

    tree.r.populate_x(width_usize);
    tree.g.populate_x(width_usize);
    tree.b.populate_x(width_usize);
    tree.r.populate_y(height_usize);
    tree.g.populate_y(height_usize);
    tree.b.populate_y(height_usize);

    let buffer: Vec<u8> = (0..height_usize * width_usize)
        .into_par_iter()
        .flat_map_iter(|i| {
            let x_index = i % width_usize;
            let y_index = i / width_usize;

            let x = (x_index as f32 / (width - 1) as f32) * 2.0 - 1.0;
            let y = (y_index as f32 / (height - 1) as f32) * 2.0 - 1.0;

            let r = tree.r.eval(x, y, x_index, y_index);
            let g = tree.g.eval(x, y, x_index, y_index);
            let b = tree.b.eval(x, y, x_index, y_index);

            let r = ((r + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
            let g = ((g + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
            let b = ((b + 1.0) * 127.5).clamp(0.0, 255.0) as u8;

            vec![r, g, b]
        })
        .collect();

    ImageBuffer::from_vec(width, height, buffer)
        .expect("Failed to construct image from buffer")
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