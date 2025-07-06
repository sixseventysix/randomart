mod reader;
mod grammar;
mod node;
mod statistics;
mod rng;
mod metal_codegen;

use std::fs::File;
use std::io::Write;
use crate::{
    reader::{TokenStream, parse_expr},
    grammar::generate_tree_parallel,
    statistics::{TreeStats},
    metal_codegen::{emit_metal_from_triple, CodegenCtx }
};
use std::time::Instant;
use xxhash_rust::xxh3::xxh3_64;

fn get_output_path(file_name: &str) -> std::path::PathBuf {
    let current_dir = std::env::current_dir().expect("failed to get the current working directory");
    current_dir.join(file_name)
}

pub struct RandomArtGenerateCtx {
    pub string: String,
    pub depth: u32,
}

impl RandomArtGenerateCtx {
    pub fn run(&self) {
        let seed: u64 = xxh3_64(self.string.as_bytes());
        let mut node = generate_tree_parallel(seed, self.depth).unwrap();

        node.simplify_triple();

        let stats = TreeStats::from_triple(&node);
        let formula = format!("{}", node);

        let crate::node::Node::Triple(r, g, b) = *node else {
            panic!("Expected top-level Triple node");
        };
        
        let out = emit_metal_from_triple(&r, &g, &b);
        let output_metal_filename = get_output_path(&format!("data/metal/randomart_shader.metal"));
        let mut file = File::create(output_metal_filename).expect("error while creating randomart_shader.metal file");
        file.write_all(out.as_bytes()).expect("error while writing out to randomart_shader.metal");

        let output_formula = get_output_path(&format!("data/randomart_spec_lang/{}.txt", self.string));
        std::fs::write(output_formula, formula).unwrap();

        println!("\nrandomart\nstr: {}\ndepth:{}\n", 
            self.string, self.depth);
        stats.report();
    }
 }

pub struct RandomArtReadCtx {
    pub input_file: String,
}

impl RandomArtReadCtx {
    pub fn run(&self) {
        let input = std::fs::read_to_string(format!("{}", &self.input_file)).expect("Failed to read file");
        let mut ts = TokenStream::new(&input);
        let node = parse_expr(&mut ts);

        let stats = TreeStats::from_triple(&node);

        let crate::node::Node::Triple(r, g, b) = node else {
            panic!("Expected top-level Triple node");
        };
        
        let out = emit_metal_from_triple(&r, &g, &b);
        let output_metal_filename = get_output_path(&format!("data/metal/randomart_shader.metal"));
        let mut file = File::create(output_metal_filename).expect("error while creating randomart_shader.metal file");
        file.write_all(out.as_bytes()).expect("error while writing out to randomart_shader.metal");

        println!("\nrandomart\nstr: {}\ndepth:{}\n", 
            self.string, self.depth);
        stats.report();
    }
}