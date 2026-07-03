pub mod utils;

use utils::compile_node;
use randomart_core::{
    grammar::generate_tree_parallel,
    node::Node,
    pixel_buffer::{PixelBuffer, GenerateOutput, ReadOutput},
    render::{render_tiled, Colour, PixelCoordinates},
};
use anyhow::{bail, Context, Result};
use xxhash_rust::xxh3::xxh3_64;

fn render_node(node: &Node, width: u32, height: u32) -> Result<PixelBuffer> {
    let (r, g, b) = match node {
        Node::Triple(r, g, b) => (r.as_ref(), g.as_ref(), b.as_ref()),
        _ => bail!("top-level node must be a Triple"),
    };
    let r_fn = compile_node(r);
    let g_fn = compile_node(g);
    let b_fn = compile_node(b);
    Ok(render_tiled(
        &move |coord: PixelCoordinates| Colour {
            r: r_fn(coord.x, coord.y),
            g: g_fn(coord.x, coord.y),
            b: b_fn(coord.x, coord.y),
        },
        width,
        height,
    ))
}

pub fn generate(string: &str, depth: u32, width: u32, height: u32) -> Result<GenerateOutput> {
    let seed: u64 = xxh3_64(string.as_bytes());
    let mut node = generate_tree_parallel(seed, depth)
        .context("tree generation failed")?;
    node.simplify_triple();

    let json = serde_json::to_string_pretty(&*node)
        .context("failed to serialize node tree")?;
    let pixels = render_node(&node, width, height)?;
    Ok(GenerateOutput { pixels, json })
}

pub fn read_json(json: &str, width: u32, height: u32) -> Result<ReadOutput> {
    let node: Node = serde_json::from_str(json)
        .context("failed to deserialize node tree from JSON")?;
    let pixels = render_node(&node, width, height)?;
    Ok(ReadOutput { pixels })
}
