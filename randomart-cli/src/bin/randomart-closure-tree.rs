use clap::Parser;
use randomart_cli::{Cli, RandomArtBackend, run};
use randomart_core::pixel_buffer::{GenerateOutput, ReadOutput};

struct ClosureTree;

impl RandomArtBackend for ClosureTree {
    fn generate(string: &str, depth: u32, width: u32, height: u32) -> GenerateOutput {
        randomart_closure_tree::generate(string, depth, width, height)
    }
    fn read_json(json: &str, width: u32, height: u32) -> ReadOutput {
        randomart_closure_tree::read_json(json, width, height)
    }
}

fn main() {
    run::<ClosureTree>(Cli::parse());
}
