use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Lit, LitStr, Token, Type, Ident};
use crate::argument::Argument;

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            panic!("At least a type must be specified for an empty hashmap");
        }
        let mut vars = Vec::<char>::new();
        let declaration = input.parse::<Token![let]>()?;
        while !input.is_empty() {
            let var = if let Ok(var) = input.parse::<Ident>() {
                vars.push(var.to_string().pop().unwrap());
            } else {
                panic!("must be a var")
            };
            input.prase::<Token![,]>()?;
        }


    }
}