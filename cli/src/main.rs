use anyhow::Result;
use clap::Parser;
use cli::{run, Cli};

// Exactly one backend feature must be enabled. Alias the selected backend
// to `SelectedBackend` so the rest of this file is backend-agnostic.
// Precedence (closure > cranelift > metal) keeps exactly one alias active even
// when several features are on, so the compile_error! below is the only error.
#[cfg(feature = "closure")]
use closure_tree::ClosureTree as SelectedBackend;
#[cfg(all(feature = "cranelift", not(feature = "closure")))]
use cranelift_backend::Cranelift as SelectedBackend;
#[cfg(all(feature = "metal", not(feature = "closure"), not(feature = "cranelift")))]
use metal::Metal as SelectedBackend;

#[cfg(not(any(feature = "closure", feature = "cranelift", feature = "metal")))]
compile_error!("no backend selected: enable one of the `closure`, `cranelift`, or `metal` features");

#[cfg(any(
    all(feature = "closure", feature = "cranelift"),
    all(feature = "closure", feature = "metal"),
    all(feature = "cranelift", feature = "metal"),
))]
compile_error!("multiple backends selected: enable exactly one of `closure`, `cranelift`, `metal`");

fn main() -> Result<()> {
    run(&SelectedBackend, Cli::parse())
}
