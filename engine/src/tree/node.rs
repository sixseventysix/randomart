#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Node {
    X,
    Y,
    Random,
    Rule(usize),
    Number(f32),
    Sqrt(Box<Node>),
    Sin(Box<Node>),
    Cos(Box<Node>),
    Exp(Box<Node>),
    Add(Box<Node>, Box<Node>),
    Mult(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Triple(Box<Node>, Box<Node>, Box<Node>),
    MixUnbounded(Box<Node>, Box<Node>, Box<Node>, Box<Node>),
}

impl Node {
    pub fn as_triple(&self) -> Option<(&Node, &Node, &Node)> {
        match self {
            Node::Triple(r, g, b) => Some((r, g, b)),
            _ => None,
        }
    }
}
