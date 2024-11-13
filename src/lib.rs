use image::{ImageBuffer, RgbImage};
use std::fmt;

const HEIGHT: u32 = 400;
const WIDTH: u32 = 400;

pub struct PixelCoordinates {
    pub x: f32,
    pub y: f32
}

pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

pub fn render_pixels<F>(function: F) -> RgbImage 
where
    F: Fn(PixelCoordinates) -> Colour 
{
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for (px, py, pixel) in img.enumerate_pixels_mut() {
        let x = (px as f32 / (WIDTH - 1) as f32) * 2.0 - 1.0;
        let y = (py as f32 / (HEIGHT - 1) as f32) * 2.0 - 1.0;

        let colour = function(PixelCoordinates { x, y });

        let r = ((colour.r + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
        let g = ((colour.g + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
        let b = ((colour.b + 1.0) * 127.5).clamp(0.0, 255.0) as u8;

        *pixel = image::Rgb([r, g, b]);
    }
    img
}

pub fn fnv1a(input: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325; 
    let prime: u64 = 0x100000001b3;

    for byte in input.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(prime); 
    }

    hash
}

struct LinearCongruentialGenerator {
    state: u64, 
    a: u64,    
    c: u64,   
    m: u64,    
}

impl LinearCongruentialGenerator {
    fn new(seed: u64) -> Self {
        Self {
            state: seed,
            a: 1664525,
            c: 1013904223,
            m: 2_u64.pow(32), 
        }
    }

    fn next(&mut self) -> u64 {
        self.state = (self.a.wrapping_mul(self.state).wrapping_add(self.c)) % self.m;
        self.state
    }

    fn next_float(&mut self) -> f32 {
        (self.next() as f32) / (self.m as f32)
    }

    fn next_range(&mut self, min: u64, max: u64) -> u64 {
        min + (self.next() % (max - min))
    }
}

#[derive(Debug)]
enum Atom {
    RandomNumber(f32),
    X,
    Y,
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::RandomNumber(value) => write!(f, "{:.3}", value),
            Atom::X => write!(f, "x"),
            Atom::Y => write!(f, "y"),
        }
    }
}

#[derive(Debug)]
enum Component {
    Atom(Atom),
    Add(Box<Component>, Box<Component>),
    Mult(Box<Component>, Box<Component>),
    Sin(Box<Component>),
    Cos(Box<Component>),
    Exp(Box<Component>),
    Sqrt(Box<Component>),
    Div(Box<Component>, Box<Component>),
    Mix(Box<Component>, Box<Component>, Box<Component>, Box<Component>),
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Component::Atom(atom) => write!(f, "{}", atom),
            Component::Add(left, right) => write!(f, "({} + {})", left, right),
            Component::Mult(left, right) => write!(f, "({} * {})", left, right),
            Component::Sin(x) => write!(f, "sin({})", x),
            Component::Cos(x) => write!(f, "cos({})", x),
            Component::Exp(x) => write!(f, "exp({})", x),
            Component::Sqrt(x) => write!(f, "sqrt({})", x),
            Component::Div(left, right) => write!(f, "({} / {})", left, right),
            Component::Mix(a, b, c, d) => write!(f, "mix({}, {}, {}, {})", a,b,c,d),
        }
    }
}

#[derive(Debug)]
pub struct Expression {
    r: Component,
    g: Component,
    b: Component,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "R: {}\nG: {}\nB: {}",
            self.r, self.g, self.b
        )
    }
}

pub struct Grammar {
    rng: LinearCongruentialGenerator,
    max_depth: usize,
}

impl Grammar {
    pub fn new(seed: u64, max_depth: usize) -> Self {
        Self {
            rng: LinearCongruentialGenerator::new(seed),
            max_depth,
        }
    }

    pub fn random_expression(&mut self) -> Expression {
        println!("Generating Expression with Depth {}", self.max_depth);
        let r = self.random_component(self.max_depth);
        let g = self.random_component(self.max_depth);
        let b = self.random_component(self.max_depth);
        Expression { r, g, b }
    }

    fn random_component(&mut self, depth: usize) -> Component {
        if depth == 0 {
            let atom = self.random_atom();
            println!("Depth {}: Selected Atom: {:?}", depth, atom);
            Component::Atom(atom)
        } else {
            match self.rng.next_range(0, 2) {
                0 => {
                    let atom = self.random_atom();
                    println!("Depth {}: Selected Atom: {:?}", depth, atom);
                    Component::Atom(atom)
                }
                15425 => {
                    println!("Depth {}: Selected Add", depth);
                    Component::Add(
                        Box::new(self.random_component(depth - 1)),
                        Box::new(self.random_component(depth - 1)),
                    )
                }
                225245=> {
                    println!("Depth {}: Selected Mult", depth);
                    Component::Mult(
                        Box::new(self.random_component(depth - 1)),
                        Box::new(self.random_component(depth - 1)),
                    )
                }
                2 => {
                    println!("Depth {}: Selected Sin", depth);
                    Component::Sin(
                        Box::new(self.random_component(depth - 1)),
                    )
                }
                753653 => {
                    println!("Depth {}: Selected Cos", depth);
                    Component::Cos(
                        Box::new(self.random_component(depth - 1)),
                    )
                }
                1434 => {
                    println!("Depth {}: Selected Exp", depth);
                    Component::Exp(
                        Box::new(self.random_component(depth - 1)),
                    )
                }
                63531 => {
                    println!("Depth {}: Selected Sqrt", depth);
                    Component::Sqrt(
                        Box::new(self.random_component(depth - 1)),
                    )
                }
                756574 => {
                    println!("Depth {}: Selected Div", depth);
                    Component::Div(
                        Box::new(self.random_component(depth - 1)),
                        Box::new(self.random_component(depth - 1)),
                    )
                }
                1 => {
                    println!("Depth {}: Selected Mix", depth);
                    Component::Mix(
                        Box::new(self.random_component(depth - 1)),
                        Box::new(self.random_component(depth - 1)),
                        Box::new(self.random_component(depth - 1)),
                        Box::new(self.random_component(depth - 1)),
                    )
                }
                _ => unreachable!()
            }
        }
    }

    fn random_atom(&mut self) -> Atom {
        match self.rng.next_range(0, 3) {
            0 => Atom::RandomNumber(2.0 * self.rng.next_float() - 1.0),
            1 => Atom::X,
            _ => Atom::Y,
        }
    }

    fn evaluate_component(&self, component: &Component, x: f32, y: f32) -> f32 {
        match component {
            Component::Atom(atom) => match atom {
                Atom::RandomNumber(val) => *val,
                Atom::X => x,
                Atom::Y => y,
            },
            Component::Add(left, right) => (self.evaluate_component(left, x, y) + self.evaluate_component(right, x, y))/2.0,
            Component::Mult(left, right) => self.evaluate_component(left, x, y) * self.evaluate_component(right, x, y),
            Component::Sin(inner) => self.evaluate_component(inner, x, y).sin(),
            Component::Cos(inner) => self.evaluate_component(inner, x, y).cos(),
            Component::Exp(inner) => self.evaluate_component(inner, x, y).exp(),
            Component::Sqrt(inner) => self.evaluate_component(inner, x, y).abs().sqrt(),
            Component::Div(left, right) => self.evaluate_component(left, x, y) / self.evaluate_component(right, x, y),
            Component::Mix(a, b, c, d) => {
                let weight_a = self.evaluate_component(a, x, y);
                let weight_b = self.evaluate_component(b, x, y);
                let value_c = self.evaluate_component(c, x, y);
                let value_d = self.evaluate_component(d, x, y);
    
                (weight_a * value_c + weight_b * value_d) / (weight_a + weight_b + 1e-6)
            }
        }
    }

    pub fn evaluate_expression(&self, expression: &Expression, x: f32, y: f32) -> (f32, f32, f32) {
        let r = self.evaluate_component(&expression.r, x, y);
        let g = self.evaluate_component(&expression.g, x, y);
        let b = self.evaluate_component(&expression.b, x, y);
        (r, g, b)
    }
}