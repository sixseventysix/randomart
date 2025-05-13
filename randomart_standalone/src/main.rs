use std::env;
use std::time::Instant;
use randomart_standalone::{grammar::Grammar, closure_tree::ClosureTree, utils::{render_pixels, fnv1a}};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} generate <string> <depth> [output_namespace] [width] [height]", args[0]);
        std::process::exit(1);
    }

    let string = args[2].clone();
    let depth: u32 = args.get(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            eprintln!("Error: depth must be a positive integer");
            std::process::exit(1);
        });

    let output_namespace = args.get(4).cloned().unwrap_or_else(|| string.clone());
    let output_img_filename = format!("{}.png", output_namespace);
    let output_formula_filename = format!("{}.txt", output_namespace);

    let width = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(400);
    let height = args.get(6).and_then(|s| s.parse().ok()).unwrap_or(400);

    let start_total = Instant::now();

    let seed = fnv1a(&string);
    let mut grammar = Grammar::default(seed);

    let start_gen = Instant::now();
    let generated_node = grammar.gen_rule(0, depth).expect("Failed to generate rule");
    let elapsed_gen = start_gen.elapsed();
    let mut node = generated_node.clone();
    node.simplify_triple();

    let start_tree = Instant::now();
    let closure_tree = ClosureTree::from_node(&generated_node);
    let elapsed_tree = start_tree.elapsed();

    println!("{:#?}", closure_tree);

    let start_render = Instant::now();
    let img = render_pixels(&closure_tree, width, height);
    let elapsed_render = start_render.elapsed();
    img.save(output_img_filename).expect("Failed to save image");
    std::fs::write(output_formula_filename, format!("{}", generated_node)).expect("Failed to write formula");
    
    println!("Tree generation:      {:?}", elapsed_gen);
    println!("Closure tree creation:{:?}", elapsed_tree);
    println!("Image render time:    {:?}", elapsed_render);
    println!("Total time:           {:?}", start_total.elapsed());
}