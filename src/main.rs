use std::{env, path::PathBuf};
use randomart_core::utils::{render_pixels, PixelCoordinates, Colour};
include!(concat!(env!("OUT_DIR"), "/generated_rgb_fn.rs"));

fn assert_send_sync<T: Send + Sync>(_f: &T) {}

fn main() {
    let width = 1920;
    let height = 1080;

    let start = std::time::Instant::now();

    let r = r_fn();
    assert_send_sync(&r);

    let g = g_fn();
    assert_send_sync(&g);

    let b = b_fn();
    assert_send_sync(&b);

    let rgb_function = move |coord: PixelCoordinates| Colour {
        r: r(coord.x, coord.y),
        g: g(coord.x, coord.y),
        b: b(coord.x, coord.y),
    };

    let img = render_pixels(&rgb_function, width, height);
    let elaps = start.elapsed();
    println!("elaps:{:?}", elaps);
    img.save("output.png").expect("Failed to save image");
}