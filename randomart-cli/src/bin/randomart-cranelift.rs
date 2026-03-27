use clap::Parser;
use randomart_cli::{Cli, RandomArtBackend, run};
use randomart_core::pixel_buffer::{GenerateOutput, ReadOutput};

struct Cranelift;

impl RandomArtBackend for Cranelift {
    fn generate(string: &str, depth: u32, width: u32, height: u32) -> GenerateOutput {
        randomart_cranelift_jit::generate(string, depth, width, height)
    }
    fn read_json(json: &str, width: u32, height: u32) -> ReadOutput {
        randomart_cranelift_jit::read_json(json, width, height)
    }
}

fn main() {
    run::<Cranelift>(Cli::parse());
}
