use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
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
    let get_branches: TokenStream;
    let mut named_fields_statics = None;

    match &data.fields {
        syn::Fields::Named(_) => {
            // <struct>_FIELDS
            let named_fields_static_name = format_ident!("{}_FIELDS", input.ident);
            named_fields_statics =
                Some(named_fields_static(&named_fields_static_name, &data.fields));

            struct_def = quote! {
                ::valuable::StructDef::new_static(
                    #name_literal,
                    ::valuable::Fields::Named(#named_fields_static_name),
                )
            };

            get_branches =
                data.fields.iter().map(|field| {
                    let field_name_ident = field.ident.as_ref();
                    let field_name_str = field_name_ident.unwrap().to_string();
                    quote! {
                        ::valuable::Field::Named(field) if field.name() == #field_name_str => Some(::valuable::Valuable::as_value(&self.#field_name_ident)),
                    }
                }).collect();

            quote! {
                ::valuable::Field::Named(field) if field.name() == ""
            };

            let fields = data.fields.iter().map(|field| {
                let f = field.ident.as_ref();
                let tokens = quote! {
                    &self.#f
                };
                respan(tokens, &field.ty)
            });
            visit_fields = quote! {
                visitor.visit_named_fields(&::valuable::NamedValues::new(
                    #named_fields_static_name,
                    &[
                        #(::valuable::Valuable::as_value(#fields),)*
                    ],
                ));
            }
        }
        syn::Fields::Unnamed(_) | syn::Fields::Unit => {
            let len = data.fields.len();
            struct_def = quote! {
                ::valuable::StructDef::new_static(
                    #name_literal,
                    ::valuable::Fields::Unnamed(#len),
                )
            };

            get_branches =
                data.fields.iter().enumerate().map(|(i, _)| {
                    let i = syn::Index::from(i);
                    quote! {
                        ::valuable::Field::Unnamed(f) if f == #i => Some(::valuable::Valuable::as_value(&self.#i)),
                    }
                }).collect();

            let indices = data.fields.iter().enumerate().map(|(i, field)| {
                let index = syn::Index::from(i);
                let tokens = quote! {
                    &self.#index
                };
                respan(tokens, &field.ty)
            });
            visit_fields = quote! {
                visitor.visit_unnamed_fields(
                    &[
                        #(::valuable::Valuable::as_value(#indices),)*
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

            fn get(&self, field: ::valuable::Field<'_>) -> Option<::valuable::Value<'_>> {
                match field {
                    #get_branches
                    _ => None,
                }
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
    let mut variant_fn = vec![];
    let mut visit_variants = vec![];

    for (i, variant) in data.variants.iter().enumerate() {
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
                    ::valuable::VariantDef::new(
                        #variant_name_literal,
                        ::valuable::Fields::Named(#named_fields_static_name),
                    ),
                });

                variant_fn.push(quote! {
                    Self::#variant_name { .. } => {
                        ::valuable::Variant::Static(&#variants_static_name[#i])
                    }
                });

                let fields: Vec<_> = variant
                    .fields
                    .iter()
                    .map(|field| field.ident.as_ref())
                    .collect();
                let as_value = variant.fields.iter().map(|field| {
                    let f = field.ident.as_ref();
                    let tokens = quote! {
                        // HACK(taiki-e): This `&` is not actually needed to calling as_value,
                        // but is needed to emulate multi-token span on stable Rust.
                        &#f
                    };
                    respan(tokens, &field.ty)
                });
                visit_variants.push(quote! {
                    Self::#variant_name { #(#fields),* } => {
                        visitor.visit_named_fields(
                            &::valuable::NamedValues::new(
                                #named_fields_static_name,
                                &[
                                    #(::valuable::Valuable::as_value(#as_value),)*
                                ],
                            ),
                        );
                    }
                });
            }
            syn::Fields::Unnamed(_) => {
                let variant_name = &variant.ident;
                let variant_name_literal = variant_name.to_string();
                let len = variant.fields.len();
                variant_defs.push(quote! {
                    ::valuable::VariantDef::new(
                        #variant_name_literal,
                        ::valuable::Fields::Unnamed(#len),
                    ),
                });

                variant_fn.push(quote! {
                    Self::#variant_name(..) => {
                        ::valuable::Variant::Static(&#variants_static_name[#i])
                    }
                });

                let bindings: Vec<_> = (0..variant.fields.len())
                    .map(|i| format_ident!("__binding_{}", i))
                    .collect();
                let as_value = bindings
                    .iter()
                    .zip(&variant.fields)
                    .map(|(binding, field)| {
                        let tokens = quote! {
                            // HACK(taiki-e): This `&` is not actually needed to calling as_value,
                            // but is needed to emulate multi-token span on stable Rust.
                            &#binding
                        };
                        respan(tokens, &field.ty)
                    });
                visit_variants.push(quote! {
                    Self::#variant_name(#(#bindings),*) => {
                        visitor.visit_unnamed_fields(
                            &[
                                #(::valuable::Valuable::as_value(#as_value),)*
                            ],
                        );
                    }
                });
            }
            syn::Fields::Unit => {
                let variant_name = &variant.ident;
                let variant_name_literal = variant_name.to_string();
                variant_defs.push(quote! {
                    ::valuable::VariantDef::new(
                        #variant_name_literal,
                        ::valuable::Fields::Unnamed(0),
                    ),
                });

                variant_fn.push(quote! {
                    Self::#variant_name => {
                        ::valuable::Variant::Static(&#variants_static_name[#i])
                    }
                });

                visit_variants.push(quote! {
                    Self::#variant_name => {
                        visitor.visit_unnamed_fields(
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
                ::valuable::EnumDef::new_static(
                    #name_literal,
                    #variants_static_name,
                )
            }

            fn variant(&self) -> ::valuable::Variant<'_> {
                match self {
                    #(#variant_fn)*
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
            ::valuable::NamedField::new(#field_name_literal),
        }
    });
    quote! {
        static #name: &[::valuable::NamedField<'static>] = &[
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
        #[allow(clippy::indexing_slicing)]
    }
}

fn respan(tokens: TokenStream, span: &impl ToTokens) -> TokenStream {
    let mut iter = span.to_token_stream().into_iter();
    // `Span` on stable Rust has a limitation that only points to the first
    // token, not the whole tokens. We can work around this limitation by
    // using the first/last span of the tokens like `syn::Error::new_spanned` does.
    let start_span = iter.next().map_or_else(Span::call_site, |t| t.span());
    let end_span = iter.last().map_or(start_span, |t| t.span());

    let mut tokens = tokens.into_iter().collect::<Vec<_>>();
    if let Some(tt) = tokens.first_mut() {
        tt.set_span(start_span);
    }
    for tt in tokens.iter_mut().skip(1) {
        tt.set_span(end_span);
    }
    tokens.into_iter().collect()
}
