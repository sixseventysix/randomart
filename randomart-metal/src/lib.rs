mod grammar;
mod node;
mod statistics;
mod rng;
mod metal_codegen;
pub mod gpu;

use crate::{
    grammar::generate_tree_parallel,
    statistics::TreeStats,
    metal_codegen::emit_metal_from_triple,
    gpu::{compile_metal, run_gpu_kernel},
};
use xxhash_rust::xxh3::xxh3_64;

fn get_output_path(file_name: &str) -> std::path::PathBuf {
    let current_dir = std::env::current_dir().expect("failed to get the current working directory");
    current_dir.join(file_name)
}

fn render_and_save(r: &node::Node, g: &node::Node, b: &node::Node, out_png: &str, width: u32, height: u32) {
    let out_path = get_output_path(out_png);
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).expect("failed to create output directory");
    }
    let metal_src = emit_metal_from_triple(r, g, b);

    let src_file = tempfile::Builder::new()
        .suffix(".metal")
        .tempfile()
        .expect("failed to create temp .metal file");
    std::fs::write(src_file.path(), metal_src.as_bytes())
        .expect("failed to write .metal source");

    let lib_file = tempfile::Builder::new()
        .suffix(".metallib")
        .tempfile()
        .expect("failed to create temp .metallib file");

    compile_metal(src_file.path(), lib_file.path())
        .expect("Metal shader compilation failed");

    let img = run_gpu_kernel(lib_file.path(), width, height);
    img.save(get_output_path(out_png)).expect("failed to save image");
}

pub struct RandomArtGenerateCtx {
    pub string: String,
    pub depth: u32,
    pub width: u32,
    pub height: u32,
}

impl RandomArtGenerateCtx {
    pub fn run(&self) {
        let seed: u64 = xxh3_64(self.string.as_bytes());
        let mut node = generate_tree_parallel(seed, self.depth).unwrap();
        node.simplify_triple();

        let stats = TreeStats::from_triple(&node);

        let node::Node::Triple(r, g, b) = *node else {
            panic!("Expected top-level Triple node");
        };

        let out_png = format!("data/images/{}.png", self.string);
        render_and_save(&r, &g, &b, &out_png, self.width, self.height);

        let tree = node::Node::Triple(r, g, b);
        let output_json = get_output_path(&format!("data/formulas/{}.json", self.string));
        std::fs::create_dir_all(output_json.parent().unwrap()).expect("failed to create formulas directory");
        let json = serde_json::to_string_pretty(&tree).unwrap();
        std::fs::write(output_json, json).unwrap();

        println!("\nrandomart\nstr: {}\ndepth:{}\nwidth:{} height:{}\n",
            self.string, self.depth, self.width, self.height);
        stats.report();
    }
}

pub struct RandomArtReadCtx {
    pub input_filepath: String,
    pub width: u32,
    pub height: u32,
}

impl RandomArtReadCtx {
    pub fn run(&self) {
        let json = std::fs::read_to_string(&self.input_filepath)
            .expect("failed to read file");
        let node: node::Node = serde_json::from_str(&json)
            .expect("failed to deserialize node from JSON");

        let stats = TreeStats::from_triple(&node);

        let node::Node::Triple(r, g, b) = node else {
            panic!("Expected top-level Triple node");
        };

        let stem = std::path::Path::new(&self.input_filepath)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&self.input_filepath);
        let out_png = format!("data/images/{}.png", stem);
        render_and_save(&r, &g, &b, &out_png, self.width, self.height);

        println!("\nrandomart\ninput filepath:{}\n", self.input_filepath);
        stats.report();
    }
}
