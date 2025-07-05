mod reader;
mod grammar;
mod node;
mod statistics;
mod jit;
mod render;
mod rng;

use crate::{
    render::{ render_pixels, PixelCoordinates, Colour }, 
    grammar::generate_tree_parallel, 
    reader::{TokenStream, parse_expr},
    statistics::{TreeStats},
    jit::build_jit_function_triple
};
use std::time::Instant;
use xxhash_rust::xxh3::xxh3_64;

fn get_output_path(file_name: &str) -> std::path::PathBuf {
    let current_dir = std::env::current_dir().expect("failed to get the current working directory");
    current_dir.join(file_name)
}

pub struct RandomArtGenerate {
    pub string: String,
    pub depth: u32,
    pub width: u32,
    pub height: u32,
    pub output_file_namespace: String,
}

impl RandomArtGenerate {
    pub fn run(&self) {
        let seed: u64 = xxh3_64(self.string.as_bytes());
        let start1 = Instant::now();
        let mut node = generate_tree_parallel(seed, self.depth).unwrap();
        let elaps1 = start1.elapsed();

        let start2 = Instant::now();
        node.simplify_triple();
        let elaps2 = start2.elapsed();

        let start3 = Instant::now();
        let stats = TreeStats::from_triple(&node);
        let elaps3 = start3.elapsed();
        
        let start4 = Instant::now();
        let formula = format!("{}", node);
        let elaps4 = start4.elapsed();

        let start5 = Instant::now();
        let (r_jit_fn, g_jit_fn, b_jit_fn) = build_jit_function_triple(&node);
        let rgb_fn = |coord: PixelCoordinates| Colour {
            r: r_jit_fn(coord.x, coord.y),
            g: g_jit_fn(coord.x, coord.y),
            b: b_jit_fn(coord.x, coord.y),
        };
        let elaps5 = start5.elapsed();

        let start6 = Instant::now();
        let img = render_pixels(&rgb_fn, self.width, self.height);
        let elaps6 = start6.elapsed();

        let output_img = get_output_path(&format!("{}.png", self.output_file_namespace));
        let output_formula = get_output_path(&format!("{}.txt", self.output_file_namespace));
        img.save(output_img).unwrap();
        std::fs::write(output_formula, formula).unwrap();

        println!("\nrandomart\nstr: {}\ndepth:{}\nwidth:{} height:{}\n", 
            self.string, self.depth, self.width, self.height);
        stats.report();

        println!("\n\ngenerate_tree_parallel: {:?}", elaps1);
        println!("simplify: {:?}", elaps2);
        println!("stats: {:?}", elaps3);
        println!("saving formula as string: {:?}", elaps4);
        println!("building jit compiled fn: {:?}", elaps5);
        println!("render pixels: {:?}", elaps6);
    }
 }

pub struct RandomArtRead {
    pub input_file: String,
    pub width: u32,
    pub height: u32,
    pub output_file_namespace: String,
}

impl RandomArtRead {
    pub fn run(&self) {
        let input = std::fs::read_to_string(format!("{}.txt", &self.input_file)).expect("Failed to read file");

        let start1 = Instant::now();
        let mut ts = TokenStream::new(&input);
        let elaps1 = start1.elapsed();

        let start2 = Instant::now();
        let node = parse_expr(&mut ts);
        let elaps2 = start2.elapsed();

        let start3 = Instant::now();
        let (r_jit_fn, g_jit_fn, b_jit_fn) = build_jit_function_triple(&node);
        let rgb_fn = |coord: PixelCoordinates| Colour {
            r: r_jit_fn(coord.x, coord.y),
            g: g_jit_fn(coord.x, coord.y),
            b: b_jit_fn(coord.x, coord.y),
        };
        let elaps3 = start3.elapsed();

        let start4 = Instant::now();
        let img = render_pixels(&rgb_fn, self.width, self.height);
        let elaps4 = start4.elapsed();

        println!("tokenize: {:?}", elaps1);
        println!("parse_expr: {:?}", elaps2);
        println!("building jit compiled fn: {:?}", elaps3);
        println!("render pixels: {:?}", elaps4);
        img.save(format!("{}.png", self.output_file_namespace)).unwrap();
    }
}