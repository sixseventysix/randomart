use randomart::{
    utils::{ fnv1a, render_pixels, PixelCoordinates, Colour }, 
    grammar::Grammar, 
    closure_tree::ClosureTree,
    reader::{tokenize, parse_expr},
    statistics::{TreeStats},
    node::Node,
    jit::build_jit_function
};
use std::{
    env, 
    path::PathBuf,
    time::Instant
};

fn get_output_path(file_name: &str) -> PathBuf {
    let current_dir = env::current_dir().expect("failed to get the current working directory");
    current_dir.join(file_name)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("generate") => {
            if args.len() < 4 || args.len() > 7 {
                eprintln!("usage: {} generate <string> <depth> <width>(optional) <height>(optional) <output file name>(optional)", args[0]);
                std::process::exit(1);
            }

            let string = args[2].clone();
            let depth: u32 = args[3].parse().unwrap_or_else(|_| {
                eprintln!("error: depth must be a positive integer");
                std::process::exit(1);
            });

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

            let output_file_namespace = args.get(6).map_or(string.clone(), |arg| {
                arg.parse().unwrap_or_else(|_| {
                    std::process::exit(1);
                })
            });
            let output_img_filename = format!("{}.png", output_file_namespace);
            let output_formula_filename = format!("{}.txt", output_file_namespace);

            let seed = fnv1a(&string);
            let mut grammar = Grammar::default(seed);
            let start_rule = 0;
            
            let start5 = Instant::now();
            let mut generated_node = grammar.gen_rule(start_rule, depth).unwrap();
            let elaps5 = start5.elapsed();

            let start3 = Instant::now();
            generated_node.simplify_triple();
            let elaps3 = start3.elapsed();

            let (r, g, b) = match &*generated_node {
                Node::Triple(r, g, b) => (r, g, b),
                _ => panic!("Expected Triple node at top level"),
            };
            let start6 = Instant::now();
            let r_stats = TreeStats::from_node(r);
            let g_stats = TreeStats::from_node(g);
            let b_stats = TreeStats::from_node(b);
            let elaps6 = start6.elapsed();

            let formula = format!("{}", generated_node);

            let start7 = Instant::now();
            let r_jit_fn = build_jit_function(r);
            let g_jit_fn = build_jit_function(g);
            let b_jit_fn = build_jit_function(b);
            let elaps7 = start7.elapsed();
            // let start4 = Instant::now();
            // let closure_tree = ClosureTree::from_node(&generated_node);
            // let elaps4 = start4.elapsed();

            // let rgb_fn = move |coord: PixelCoordinates| closure_tree.eval_rgb(coord.x, coord.y);
            let rgb_fn = move |coord: PixelCoordinates| {
                Colour {
                    r: r_jit_fn(coord.x, coord.y),
                    g: g_jit_fn(coord.x, coord.y),
                    b: b_jit_fn(coord.x, coord.y),
                }
            };

            let start2 = Instant::now();
            let img = render_pixels(&rgb_fn, width, height);
            let elaps2 = start2.elapsed();

            println!("randomart\nstr: {string}\ndepth:{depth}\nwidth:{width} height:{height}\n\n");
            println!("R channel report:");
            r_stats.report();
            println!("\nG channel report:");
            g_stats.report();
            println!("\nB channel report:");
            b_stats.report();
            println!("\ntree generation: {:?}", elaps5);
            println!("simplify: {:?}", elaps3);
            println!("tree stats: {:?}", elaps6);
            // println!("closure tree creation: {:?}", elaps4);
            println!("fn creation via cranelift: {:?}", elaps7);
            println!("hot path (render pixels loop): {:?}", elaps2);

            let output_img_filepath = get_output_path(&output_img_filename);
            img.save(output_img_filepath.clone()).expect("failed to save the image");
            let output_formula_filepath = get_output_path(&output_formula_filename);
            std::fs::write(output_formula_filepath, formula).unwrap();
        }
        Some("read") => {
            if args.len() < 3 || args.len() > 6 {
                eprintln!("usage: {} read <input file name> <width>(optional) <height>(optional) <output file name>(optional)", args[0]);
                std::process::exit(1);
            }
            let input = std::fs::read_to_string(&args[2]).expect("Failed to read file");
            let width: u32 = args.get(3).map_or(400, |arg| {
                arg.parse().unwrap_or_else(|_| {
                    eprintln!("ERR: invalid width, must be a positive integer");
                    std::process::exit(1);
                })
            });
            let height: u32 = args.get(4).map_or(400, |arg| {
                arg.parse().unwrap_or_else(|_| {
                    eprintln!("ERR: invalid height, must be a positive integer");
                    std::process::exit(1);
                })
            });
            let output_file_namespace = args.get(5).map_or(input.clone(), |arg| {
                arg.parse().unwrap_or_else(|_| {
                    std::process::exit(1);
                })
            });

            let start1 = Instant::now();
            let tokens = tokenize(&input);
            let mut iter = tokens.into_iter();
            let elaps1 = start1.elapsed();
            let start2 = Instant::now();
            let node = parse_expr(&mut iter);
            let elaps2 = start2.elapsed();

            let start4 = Instant::now();
            let closure_tree = ClosureTree::from_node(&node);
            let elaps4 = start4.elapsed();

            let rgb_fn = move |coord: PixelCoordinates| closure_tree.eval_rgb(coord.x, coord.y);
            let elaps1_1 = start1.elapsed();

            let start3 = Instant::now();
            let img = render_pixels(&rgb_fn, width, height);
            let elaps3 = start3.elapsed();

            println!("tokenize and iter creation: {:?}", elaps1);
            println!("parse_expr: {:?}", elaps2);
            println!("closure tree creation: {:?}", elaps4);
            println!("tokenize to rgb fn creation: {:?}", elaps1_1);
            println!("hot path (render pixels loop): {:?}", elaps3);

            img.save(format!("{}.png", output_file_namespace)).unwrap();
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("{} generate <string> <depth> <output file path>(optional) <width>(optional) <height>(optional)", args[0]);
            eprintln!("{} read <input file name> <output file path>(optional) <width>(optional) <height>(optional)", args[0]);
        }
    }
    
}