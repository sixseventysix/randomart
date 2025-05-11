use randomart::{utils::{ fnv1a, render_pixels, PixelCoordinates }, Grammar, ClosureTree};
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

    let (r_node, g_node, b_node) = match &*generated_node {
        randomart::Node::Triple(r, g, b) => (r, g, b),
            _ => panic!("Expected Node::Triple at the top level"),
    };

    let mut r_instructions = Vec::new();
    let mut g_instructions = Vec::new();
    let mut b_instructions = Vec::new();

    randomart::emit_postfix(r_node, &mut r_instructions);
    randomart::emit_postfix(g_node, &mut g_instructions);
    randomart::emit_postfix(b_node, &mut b_instructions);

    let program = randomart::PostfixRgbProgram {
        r: r_instructions,
        g: g_instructions,
        b: b_instructions,
    };

    let rgb_fn = program.to_fn();
    let img = render_pixels(rgb_fn, width, height);
    let output_img_filepath = get_output_path(&output_img_filename);
    img.save(output_img_filepath.clone()).expect("failed to save the image");
}