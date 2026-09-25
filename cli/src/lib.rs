use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use image::RgbImage;
use engine::{
    backend::Backend,
    derivation::seed::generate_from_str,
    render::pixel_buffer::PixelBuffer,
    tree::node::Node,
};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(about = "Generate randomart images")]
pub struct Cli {
    #[arg(long, value_enum, global = true)]
    pub backend: Option<BackendKind>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum BackendKind {
    #[cfg(feature = "closure")]
    Closure,
    #[cfg(feature = "cranelift")]
    Cranelift,
    #[cfg(feature = "metal")]
    Metal,
}

#[derive(Subcommand)]
pub enum Command {
    /// Generate an image from a string seed
    Generate {
        /// Input string used as seed
        string: String,

        /// Tree depth
        depth: u32,

        /// Image width in pixels
        #[arg(long, default_value_t = 512)]
        width: u32,

        /// Image height in pixels
        #[arg(long, default_value_t = 512)]
        height: u32,

        /// Output filename stem (default: the input string)
        #[arg(long)]
        out: Option<String>,

        /// Also write a .json file with the formula
        #[arg(long)]
        save_json: bool,
    },

    /// Render an image from a previously saved .json formula file
    Read {
        /// Path to the .json formula file
        input: String,

        /// Image width in pixels
        #[arg(long, default_value_t = 512)]
        width: u32,

        /// Image height in pixels
        #[arg(long, default_value_t = 512)]
        height: u32,

        /// Output filename stem (default: input file stem)
        #[arg(long)]
        out: Option<String>,
    },
}

pub fn run(backend: &dyn Backend, command: Command) -> Result<()> {
    match command {
        Command::Generate { string, depth, width, height, out, save_json } => {
            let stem = out.unwrap_or_else(|| string.clone());
            let node = generate_from_str(&string, depth).context("tree generation failed")?;
            let pixels = backend.render(&node, width, height)?;

            save_image(pixels, &pwd(&format!("{stem}.png")))?;

            if save_json {
                let path = pwd(&format!("{stem}.json"));
                let json = serde_json::to_string_pretty(&*node)
                    .context("failed to serialize node tree")?;
                std::fs::write(&path, json)
                    .with_context(|| format!("failed to write JSON to {}", path.display()))?;
            }
        }

        Command::Read { input, width, height, out } => {
            let stem = out.unwrap_or_else(|| {
                Path::new(&input)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(&input)
                    .to_string()
            });

            let json = std::fs::read_to_string(&input)
                .with_context(|| format!("failed to read input file {input}"))?;
            let node: Node = serde_json::from_str(&json)
                .context("failed to deserialize node tree from JSON")?;
            let pixels = backend.render(&node, width, height)?;

            save_image(pixels, &pwd(&format!("{stem}.png")))?;
        }
    }
    Ok(())
}

fn save_image(buf: PixelBuffer, path: &Path) -> Result<()> {
    RgbImage::from_raw(buf.width, buf.height, buf.data)
        .context("pixel buffer dimensions do not match its data length")?
        .save(path)
        .with_context(|| format!("failed to save image to {}", path.display()))?;
    Ok(())
}

fn pwd(filename: &str) -> PathBuf {
    std::env::current_dir()
        .expect("failed to get current directory")
        .join(filename)
}
