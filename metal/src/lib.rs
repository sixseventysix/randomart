mod metal_codegen;
pub mod gpu;

use engine::{
    backend::Backend,
    tree::node::Node,
    render::pixel_buffer::PixelBuffer,
};
use crate::{
    metal_codegen::emit_metal_from_triple,
    gpu::run_gpu_kernel,
};
use anyhow::{Context, Result};

pub struct Metal;

impl Backend for Metal {
    fn render(&self, node: &Node, width: u32, height: u32) -> Result<PixelBuffer> {
        let (r, g, b) = node.as_triple().context("top-level node must be a Triple")?;
        let metal_src = emit_metal_from_triple(r, g, b);
        run_gpu_kernel(&metal_src, width, height)
    }
}
