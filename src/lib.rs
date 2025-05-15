mod utils;
mod reader;
mod grammar;
mod node;
mod statistics;
mod jit;

use crate::{
    utils::{ fnv1a, render_pixels, PixelCoordinates, Colour }, 
    grammar::Grammar, 
    reader::{tokenize, parse_expr},
    statistics::{TreeStats},
    jit::build_jit_function_triple
};
use std::time::Instant;

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
        let seed = fnv1a(&self.string);
        let mut grammar = Grammar::default(seed);

        let start = Instant::now();
        let mut node = grammar.gen_top_rule(self.depth).unwrap();
        let elaps_gen = start.elapsed();

        node.simplify_triple();
        let stats = TreeStats::from_triple(&node);
        let formula = format!("{}", node);

        let (r_jit_fn, g_jit_fn, b_jit_fn) = build_jit_function_triple(&node);
        let rgb_fn = |coord: PixelCoordinates| Colour {
            r: r_jit_fn(coord.x, coord.y),
            g: g_jit_fn(coord.x, coord.y),
            b: b_jit_fn(coord.x, coord.y),
        };

        let img = render_pixels(&rgb_fn, self.width, self.height);
        let output_img = get_output_path(&format!("{}.png", self.output_file_namespace));
        let output_formula = get_output_path(&format!("{}.txt", self.output_file_namespace));
        img.save(output_img).unwrap();
        std::fs::write(output_formula, formula).unwrap();

        println!("\nrandomart\nstr: {}\ndepth:{}\nwidth:{} height:{}\ngeneration: {:?}", 
            self.string, self.depth, self.width, self.height, elaps_gen);
        stats.report();
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
        let input = std::fs::read_to_string(&self.input_file).expect("Failed to read file");
        let tokens = tokenize(&input);
        let mut iter = tokens.into_iter();
        let node = parse_expr(&mut iter);

        let (r_jit_fn, g_jit_fn, b_jit_fn) = build_jit_function_triple(&node);
        let rgb_fn = |coord: PixelCoordinates| Colour {
            r: r_jit_fn(coord.x, coord.y),
            g: g_jit_fn(coord.x, coord.y),
            b: b_jit_fn(coord.x, coord.y),
        };

        let img = render_pixels(&rgb_fn, self.width, self.height);
        img.save(format!("{}.png", self.output_file_namespace)).unwrap();
    }
}