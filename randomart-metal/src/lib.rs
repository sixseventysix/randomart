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
use anyhow::{Context, Result};
use xxhash_rust::xxh3::xxh3_64;

fn render_to_buffer(r: &Node, g: &Node, b: &Node, width: u32, height: u32) -> Result<PixelBuffer> {
    let metal_src = emit_metal_from_triple(r, g, b);
    run_gpu_kernel(&metal_src, width, height)
}

pub fn generate(string: &str, depth: u32, width: u32, height: u32) -> Result<GenerateOutput> {
    let seed: u64 = xxh3_64(string.as_bytes());
    let mut node = generate_tree_parallel(seed, depth)
        .context("tree generation failed")?;
    node.simplify_triple();

    let json = serde_json::to_string_pretty(&*node)
        .context("failed to serialize node tree")?;

    let Node::Triple(r, g, b) = *node else {
        // Invariant: generate_tree_parallel always produces a Triple root.
        unreachable!("generated tree root is not a Triple");
    };

    let pixels = render_to_buffer(&r, &g, &b, width, height)?;
    Ok(GenerateOutput { pixels, json })
}

pub fn read_json(json: &str, width: u32, height: u32) -> Result<ReadOutput> {
    let node: Node = serde_json::from_str(json)
        .context("failed to deserialize node tree from JSON")?;

    let Node::Triple(r, g, b) = node else {
        anyhow::bail!("top-level JSON node must be a Triple");
    };

    let pixels = render_to_buffer(&r, &g, &b, width, height)?;
    Ok(ReadOutput { pixels })
}
