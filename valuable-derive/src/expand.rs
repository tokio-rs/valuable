use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) fn derive_valuable(input: &mut syn::DeriveInput) -> TokenStream {
    match &input.data {
        syn::Data::Struct(data) => derive_struct(input, data),
        // TODO: waiting Enumerable
        syn::Data::Enum(_data) => unimplemented!("enum is not supported yet"),
        // It's probably impossible to derive union because you cannot safely reference the field.
        syn::Data::Union(..) => panic!("union is not supported"),
    }
}

fn derive_struct(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    if !matches!(data.fields, syn::Fields::Named(..)) {
        unimplemented!("tuple struct and unit struct are not supported yet")
    }

    let static_fields_static_name = format_ident!("{}_FIELDS", input.ident);
    let static_fields = data.fields.iter().enumerate().map(|(i, f)| {
        let index = syn::Index::from(i);
        let name = f.ident.as_ref().unwrap().to_string();
        quote! {
            ::valuable::field::StaticField::new(#index, #name),
        }
    });
    let static_fields_static = quote! {
        static #static_fields_static_name: &[::valuable::field::StaticField] = &[
            #(#static_fields)*
        ];
    };

    let ident = &input.ident;
    let ident_str = ident.to_string();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let as_values = data.fields.iter().map(|f| {
        let ident = f.ident.as_ref();
        quote! {
            ::valuable::Valuable::as_value(&self.#ident),
        }
    });
    let structable_impl = quote! {
        impl #impl_generics ::valuable::Structable for #ident #ty_generics #where_clause {
            fn definition(&self) -> ::valuable::StructDef<'_> {
                ::valuable::StructDef {
                    name: #ident_str,
                    static_fields: #static_fields_static_name,
                    is_dynamic: false,
                }
            }

            fn visit(&self, v: &mut dyn ::valuable::Visit) {
                let definition = self.definition();
                v.visit_named_fields(&::valuable::NamedValues::new(
                    &definition,
                    &[
                        #(#as_values)*
                    ],
                ));
            }
        }
    };

    let valuable_impl = quote! {
        impl #impl_generics ::valuable::Valuable for #ident #ty_generics #where_clause {
            fn as_value(&self) -> ::valuable::Value<'_> {
                ::valuable::Value::Structable(self)
            }
        }
    };

    quote! {
        #[allow(non_upper_case_globals)]
        const _: () = {
            #static_fields_static
            #structable_impl
            #valuable_impl
        };
    }
}
