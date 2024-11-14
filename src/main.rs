use randomart::{utils::{ fnv1a, render_pixels, LinearCongruentialGenerator, PixelCoordinates }, Grammar, GrammarBranches, GrammarBranch, Node};

fn main() {
    let string = "samarth";
    let seed = fnv1a(string);
    let mut rng = LinearCongruentialGenerator::new(seed);
    let grammar = Grammar {
        items: vec![
            // Rule E: (C, C, C)
            GrammarBranches {
                items: vec![
                    GrammarBranch {
                        node: Box::new(Node::Triple(
                            Box::new(Node::Rule(1)), // C (first element)
                            Box::new(Node::Rule(1)), // C (second element)
                            Box::new(Node::Rule(1)), // C (third element)
                        )),
                        probability: 1.0, // Only one option for E
                    },
                ],
            },
            // Rule C: A, Add(C, C), Mult(C, C), Sin(C), Cos(C), Exp(C), Sqrt(C), Div(C, C), Mix(C, C, C, C)
            GrammarBranches {
                items: vec![
                    GrammarBranch {
                        node: Box::new(Node::Rule(2)), // A
                        probability: 1.0 / 13.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Add(
                            Box::new(Node::Rule(1)),
                            Box::new(Node::Rule(1)),
                        )),
                        probability: 1.0 / 13.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Mult(
                            Box::new(Node::Rule(1)),
                            Box::new(Node::Rule(1)),
                        )),
                        probability: 1.0 / 13.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Sin(Box::new(Node::Rule(1)))),
                        probability: 3.0 / 13.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Cos(Box::new(Node::Rule(1)))),
                        probability: 3.0 / 13.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Exp(Box::new(Node::Rule(1)))),
                        probability: 1.0 / 13.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Sqrt(Box::new(Node::Rule(1)))),
                        probability: 1.0 / 13.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Div(
                            Box::new(Node::Rule(1)),
                            Box::new(Node::Rule(1)),
                        )),
                        probability: 1.0 / 13.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Mix(
                            Box::new(Node::Rule(1)),
                            Box::new(Node::Rule(1)),
                            Box::new(Node::Rule(1)),
                            Box::new(Node::Rule(1)),
                        )),
                        probability: 1.0 / 13.0,
                    },
                ],
            },
            // Rule A: x, y, or random number
            GrammarBranches {
                items: vec![
                    GrammarBranch {
                        node: Box::new(Node::X), // Variable x
                        probability: 1.0 / 3.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Y), // Variable y
                        probability: 1.0 / 3.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Random), // Random number
                        probability: 1.0 / 3.0,
                    },
                ],
            },
        ],
    };
    
    

    let start_rule = 0;
    let depth = 30;
    let generated_node = grammar.gen_rule(start_rule, depth, &mut rng).unwrap();
    println!("generated node: {:?}", generated_node);

    let rgb_function = |coords: PixelCoordinates| {
        generated_node.eval_rgb(coords.x, coords.y)
    };
    let img = render_pixels(rgb_function);

    let timestamp = "141120240104";
    let output_filepath = format!("data/images/{}.png", timestamp);
    img.save(output_filepath).expect("failed to save the image");
}