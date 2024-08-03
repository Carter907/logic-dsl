
mod model;
mod parser;

use proc_macro::TokenStream;
use quote::quote;
use crate::model::argument::Argument;

#[proc_macro]
pub fn argument(_input: TokenStream) -> TokenStream {

    // check if it's empty
    if _input.is_empty() {
        panic!("At least a type must be specified for an empty hashmap");
    }
    let argument: Argument = _input
        .to_string()
        .parse::<Argument>()
        .unwrap_or(Argument::new());

    let is_valid = argument.is_valid_argument();


    quote!({println!("{}", #is_valid)}).into()
}
