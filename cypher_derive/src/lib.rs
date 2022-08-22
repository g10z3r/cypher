mod core;
mod cyp_que;

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
    cyp_que::expand_derive_cypque(&mut input).into()
}
