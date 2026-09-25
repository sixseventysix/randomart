use anyhow::{Context, Result};
use clap::Parser;
use cli::{run, BackendKind, Cli};
use engine::backend::Backend;

#[cfg(not(any(feature = "closure", feature = "cranelift", feature = "metal")))]
compile_error!("no backend compiled in: enable at least one of the `closure`, `cranelift`, or `metal` features");

fn main() -> Result<()> {
    let cli = Cli::parse();

    #[cfg(feature = "closure")]
    let default = Some(BackendKind::Closure);
    #[cfg(not(feature = "closure"))]
    let default = None;

    let kind = cli
        .backend
        .or(default)
        .context("--backend is required: the closure backend is not compiled in")?;

    let backend: Box<dyn Backend> = match kind {
        #[cfg(feature = "closure")]
        BackendKind::Closure => Box::new(closure_tree::ClosureTree),
        #[cfg(feature = "cranelift")]
        BackendKind::Cranelift => Box::new(cranelift_backend::Cranelift),
        #[cfg(feature = "metal")]
        BackendKind::Metal => Box::new(metal::Metal),
    };

    run(backend.as_ref(), cli.command)
}
