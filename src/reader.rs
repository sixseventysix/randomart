use crate::node::Node;

pub(crate) fn parse_expr(tokens: &mut impl Iterator<Item = String>) -> Node {
    let token = tokens.next().expect("Unexpected end");

    match token.as_str() {
        "x" => Node::X,
        "y" => Node::Y,
        "const_" => {
            let _open = tokens.next().expect("Expected '(' after const_");

            let token = tokens.next().expect("Expected number after const_(");
            let val = match token.as_str() {
                "inf" => f32::INFINITY,
                "-inf" => f32::NEG_INFINITY,
                _ => match token.parse::<f32>() {
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to parse float for const_: '{}', error: {}", token, e);
                        panic!();
                    }
                },
            };

            Node::Number(val)
        }
        "add" => Node::Add(
            Box::new(parse_expr(tokens)),
            Box::new(parse_expr(tokens)),
        ),
        "sin" => Node::Sin(Box::new(parse_expr(tokens))),
        "cos" => Node::Cos(Box::new(parse_expr(tokens))),
        "div" => Node::Div(
            Box::new(parse_expr(tokens)),
            Box::new(parse_expr(tokens)),
        ),
        "mult" => Node::Mult(
            Box::new(parse_expr(tokens)),
            Box::new(parse_expr(tokens)),
        ),
        "exp" => Node::Exp(Box::new(parse_expr(tokens))),
        "sqrt" => Node::Sqrt(Box::new(parse_expr(tokens))),
        "triple" => Node::Triple(
            Box::new(parse_expr(tokens)),
            Box::new(parse_expr(tokens)),
            Box::new(parse_expr(tokens)),
        ),
        "mixu" => Node::MixUnbounded(
            Box::new(parse_expr(tokens)),
            Box::new(parse_expr(tokens)),
            Box::new(parse_expr(tokens)),
            Box::new(parse_expr(tokens)),
        ),
        "(" | ")" => parse_expr(tokens),
        other => panic!("Unknown token: {}", other),
    }
}

pub(crate) fn tokenize(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}