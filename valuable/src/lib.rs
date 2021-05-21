#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
//! Valuable provides object-safe value inspection. Use cases include passing
//! structured data to trait objects and object-safe serialization.
//!
//! # Getting started
//!
//! TODO
//!
//! # Design
//!
//! TODO

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod enumerable;
pub use enumerable::{EnumDef, Enumerable, Variant, VariantDef};

mod field;
pub use field::{Fields, NamedField};

mod listable;
pub use listable::Listable;

mod mappable;
pub use mappable::Mappable;

mod named_values;
pub use named_values::NamedValues;

mod slice;
pub use slice::Slice;

mod structable;
pub use structable::{StructDef, Structable};

mod valuable;
pub use crate::valuable::Valuable;

mod value;
pub use value::Value;

mod visit;
pub use visit::{visit, Visit};

#[cfg(feature = "derive")]
pub use valuable_derive::*;
