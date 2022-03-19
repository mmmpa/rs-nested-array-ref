use proc_macro::TokenStream;
use quote::quote;
use syn::__private::Span;
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{parse_macro_input, Ident, LitInt};

struct RefMutN {
    referred: Ident,
    n: usize,
}

impl Parse for RefMutN {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let referred: Ident = input.parse()?;
        input.parse::<Comma>()?;
        let n = input.parse::<LitInt>()?.base10_parse::<usize>()?;

        Ok(Self { referred, n })
    }
}

/// Defines a nested mutable reference.
///
/// # Example
///
/// ```
/// use ref_mut_n_lines::ref_mut_n_lines;
///
/// let mut data = [[0u8; 3]; 4];
/// let _ref_data: &mut [&mut [u8]] = ref_mut_n_lines!(data, 4);
/// ```
#[proc_macro]
pub fn ref_mut_n(input: TokenStream) -> TokenStream {
    let RefMutN { referred, n } = parse_macro_input!(input as RefMutN);

    let assign = (0..n)
        .into_iter()
        .map(|n| Ident::new(&format!("a{}", n), Span::call_site()))
        .collect::<Vec<_>>();

    let token = quote! {
        {
            let [#(ref mut #assign),*] = #referred;
            &mut [#(#assign),*]
        }
    };
    token.into()
}
