use randomart::{utils::{ fnv1a, render_pixels, PixelCoordinates }, Grammar};
use std::{env, path::PathBuf};

fn get_output_path(file_name: &str) -> PathBuf {
    let current_dir = env::current_dir().expect("failed to get the current working directory");
    current_dir.join(file_name)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 || args.len() > 6 {
        eprintln!("usage: {} <string> <depth> <output file path> <width>(optional) <height>(optional)", args[0]);
        std::process::exit(1);
    }

    let string = args[1].clone();
    let depth: u32 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("error: depth must be a positive integer");
        std::process::exit(1);
    });
    let output_filename = format!("{}.png", args[3]);

    let width: u32 = args.get(4).map_or(400, |arg| {
        arg.parse().unwrap_or_else(|_| {
            eprintln!("ERR: invalid width, must be a positive integer");
            std::process::exit(1);
        })
    });
    let height: u32 = args.get(5).map_or(400, |arg| {
        arg.parse().unwrap_or_else(|_| {
            eprintln!("ERR: invalid height, must be a positive integer");
            std::process::exit(1);
        })
    });

    let seed = fnv1a(&string);
    let mut grammar = Grammar::default(seed);
    
    let start_rule = 0;
    let mut generated_node = grammar.gen_rule(start_rule, depth).unwrap();
    generated_node.simplify_triple();
    let (r_str_optimised, g_str_optimised, b_str_optimised) = generated_node.extract_channels_as_str_from_triple();
    println!("\nR:{}\n\nG:{}\n\nB:{}", r_str_optimised, g_str_optimised, b_str_optimised);

    let rgb_function = |coords: PixelCoordinates| {
        generated_node.eval_rgb(coords.x, coords.y)
    };
    
    let img = render_pixels(rgb_function, width, height);

    let output_filepath = get_output_path(&output_filename);
    img.save(output_filepath.clone()).expect("failed to save the image");
}