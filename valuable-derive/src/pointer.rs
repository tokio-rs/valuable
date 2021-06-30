use std::collections::VecDeque;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Result, Token};

use crate::derive::respan;

pub(crate) fn visit_pointer(input: TokenStream) -> Result<TokenStream> {
    let Input {
        expr,
        segments,
        visit,
    } = syn::parse2(input)?;

    let segments = segments.iter().map(|segment| match segment {
        Segment::Member(syn::Member::Named(ident)) => {
            let literal = ident.to_string();
            quote! {
                ::valuable::PointerSegment::Field(#literal),
            }
        }
        Segment::Member(syn::Member::Unnamed(index)) => {
            quote! {
                ::valuable::PointerSegment::TupleIndex(#index),
            }
        }
        Segment::Index(expr) => {
            let expr = respan(quote! { &#expr }, expr);
            quote! {
                ::valuable::PointerSegment::Index(
                    ::valuable::Valuable::as_value(#expr)
                ),
            }
        }
    });

    let visit_pointer = respan(quote! { ::valuable::Valuable::visit_pointer }, &expr);
    Ok(quote! {
        #visit_pointer(
            &#expr,
            ::valuable::Pointer::new(&[
                #(#segments)*
            ]),
            &mut #visit,
        )
    })
}

struct Input {
    expr: Expr,
    segments: VecDeque<Segment>,
    visit: Expr,
}

enum Segment {
    Member(syn::Member),
    Index(Box<Expr>),
}

impl Parse for Input {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut chain = input.parse()?;
        let _: Token![,] = input.parse()?;
        let visit = input.parse()?;
        let _: Option<Token![,]> = input.parse()?;

        let mut segments = VecDeque::new();
        let expr;
        loop {
            match chain {
                Expr::Field(e) => {
                    chain = *e.base;
                    segments.push_front(Segment::Member(e.member))
                }
                Expr::Index(e) => {
                    chain = *e.expr;
                    segments.push_front(Segment::Index(e.index))
                }
                e => {
                    expr = e;
                    break;
                }
            }
        }

        Ok(Self {
            expr,
            segments,
            visit,
        })
    }
}
