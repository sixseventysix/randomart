use std::{env, path::PathBuf};
use randomart_core::utils::{render_pixels, PixelCoordinates, Colour};
include!(concat!(env!("OUT_DIR"), "/generated_rgb_fn.rs"));

fn get_output_path(file_name: &str) -> PathBuf {
    let current_dir = env::current_dir().expect("failed to get the current working directory");
    current_dir.join(file_name)
}

fn main() {
    let width = 400;
    let height = 400;

    let r = r_fn();
    let g = g_fn();
    let b = b_fn();

    let rgb_function = move |coord: PixelCoordinates| Colour {
        r: r(coord.x, coord.y),
        g: g(coord.x, coord.y),
        b: b(coord.x, coord.y),
    };

    let img = render_pixels(rgb_function, width, height);
    img.save("output.png").expect("Failed to save image");
}