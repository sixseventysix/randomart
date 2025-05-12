use randomart::{utils::{ fnv1a, render_pixels, PixelCoordinates }, Grammar, ClosureTree};
use std::{env, path::PathBuf};

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

    let start = std::time::Instant::now();
    let seed = fnv1a(&string);
    let mut grammar = Grammar::default(seed);
    
    let start_rule = 0;
    let mut generated_node = grammar.gen_rule(start_rule, depth).unwrap();

    generated_node.simplify_triple();

    let (r_node, g_node, b_node) = match &*generated_node {
        randomart::Node::Triple(r, g, b) => (r, g, b),
            _ => panic!("Expected Node::Triple at the top level"),
    };

    let img = render_pixels(&rgb_function, width, height);
    let elaps = start.elapsed();
    println!("elaps:{:?}", elaps);
    img.save("output.png").expect("Failed to save image");
}