use randomart::{fnv1a, Grammar, PixelCoordinates, Colour, render_pixels, Expression};
use std::fs::File;
use std::io::Write;

struct Metadata {
    input_string: String,
    seed: u64,
    expression: Expression,
    depth: usize,
    hashing_function: String,
    prng_formula: String,
    grammar: String
}

impl Metadata {
    fn new(
        input_string: String,
        seed: u64,
        expression: Expression,
        depth: usize,
        hashing_function: String,
        prng_formula: String,
        grammar: String
    ) -> Self {
        Self {
            input_string,
            seed,
            expression,
            depth,
            hashing_function,
            prng_formula,
            grammar
        }
    }
}

fn save_metadata(
    filename: &str,
    metadata: Metadata
) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "Input String: {}", metadata.input_string)?;
    writeln!(file, "Seed: {}", metadata.seed)?;
    writeln!(file, "Generated Expression:")?;
    writeln!(file, "{}", metadata.expression)?; 
    writeln!(file, "Depth: {}", metadata.depth)?;
    writeln!(file, "Hashing Function: {}", metadata.hashing_function)?;
    writeln!(file, "PRNG Formula: {}", metadata.prng_formula)?;
    writeln!(file, "Grammar:{}", metadata.grammar)?;

    Ok(())
}

fn main() {
    let string = "diya";
    let max_depth = 12;
    let seed = fnv1a(string);
    let mut grammar = Grammar::new(seed, max_depth);

    let expression = grammar.random_expression();
    println!("Generated Expression:\n{}", expression);
    let rgb_function = |coords: PixelCoordinates| {
        let (r, g, b) = grammar.evaluate_expression(&expression, coords.x, coords.y);
        Colour { r, g, b }
    };
    let img = render_pixels(rgb_function);
    let timestamp = "141120240017";
    let output_filepath = format!("data/images/{}.png", timestamp);
    let metadata_filepath = format!("data/metadata/{}.metadata", timestamp);
    img.save(output_filepath).expect("failed to save the image");

    let fnv1a_code = "fnv1a(hash=0xcbf29ce484222325, prime=0x100000001b3)";
    let prng_code = "LCR(a=1664525, c=1013904223, m=2^32)";
    let grammar = r#"
E ::= (C,C,C); 1 probability
C -> 
    A
    | Add(C, C)
    | Mult(C, C)
    | Sin(C)
    | Cos(C)
    | Exp(C)
    | Sqrt(C)
    | Div(C, C)
    | Mix(C, C, C, C)
; 1/13 probability for A,Add,Mult,Exp,Sqrt,Div,Mix; 3/13 for Sin,Cos
A ::= x | y | random number in range [-1,1]; 1/3 probability each
"#;

    save_metadata(
        &metadata_filepath,
        Metadata::new(
            string.to_string(),
            seed,
            expression,
            max_depth, 
            fnv1a_code.to_string(),
            prng_code.to_string(),
            grammar.to_string()
        )
    )
    .expect("Failed to save metadata");
}