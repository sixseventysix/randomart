use clap::{Parser, Subcommand};
use image::RgbImage;
use randomart_core::pixel_buffer::{GenerateOutput, PixelBuffer, ReadOutput};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(about = "Generate randomart images")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
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

pub trait RandomArtBackend {
    fn generate(string: &str, depth: u32, width: u32, height: u32) -> GenerateOutput;
    fn read_json(json: &str, width: u32, height: u32) -> ReadOutput;
}

pub fn run<B: RandomArtBackend>(cli: Cli) {
    match cli.command {
        Command::Generate { string, depth, width, height, out, save_json } => {
            let stem = out.unwrap_or_else(|| string.clone());
            let output = B::generate(&string, depth, width, height);

            save_image(output.pixels, &pwd(&format!("{stem}.png")));

            if save_json {
                std::fs::write(pwd(&format!("{stem}.json")), &output.json)
                    .expect("failed to write json");
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

            let json = std::fs::read_to_string(&input).expect("failed to read input file");
            let output = B::read_json(&json, width, height);

            save_image(output.pixels, &pwd(&format!("{stem}.png")));
        }
    }
}

fn save_image(buf: PixelBuffer, path: &Path) {
    RgbImage::from_raw(buf.width, buf.height, buf.data)
        .expect("PixelBuffer to RgbImage failed")
        .save(path)
        .expect("failed to save image");
}

fn pwd(filename: &str) -> PathBuf {
    std::env::current_dir()
        .expect("failed to get current directory")
        .join(filename)
}
