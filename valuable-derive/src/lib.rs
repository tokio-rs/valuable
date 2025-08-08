#![warn(rust_2018_idioms, unreachable_pub)]

#[macro_use]
mod error;

mod attr;
mod expand;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Derive a `Valuable` implementation for a struct or enum.
///
/// # Attributes
///
/// ## `#[valuable(rename = "...")]`
///
/// Use the given name instead of its Rust name.
///
/// ## `#[valuable(transparent)]`
///
/// Delegate the trait implementation to the field.
///
/// This attribute can only be used on a struct that has a single field.
///
/// ## `#[valuable(skip)]`
///
/// Skip the field.
///
/// ## `#[valuable(with = "...")]`
///
/// Use a custom function to transform the field value. The function must take a reference
/// to the field type and return a type that implements `Valuable`.
/// The returned value will be converted to a `Value` using `as_value()`.
///
/// **Performance Note**: For minimal overhead, return primitive types (`u64`, `bool`, etc.)
/// which are cheap to copy, or reference from existing data.
/// String transformations will involve heap allocation. Example:
///
/// ```rust,ignore
/// // Cheap copy: return primitives (stack copy, no allocation)
/// fn get_len(vec: &Vec<u8>) -> usize { vec.len() }
/// fn get_active(user: &User) -> bool { user.is_active }
///
/// // Expensive: create new strings (heap allocation)
/// fn format_id(id: &u64) -> String { format!("ID-{}", id) }
/// ```
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
