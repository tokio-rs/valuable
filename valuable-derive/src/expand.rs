use proc_macro2::TokenStream;

pub(crate) fn derive_valuable(input: &mut syn::DeriveInput) -> TokenStream {
    println!("{:#?}", input);
    unimplemented!()
}