#![warn(rust_2018_idioms, unreachable_pub)]

mod attr;
mod expand;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Derive a `Valuable` implementation for a struct or enum.
///
/// # Container attributes
///
/// | Attribute                  | Description                   | Type     |
/// |----------------------------|-------------------------------|----------|
/// | `#[valuable(rename)]`      | Container name                | string   |
/// | `#[valuable(transparent)]` |                               | -        |
///
/// # Variant attributes
///
/// | Attribute                  | Description                   | Type     |
/// |----------------------------|-------------------------------|----------|
/// | `#[valuable(rename)]`      | Variant name                  | string   |
/// | `#[valuable(skip)]`        | Skip this variant             | -        |
///
/// # Field attributes
///
/// | Attribute                  | Description                   | Type     |
/// |----------------------------|-------------------------------|----------|
/// | `#[valuable(rename)]`      | Field name                    | string   |
/// | `#[valuable(skip)]`        | Skip this field               | -        |
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
    expand::derive_valuable(&mut input).into()
}
