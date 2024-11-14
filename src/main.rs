use randomart::{utils::{ fnv1a, render_pixels, PixelCoordinates }, Grammar, Node};
use std::env;

fn print_channels_from_triple(node: &Node) {
    match node {
        Node::Triple(left, middle, right) => {
            println!("R: {:?}", left);
            println!("G: {:?}", middle);
            println!("B: {:?}", right);
        }
        _ => {
            println!("node is not a Node::Triple");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("usage: {} <string> <depth> <output file path>", args[0]);
        std::process::exit(1);
    }

    let string = args[1].clone();
    let depth: u32 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Error: depth must be a positive integer");
        std::process::exit(1);
    });
    let output_filename = args[3].clone();
    let seed = fnv1a(&string);
    let mut grammar = Grammar::default(seed);
    
    let start_rule = 0;
    let generated_node = grammar.gen_rule(start_rule, depth).unwrap();
    print_channels_from_triple(&generated_node);

    let rgb_function = |coords: PixelCoordinates| {
        generated_node.eval_rgb(coords.x, coords.y)
    };
    let img = render_pixels(rgb_function);

    let output_filepath = format!("data/images/{}.png", &output_filename);
    img.save(output_filepath).expect("failed to save the image");
}