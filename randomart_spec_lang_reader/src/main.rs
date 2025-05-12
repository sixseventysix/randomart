use std::env;
use randomart_spec_lang_reader::{
    utils::{render_pixels, PixelCoordinates},
    reader::{parse_expr, tokenize},
    ClosureTree
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = std::fs::read_to_string(&args[1]).expect("Failed to read formula");

    let tokens = tokenize(&input);
    let mut iter = tokens.into_iter();
    let node = parse_expr(&mut iter);
    let formula = format!("{}", node);
    let closure_tree = ClosureTree::from_node(&node);
    let rgb_fn = move |coord: PixelCoordinates| closure_tree.eval_rgb(coord.x, coord.y);

    let img = render_pixels(&rgb_fn, 400, 400);
    img.save("output.png").unwrap();
    std::fs::write("output.txt", formula).unwrap();
}
