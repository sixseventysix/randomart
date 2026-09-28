pub mod op;
pub mod pcfg;
pub mod seed;
pub mod math;
pub mod ftz;
pub mod render;
pub mod vm;

use anyhow::Result;
use pcfg::OpGenerator;
use render::{pixel_buffer::PixelBuffer, render};
use seed::derive_seeds;

pub fn randomart(text: &str, depth: u32, width: u32, height: u32) -> Result<PixelBuffer> {
    let seeds = derive_seeds(text);
    let expressions = seeds.map(|seed| OpGenerator::new(seed, depth).collect());
    render(&expressions, width, height)
}
