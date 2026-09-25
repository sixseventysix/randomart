use crate::{tree::node::Node, render::pixel_buffer::PixelBuffer};
use anyhow::Result;

pub trait Backend {
    fn render(&self, node: &Node, width: u32, height: u32) -> Result<PixelBuffer>;
}
