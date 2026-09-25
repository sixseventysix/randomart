pub mod utils;

use utils::compile_node;
use engine::{
    backend::Backend,
    tree::node::Node,
    render::pixel_buffer::PixelBuffer,
    render::tiled::render_channels,
};
use anyhow::{Context, Result};

pub struct ClosureTree;

impl Backend for ClosureTree {
    fn render(&self, node: &Node, width: u32, height: u32) -> Result<PixelBuffer> {
        let (r, g, b) = node.as_triple().context("top-level node must be a Triple")?;
        Ok(render_channels(compile_node(r), compile_node(g), compile_node(b), width, height))
    }
}
