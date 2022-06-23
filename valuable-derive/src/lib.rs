#![warn(rust_2018_idioms, unreachable_pub)]

mod derive;
mod pointer;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error};

/// Derive a `Valuable` implementation for a struct or enum.
///
/// # Examples
///
/// ```
/// use valuable::Valuable;
///
/// #[derive(Valuable)]
/// struct HelloWorld {
///     message: Message,
/// }
///
/// #[derive(Valuable)]
/// enum Message {
///     HelloWorld,
///     Custom(String),
/// }
/// ```
#[proc_macro_derive(Valuable, attributes(valuable))]
pub fn derive_valuable(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as syn::DeriveInput);
    derive::derive_valuable(&mut input).into()
}

#[proc_macro]
pub fn visit_pointer(input: TokenStream) -> TokenStream {
    pointer::visit_pointer(input.into())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
