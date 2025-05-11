use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

#[proc_macro]
pub fn generate_fn(input: TokenStream) -> TokenStream {
    let parsed_expr = parse_macro_input!(input as Expr);

    let output = quote! {
        |x: f32, y: f32| -> f32 {
            #parsed_expr
        }
    };

    output.into()
}