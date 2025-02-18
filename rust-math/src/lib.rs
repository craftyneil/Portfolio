#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn stringify_sequence_closure(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::ExprClosure = syn::parse(item).unwrap();
    //let all_attr = attr.to_string().split_whitespace().collect::<Vec<&str>>()[0];
    println!("{:#?}", input);
    TokenStream::new()
}

// #[macro_export]
// macro_rules! stringify_sequence_closure {
//     ($closure:expr) => {
//         println!("{}", $closure.to_string());
//     };
// }
