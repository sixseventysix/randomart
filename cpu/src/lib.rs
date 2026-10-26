pub mod ftz;
pub mod render;
pub mod vm;

use anyhow::Result;
use randomart::{pcfg::OpGenerator, seed::derive_seeds};
use render::{pixel_buffer::PixelBuffer, render};

pub fn randomart(text: &str, depth: u32, width: u32, height: u32) -> Result<PixelBuffer> {
    let seeds = derive_seeds(text);
    let expressions = seeds.map(|seed| OpGenerator::new(seed, depth).collect());
    render(&expressions, width, height)
}
