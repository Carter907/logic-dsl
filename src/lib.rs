mod parse;
mod argument;
mod premise;
mod conclusion;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr};
use crate::argument::Argument;

#[proc_macro]
pub fn argument(_input: TokenStream) -> TokenStream {

    // check if it's empty
    if _input.is_empty() {
        panic!("At least a type must be specified for an empty hashmap");
    }

    // Parse the input tokens into a syntax tree
    let argument = parse_macro_input!(_input as Argument);
    let result = argument.is_valid_argument();

    // Generate the output hashmap inside a code block so that
    // we don't shadow any existing variables. Return the hashmap
    // from the block.
    quote!({

        println!("{}", #result)
    }).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // argument! {
        //     let P, Q;
        //     P -> Q;
        //     P;
        //     --
        //     Q;
        // }
    }
}
