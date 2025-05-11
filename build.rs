use std::{env, fs};
use std::path::Path;

use randomart_core::{utils::fnv1a, grammar::Grammar, grammar::Node};

fn main() {
    let string = env::var("RANDOMART_STRING").unwrap_or_else(|_| "spiderman 1".to_string());
    let depth: u32 = env::var("RANDOMART_DEPTH").ok().and_then(|s| s.parse().ok()).unwrap_or(40);

    let seed = fnv1a(&string);
    let mut grammar = Grammar::default(seed);
    let mut generated_node = grammar.gen_rule(0, depth).unwrap();
    generated_node.simplify_triple();

    let (r, g, b) = match *generated_node {
        Node::Triple(ref r, ref g, ref b) => (r, g, b),
        _ => panic!("Expected Triple node at root"),
    };

    let code = format!(
        r#"
        use randomart::math::*;
        use randomart_fn_gen_macro::generate_fn;

        pub fn r_fn() -> impl Fn(f32, f32) -> f32 {{
            generate_fn!({})
        }}
        pub fn g_fn() -> impl Fn(f32, f32) -> f32 {{
            generate_fn!({})
        }}
        pub fn b_fn() -> impl Fn(f32, f32) -> f32 {{
            generate_fn!({})
        }}
        "#,
        r.to_dsl_string(),
        g.to_dsl_string(),
        b.to_dsl_string()
    );

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("generated_rgb_fn.rs");
    fs::write(out_path, code).expect("Failed to write generated RGB file");
}