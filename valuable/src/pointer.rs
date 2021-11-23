//! References to fields of [`Valuable`] values.
//!
//! A [`Pointer`] stores a path traversal to a particular value in a nested
//! [`Valuable`] structure. For example, a [`Pointer`] might refer to the field `z`
//! of the field `y` of the value `x`. [`Pointer`] paths can also include indices into
//! [`Listable`] and [`Tupleable`] values, in order to represent expressions like
//! `x.y[3].0`.
//!
//! # Examples
//!
//! ```rust
//! use valuable::*;
//!
//! #[derive(Valuable)]
//! struct Struct1 {
//!     x: String,
//!     y: Struct2,
//! }
//!
//! #[derive(Valuable)]
//! struct Struct2 {
//!     z: String,
//! }
//!
//! struct Visitor;
//!
//! impl Visit for Visitor {
//!     fn visit_value(&mut self, value: Value<'_>) {
//!         println!("{:?}", value);
//!     }
//! }
//!
//! let value = Struct1 {
//!     x: "a".to_owned(),
//!     y: Struct2 {
//!         z: "b".to_owned(),
//!     },
//! };
//!
//! let mut visitor = Visitor;
//!
//! visit_pointer!(value.x, visitor);   // "a"
//! visit_pointer!(value.y, visitor);   // Struct2 { field: "b" }
//! visit_pointer!(value.y.z, visitor); // "b"
//! ```

use crate::{NamedValues, Slice, Valuable, Value, Visit};

/// A pointer to the value.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Pointer<'a> {
    path: &'a [Segment<'a>],
}

impl<'a> Pointer<'a> {
    /// Creates a new `Pointer`.
    pub const fn new(path: &'a [Segment<'a>]) -> Self {
        Self { path }
    }

    /// Returns a path pointed by this pointer.
    ///
    /// If this pointer points to the current value, this method returns an empty slice.
    pub fn path(self) -> &'a [Segment<'a>] {
        self.path
    }

    #[doc(hidden)]
    #[must_use]
    pub fn step(self) -> Self {
        Self::new(&self.path[1..])
    }
}

impl<'a> From<&'a [Segment<'a>]> for Pointer<'a> {
    fn from(path: &'a [Segment<'a>]) -> Self {
        Self::new(path)
    }
}

/// A segment of a path.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Segment<'a> {
    /// Access of a named struct field.
    Field(&'a str),
    /// Access of an unnamed struct or a tuple field.
    TupleIndex(usize),
    /// Indexing of a list or a map.
    Index(Value<'a>),
}

pub(crate) fn visit_pointer<V>(value: &V, pointer: Pointer<'_>, visit: &mut dyn Visit)
where
    V: ?Sized + Valuable,
{
    let value = value.as_value();
    if pointer.path.is_empty() {
        visit.visit_value(value);
        return;
    }

    let visitor = &mut Visitor {
        pointer,
        visit,
        index: 0,
    };
    match (value, pointer.path[0]) {
        (Value::Listable(l), Segment::Index(..)) => {
            l.visit(visitor);
        }
        (Value::Mappable(m), Segment::Index(..)) => {
            m.visit(visitor);
        }
        (Value::Tuplable(t), Segment::TupleIndex(..)) => {
            t.visit(visitor);
        }
        (Value::Structable(s), Segment::TupleIndex(..)) if s.definition().fields().is_unnamed() => {
            s.visit(visitor);
        }
        (Value::Structable(s), Segment::Field(..)) if s.definition().fields().is_named() => {
            s.visit(visitor);
        }
        (_, p) => {
            panic!("invalid pointer: {:?},", p)
        }
    }
}

struct Visitor<'a> {
    pointer: Pointer<'a>,
    visit: &'a mut dyn Visit,
    index: usize,
}

impl Visit for Visitor<'_> {
    fn visit_value(&mut self, value: Value<'_>) {
        if let Segment::Index(index) = self.pointer.path[0] {
            if index.as_usize() == Some(self.index) {
                value.visit_pointer(self.pointer.step(), self.visit);
            }
        }
        self.index += 1;
    }

    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        if let Segment::Field(name) = self.pointer.path[0] {
            if let Some(value) = named_values.get_by_name(name) {
                value.visit_pointer(self.pointer.step(), self.visit);
            }
        }
    }

    fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
        if let Segment::TupleIndex(index) = self.pointer.path[0] {
            if let Some(value) = values.get(index - self.index) {
                value.visit_pointer(self.pointer.step(), self.visit);
            }
        }
    }

    fn visit_primitive_slice(&mut self, slice: Slice<'_>) {
        if let Segment::Index(index) = self.pointer.path[0] {
            if let Some(index) = index.as_usize() {
                if let Some(value) = slice.get(index - self.index) {
                    value.visit_pointer(self.pointer.step(), self.visit);
                }
            }
        }
        self.index += slice.len();
    }

    fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
        if let Segment::Index(index) = self.pointer.path[0] {
            let matched = match key {
                Value::Bool(k) => index.as_bool() == Some(k),
                Value::Char(k) => index.as_char() == Some(k),
                Value::I8(k) => index.as_i8() == Some(k),
                Value::I16(k) => index.as_i16() == Some(k),
                Value::I32(k) => index.as_i32() == Some(k),
                Value::I64(k) => index.as_i64() == Some(k),
                Value::I128(k) => index.as_i128() == Some(k),
                Value::Isize(k) => index.as_isize() == Some(k),
                Value::U8(k) => index.as_u8() == Some(k),
                Value::U16(k) => index.as_u16() == Some(k),
                Value::U32(k) => index.as_u32() == Some(k),
                Value::U64(k) => index.as_u64() == Some(k),
                Value::U128(k) => index.as_u128() == Some(k),
                Value::Usize(k) => index.as_usize() == Some(k),
                Value::String(k) => index.as_str() == Some(k),
                #[cfg(feature = "std")]
                Value::Path(k) => index.as_path() == Some(k),
                // f32 and f64 are not included because they do not implement `Eq`.
                _ => false,
            };
            if matched {
                value.visit_pointer(self.pointer.step(), self.visit);
            }
        }
    }
}
