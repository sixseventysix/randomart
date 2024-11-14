use randomart::{utils::{ fnv1a, render_pixels, LinearCongruentialGenerator, PixelCoordinates }, Grammar, GrammarBranches, GrammarBranch, Node};

fn main() {
    let string = "samarth kulkarni";
    let seed = fnv1a(string);
    let mut rng = LinearCongruentialGenerator::new(seed);
    let grammar = Grammar {
        items: vec![
            // E::= (C, C, C)
            GrammarBranches {
                items: vec![
                    GrammarBranch {
                        node: Box::new(Node::Triple(
                            Box::new(Node::Rule(1)), 
                            Box::new(Node::Rule(1)), 
                            Box::new(Node::Rule(1)), 
                        )),
                        probability: 1.0, 
                    },
                ],
            },
            // C::= A | Add(C, C) | Mult(C, C) | Sin(C) | Cos(C) | Exp(C) | Sqrt(C) | Div(C, C) | Mix(C, C, C, C)
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
            // A::= x | y | random number in [-1,1]
            GrammarBranches {
                items: vec![
                    GrammarBranch {
                        node: Box::new(Node::X), 
                        probability: 1.0 / 3.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Y), 
                        probability: 1.0 / 3.0,
                    },
                    GrammarBranch {
                        node: Box::new(Node::Random), 
                        probability: 1.0 / 3.0,
                    },
                ],
            },
        ],
    };
    
    let start_rule = 0;
    let depth: u32 = 40;
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