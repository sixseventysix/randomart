use randomart::{fnv1a, Grammar, PixelCoordinates, Colour, render_pixels};

fn main() {
    let string = "samarth kulkarni";
    let seed = fnv1a(string);
    let mut grammar = Grammar::new(seed, 3);

    let expression = grammar.random_expression();
    println!("Generated Expression:\n{}", expression);
    let rgb_function = |coords: PixelCoordinates| {
        let (r, g, b) = grammar.evaluate_expression(&expression, coords.x, coords.y);
        Colour { r, g, b }
    };
    let img = render_pixels(rgb_function);
    img.save("data/output.png").expect("failed to save the image");
}