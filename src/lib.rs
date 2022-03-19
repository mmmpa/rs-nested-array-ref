use proc_macro::TokenStream;
use quote::quote;
use syn::__private::Span;
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{parse_macro_input, ExprArray, Ident, Lit, LitInt, LitStr, Type, TypeArray};

#[macro_use]
extern crate log;

#[proc_macro]
pub fn nested_ref(input: TokenStream) -> TokenStream {
    let ArrayInfo {
        init,
        x,
        y,
        raw_name,
        ref_name,
    } = parse_macro_input!(input as ArrayInfo);

    let y_value = y.base10_parse::<usize>().unwrap();

    let assign = (0..y_value)
        .into_iter()
        .map(|n| Ident::new(&format!("_array_{}", n), Span::call_site()))
        .collect::<Vec<_>>();

    let token = quote! {
        let mut #raw_name = [[#init; #x]; #y];
        let [#(ref mut #assign),*] = #raw_name;
        let #ref_name = &mut [#(#assign),*];

    };
    token.into()
}

struct ArrayInfo {
    init: Lit,
    y: LitInt,
    x: LitInt,
    raw_name: Ident,
    ref_name: Ident,
}

impl Parse for ArrayInfo {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let init: Lit = input.parse()?;
        input.parse::<Comma>()?;
        let x: LitInt = input.parse()?;
        input.parse::<Comma>()?;
        let y: LitInt = input.parse()?;
        input.parse::<Comma>()?;
        let raw_name: Ident = input.parse()?;
        input.parse::<Comma>()?;
        let ref_name: Ident = input.parse()?;

        Ok(Self {
            init,
            y,
            x,
            raw_name,
            ref_name,
        })
    }
}
