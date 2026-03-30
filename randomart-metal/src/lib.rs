mod metal_codegen;
pub mod gpu;

use randomart_core::{
    grammar::generate_tree_parallel,
    node::Node,
    pixel_buffer::{PixelBuffer, GenerateOutput, ReadOutput},
};
use crate::{
    metal_codegen::emit_metal_from_triple,
    gpu::run_gpu_kernel,
};
use xxhash_rust::xxh3::xxh3_64;

fn render_to_buffer(r: &Node, g: &Node, b: &Node, width: u32, height: u32) -> PixelBuffer {
    let metal_src = emit_metal_from_triple(r, g, b);
    run_gpu_kernel(&metal_src, width, height)
}

pub fn generate(string: &str, depth: u32, width: u32, height: u32) -> GenerateOutput {
    let seed: u64 = xxh3_64(string.as_bytes());
    let mut node = generate_tree_parallel(seed, depth).unwrap();
    node.simplify_triple();

    let json = serde_json::to_string_pretty(&*node).unwrap();

    let Node::Triple(r, g, b) = *node else {
        panic!("Expected top-level Triple node");
    };

    let pixels = render_to_buffer(&r, &g, &b, width, height);
    GenerateOutput { pixels, json }
}

pub fn read_json(json: &str, width: u32, height: u32) -> ReadOutput {
    let node: Node = serde_json::from_str(json).expect("failed to deserialize node from JSON");

    let Node::Triple(r, g, b) = node else {
        panic!("Expected top-level Triple node");
    };

    let pixels = render_to_buffer(&r, &g, &b, width, height);
    ReadOutput { pixels }
}
