use randomart::{utils::{ fnv1a, render_pixels, PixelCoordinates }, Grammar};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("usage: {} <string> <depth> <output file path>", args[0]);
        std::process::exit(1);
    }

    let string = args[1].clone();
    let depth: u32 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("error: depth must be a positive integer");
        std::process::exit(1);
    });
    let output_filename = args[3].clone();
    let seed = fnv1a(&string);
    let mut grammar = Grammar::default(seed);
    
    let start_rule = 0;
    let generated_node = grammar.gen_rule(start_rule, depth).unwrap();
    let (r_str, g_str, b_str) = generated_node.extract_channels_from_triple();
    println!("R:{}\n\nG:{}\n\nB:{}", r_str, g_str, b_str);

    let rgb_function = |coords: PixelCoordinates| {
        generated_node.eval_rgb(coords.x, coords.y)
    };
    let img = render_pixels(rgb_function);

    let output_filepath = format!("data/images/{}.png", &output_filename);
    img.save(output_filepath).expect("failed to save the image");
}