pub mod utils;

use utils::compile;
use engine::{
    backend::Backend,
    op::Op,
    render::pixel_buffer::PixelBuffer,
    render::render_channels,
};
use anyhow::Result;

pub struct ClosureTree;

impl Backend for ClosureTree {
    fn render(&self, channels: &[Vec<Op>; 3], width: u32, height: u32) -> Result<PixelBuffer> {
        let [r, g, b] = channels;
        Ok(render_channels(compile(r), compile(g), compile(b), width, height))
    }
}
