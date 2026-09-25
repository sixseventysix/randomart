use crate::{op::Op, render::pixel_buffer::PixelBuffer};
use anyhow::Result;

pub trait Backend {
    fn render(&self, channels: &[Vec<Op>; 3], width: u32, height: u32) -> Result<PixelBuffer>;
}
