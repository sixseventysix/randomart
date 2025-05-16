use crate::node::Node;

pub(crate) struct TokenStream<'a> {
    tokens: Vec<&'a str>,
    index: usize,
}

impl<'a> TokenStream<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            tokens: input.split_whitespace().collect(),
            index: 0,
        }
    }

    pub(crate) fn next(&mut self) -> Option<&'a str> {
        let tok = self.tokens.get(self.index);
        if tok.is_some() {
            self.index += 1;
        }
        tok.copied()
    }

    pub(crate) fn expect(&mut self, context: &str) -> &'a str {
        self.next().unwrap_or_else(|| {
            panic!(
                "Unexpected end of input while parsing: expected {} at position {}",
                context, self.index
            );
        })
    }

    pub(crate) fn current_pos(&self) -> usize {
        self.index
    }
}

pub(crate) fn parse_expr<'a>(tokens: &mut TokenStream<'a>) -> Node {
    let token = tokens.expect("an expression");

    match token {
        "x" => Node::X,
        "y" => Node::Y,
        "const_" => {
            let token = tokens.expect("a float after const_");
            let val = match token {
                "inf" => f32::INFINITY,
                "-inf" => f32::NEG_INFINITY,
                _ => token.parse::<f32>().unwrap_or_else(|e| {
                    panic!(
                        "Invalid float '{}', error: {} (at token {})",
                        token,
                        e,
                        tokens.current_pos()
                    );
                }),
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
        other => panic!(
            "Unknown token '{}' at position {}",
            other,
            tokens.current_pos()
        ),
    }
}