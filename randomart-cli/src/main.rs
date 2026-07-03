use anyhow::Result;
use clap::Parser;
use randomart_cli::{run, Cli, RandomArtBackend};
use randomart_core::pixel_buffer::{GenerateOutput, ReadOutput};

// Exactly one backend feature must be enabled. Alias the selected backend crate
// to `backend` so the rest of this file is backend-agnostic.
// Precedence (closure > cranelift > metal) keeps exactly one alias active even
// when several features are on, so the compile_error! below is the only error.
#[cfg(feature = "closure")]
use randomart_closure_tree as backend;
#[cfg(all(feature = "cranelift", not(feature = "closure")))]
use randomart_cranelift_jit as backend;
#[cfg(all(feature = "metal", not(feature = "closure"), not(feature = "cranelift")))]
use randomart_metal as backend;

#[cfg(not(any(feature = "closure", feature = "cranelift", feature = "metal")))]
compile_error!("no backend selected: enable one of the `closure`, `cranelift`, or `metal` features");

#[cfg(any(
    all(feature = "closure", feature = "cranelift"),
    all(feature = "closure", feature = "metal"),
    all(feature = "cranelift", feature = "metal"),
))]
compile_error!("multiple backends selected: enable exactly one of `closure`, `cranelift`, `metal`");

struct Backend;

impl RandomArtBackend for Backend {
    fn generate(string: &str, depth: u32, width: u32, height: u32) -> Result<GenerateOutput> {
        backend::generate(string, depth, width, height)
    }
    fn read_json(json: &str, width: u32, height: u32) -> Result<ReadOutput> {
        backend::read_json(json, width, height)
    }
}

fn main() -> Result<()> {
    run::<Backend>(Cli::parse())
}
