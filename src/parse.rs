use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Lit, LitStr, Token, Type, Ident};
use syn::token::Comma;
use crate::argument::Argument;
use crate::conclusion::Conclusion;
use crate::premise::Premise;

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            panic!("At least a type must be specified for an empty hashmap");
        }
        let mut vars = Vec::<char>::new();

        //parse let
        input.parse::<Token![let]>()?;

        // parse all atomic variables
        while !input.is_empty() {

            if let Ok(var) = input.parse::<Ident>() {
                vars.push(var.to_string().pop().unwrap());
            } else {
                panic!("must be a var")
            };

            if input.peek(Token![;]) {
                break;
            } else {
                input.parse::<Comma>()?;
            }
        }
        input.parse::<Token![;]>()?;
        let mut premises = Vec::<Premise>::new();
        // parse all premises
        while !input.is_empty() {

            if let Ok(premise) = input.parse::<Ident>() {
                let mut premise_string = String::new();
                premise_string.push_str(&premise.to_string());
                if input.peek(Token![->]) {
                    let mut implication = input.parse::<Token![->]>()?;
                    premise_string.push_str(&implication.spans
                        .iter_mut()
                        .map(|e| {e.source_text().unwrap()})
                        .collect::<String>()
                    );

                    premise_string.push_str(&input.parse::<Ident>()?.to_string());
                }
                premises.push(Premise::from(premise_string));
            } else {
                panic!("must have at least one premise");
            };
            input.parse::<Token![;]>()?;
            if input.peek(Token![==]) {
                break;
            }
        }
        // parse conclusion line
        input.parse::<Token![==]>()?;

        let conclusion = Conclusion::from(input.parse::<Ident>()?.to_string());
        input.parse::<Token![;]>()?;


        Ok(Argument {
            vars,
            premises,
            conclusion,
        })
    }
}