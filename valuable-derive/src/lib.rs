extern crate proc_macro;

mod expand;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Valuable, attributes(valuable))]
pub fn derive_valuable(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    /*
    ser::expand_derive_serialize(&mut input)
        .unwrap_or_else(to_compile_errors)
        .into()
    */
    expand::derive_valuable(&mut input).into()
}