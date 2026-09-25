mod jit;

use crate::jit::build_jit_function_triple;
use engine::{
    backend::Backend,
    tree::node::Node,
    render::pixel_buffer::PixelBuffer,
    render::tiled::render_channels,
};
use anyhow::{Context, Result};

pub struct Cranelift;

impl Backend for Cranelift {
    fn render(&self, node: &Node, width: u32, height: u32) -> Result<PixelBuffer> {
        let (r, g, b) = node.as_triple().context("top-level node must be a Triple")?;
        let (r_fn, g_fn, b_fn) = build_jit_function_triple(r, g, b);
        Ok(render_channels(r_fn, g_fn, b_fn, width, height))
    }
}
