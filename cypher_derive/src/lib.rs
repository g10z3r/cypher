mod core;
mod cypque;

#[macro_use]
mod fragment;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(CypQue, attributes(cypher))]
pub fn derive_cypue(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    cypque::expand_derive_cypque(&mut input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
