use randomart_closure_tree::{utils::{ fnv1a, render_pixels, PixelCoordinates }, Grammar, ClosureTree};
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
    let output_img_filename = format!("{}.png", args[3]);
    let output_formula_filename = format!("{}.txt", args[3]);

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

    let formula = format!("{}", generated_node);
    let closure_tree = ClosureTree::from_node(&generated_node);

    let rgb_fn = move |coord: PixelCoordinates| closure_tree.eval_rgb(coord.x, coord.y);
    let img = render_pixels(rgb_fn, width, height);

    let output_img_filepath = get_output_path(&output_img_filename);
    img.save(output_img_filepath.clone()).expect("failed to save the image");
    let output_formula_filepath = get_output_path(&output_formula_filename);
    std::fs::write(output_formula_filepath, formula).unwrap();
}