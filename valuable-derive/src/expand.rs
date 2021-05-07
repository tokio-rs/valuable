use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub(crate) fn derive_valuable(input: &mut syn::DeriveInput) -> TokenStream {
    match &input.data {
        syn::Data::Struct(data) => derive_struct(input, data),
        syn::Data::Enum(data) => derive_enum(input, data),
        // It's probably impossible to derive union because you cannot safely reference the field.
        // TODO: error instead of panic
        syn::Data::Union(..) => panic!("union is not supported"),
    }
}

fn derive_struct(input: &syn::DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let name = &input.ident;
    let name_literal = name.to_string();
    let visit_fields;
    let struct_def;
    let mut named_fields_statics = None;

    match &data.fields {
        syn::Fields::Named(_) => {
            // <struct>_FIELDS
            let named_fields_static_name = format_ident!("{}_FIELDS", input.ident);
            named_fields_statics =
                Some(named_fields_static(&named_fields_static_name, &data.fields));

            struct_def = quote! {
                ::valuable::StructDef::new(
                    #name_literal,
                    ::valuable::field::Fields::NamedStatic(#named_fields_static_name),
                    false,
                )
            };

            let fields = data.fields.iter().map(|field| field.ident.as_ref());
            visit_fields = quote! {
                visitor.visit_named_fields(&::valuable::NamedValues::new(
                    #named_fields_static_name,
                    &[
                        #(::valuable::Valuable::as_value(&self.#fields),)*
                    ],
                ));
            }
        }
        syn::Fields::Unnamed(_) | syn::Fields::Unit => {
            struct_def = quote! {
                ::valuable::StructDef::new(
                    #name_literal,
                    ::valuable::field::Fields::Unnamed,
                    false,
                )
            };

            let indices = (0..data.fields.len()).map(syn::Index::from);
            visit_fields = quote! {
                visitor.visit_unnamed_fields(
                    &[
                        #(::valuable::Valuable::as_value(&self.#indices),)*
                    ],
                );
            };
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let structable_impl = quote! {
        impl #impl_generics ::valuable::Structable for #name #ty_generics #where_clause {
            fn definition(&self) -> ::valuable::StructDef<'_> {
                #struct_def
            }
        }
    };

    let valuable_impl = quote! {
        impl #impl_generics ::valuable::Valuable for #name #ty_generics #where_clause {
            fn as_value(&self) -> ::valuable::Value<'_> {
                ::valuable::Value::Structable(self)
            }

            fn visit(&self, visitor: &mut dyn ::valuable::Visit) {
                #visit_fields
            }
        }
    };

    let allowed_lints = allowed_lints();
    quote! {
        #allowed_lints
        const _: () = {
            #named_fields_statics
            #structable_impl
            #valuable_impl
        };
    }
}

fn derive_enum(input: &syn::DeriveInput, data: &syn::DataEnum) -> TokenStream {
    // <enum>_VARIANTS
    let variants_static_name = format_ident!("{}_VARIANTS", input.ident);
    // `static FIELDS: &[NamedField<'static>]` for variant with named fields
    let mut named_fields_statics = vec![];
    let mut variant_defs = vec![];
    let mut visit_variants = vec![];

    for variant in &data.variants {
        match &variant.fields {
            syn::Fields::Named(_) => {
                // <enum>_<variant>_FIELDS
                let named_fields_static_name =
                    format_ident!("{}_{}_FIELDS", input.ident, variant.ident);
                named_fields_statics.push(named_fields_static(
                    &named_fields_static_name,
                    &variant.fields,
                ));

                let variant_name = &variant.ident;
                let variant_name_literal = variant_name.to_string();
                variant_defs.push(quote! {
                    ::valuable::VariantDef {
                        name: #variant_name_literal,
                        fields: ::valuable::field::Fields::NamedStatic(#named_fields_static_name),
                        is_dynamic: false,
                    },
                });

                let fields: Vec<_> = variant
                    .fields
                    .iter()
                    .map(|field| field.ident.as_ref())
                    .collect();
                visit_variants.push(quote! {
                    Self::#variant_name { #(#fields),* } => {
                        visitor.visit_variant_named_fields(
                            &::valuable::Variant { name: #variant_name_literal },
                            &::valuable::NamedValues::new(
                                #named_fields_static_name,
                                &[
                                    #(::valuable::Valuable::as_value(#fields),)*
                                ],
                            ),
                        );
                    }
                });
            }
            syn::Fields::Unnamed(_) => {
                let variant_name = &variant.ident;
                let variant_name_literal = variant_name.to_string();
                variant_defs.push(quote! {
                    ::valuable::VariantDef {
                        name: #variant_name_literal,
                        fields: ::valuable::field::Fields::Unnamed,
                        is_dynamic: false,
                    },
                });

                let bindings: Vec<_> = (0..variant.fields.len())
                    .map(|i| format_ident!("__binding_{}", i))
                    .collect();
                visit_variants.push(quote! {
                    Self::#variant_name(#(#bindings),*) => {
                        visitor.visit_variant_unnamed_fields(
                            &::valuable::Variant { name: #variant_name_literal },
                            &[
                                #(::valuable::Valuable::as_value(#bindings),)*
                            ],
                        );
                    }
                });
            }
            syn::Fields::Unit => {
                let variant_name = &variant.ident;
                let variant_name_literal = variant_name.to_string();
                variant_defs.push(quote! {
                    ::valuable::VariantDef {
                        name: #variant_name_literal,
                        fields: ::valuable::field::Fields::Unnamed,
                        is_dynamic: false,
                    },
                });

                visit_variants.push(quote! {
                    Self::#variant_name => {
                        visitor.visit_variant_unnamed_fields(
                            &::valuable::Variant { name: #variant_name_literal },
                            &[],
                        );
                    }
                });
            }
        }
    }
    let variants_static = quote! {
        static #variants_static_name: &[::valuable::VariantDef<'static>] = &[
            #(#variant_defs)*
        ];
    };

    let name = &input.ident;
    let name_literal = name.to_string();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let enumerable_impl = quote! {
        impl #impl_generics ::valuable::Enumerable for #name #ty_generics #where_clause {
            fn definition(&self) -> ::valuable::EnumDef<'_> {
                ::valuable::EnumDef {
                    name: #name_literal,
                    variants: #variants_static_name,
                    is_dynamic: false,
                }
            }
        }
    };

    let valuable_impl = quote! {
        impl #impl_generics ::valuable::Valuable for #name #ty_generics #where_clause {
            fn as_value(&self) -> ::valuable::Value<'_> {
                ::valuable::Value::Enumerable(self)
            }

            fn visit(&self, visitor: &mut dyn ::valuable::Visit) {
                match self {
                    #(#visit_variants)*
                }
            }
        }
    };

    let allowed_lints = allowed_lints();
    quote! {
        #allowed_lints
        const _: () = {
            #(#named_fields_statics)*
            #variants_static
            #enumerable_impl
            #valuable_impl
        };
    }
}

// `static <name>: &[NamedField<'static>] = &[ ... ];`
fn named_fields_static(name: &Ident, fields: &syn::Fields) -> TokenStream {
    debug_assert!(matches!(fields, syn::Fields::Named(..)));
    let named_fields = fields.iter().map(|field| {
        let field_name_literal = field.ident.as_ref().unwrap().to_string();
        quote! {
            ::valuable::field::NamedField::new(#field_name_literal),
        }
    });
    quote! {
        static #name: &[::valuable::field::NamedField<'static>] = &[
            #(#named_fields)*
        ];
    }
}

// Returns attributes that should be applied to generated code.
fn allowed_lints() -> TokenStream {
    quote! {
        #[allow(non_upper_case_globals)]
        #[allow(clippy::unknown_clippy_lints)]
        #[allow(clippy::used_underscore_binding)]
    }
}
