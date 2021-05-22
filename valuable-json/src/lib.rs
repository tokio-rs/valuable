#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![cfg(feature = "std")]

//! JSON support for Valuable.
//!
//! # Examples
//!
//! ```
//! use valuable::Valuable;
//!
//! #[derive(Valuable)]
//! struct Point {
//!     x: i32,
//!     y: i32,
//! }
//!
//! let point = Point { x: 1, y: 2 };
//!
//! assert_eq!(
//!     valuable_json::to_string(&point),
//!     r#"{"x":1,"y":2}"#,
//! );
//! ```

use std::io::Write;

use valuable::*;

/// Serialize the given value as a string of JSON.
pub fn to_string(value: &impl Valuable) -> String {
    let mut out = Vec::with_capacity(128);
    let mut ser = Serializer::new(&mut out);
    ser.visit_value(value.as_value());
    String::from_utf8(out).unwrap()
}

/// Serialize the given value as a pretty-printed string of JSON.
pub fn to_string_pretty(value: &impl Valuable) -> String {
    let mut out = Vec::with_capacity(128);
    let mut ser = Serializer::new_pretty(&mut out);
    ser.visit_value(value.as_value());
    String::from_utf8(out).unwrap()
}

/// A JSON serializer.
#[derive(Debug)]
pub struct Serializer<W> {
    out: W,
    style: Option<Style>,
}

#[derive(Debug, Clone, Copy)]
struct Style {
    indent: usize,
    indent_size: usize,
}

impl<W: Write> Serializer<W> {
    /// Creates a new JSON serializer.
    pub fn new(out: W) -> Self {
        Self { out, style: None }
    }

    /// Creates a new JSON pretty print serializer.
    pub fn new_pretty(out: W) -> Self {
        Self {
            out,
            style: Some(Style {
                indent_size: 2,
                indent: 0,
            }),
        }
    }

    fn increment_ident(&mut self) {
        if let Some(style) = &mut self.style {
            style.indent += 1;
        }
    }

    fn decrement_ident(&mut self) {
        if let Some(style) = &mut self.style {
            style.indent -= 1;
        }
    }

    fn push_u8(&mut self, byte: u8) {
        self.push_bytes(&[byte]);
    }

    fn push_bytes(&mut self, bytes: &[u8]) {
        // TODO: remove unwrap
        self.out.write_all(bytes).unwrap();
    }

    fn push_indent(&mut self) {
        if let Some(style) = self.style {
            for _ in 0..style.indent {
                for _ in 0..style.indent_size {
                    self.push_u8(b' ');
                }
            }
        }
    }

    fn push_newline(&mut self) {
        if self.style.is_some() {
            self.push_u8(b'\n');
        }
    }

    fn push_space(&mut self) {
        if self.style.is_some() {
            self.push_u8(b' ');
        }
    }

    /// Starts serializing a JSON array.
    fn start_array(&mut self) {
        self.push_u8(b'[');
        self.push_newline();
        self.increment_ident();
    }

    /// Finishes serializing a JSON array.
    fn end_array(&mut self) {
        self.push_newline();
        self.decrement_ident();
        self.push_indent();
        self.push_u8(b']');
    }

    /// Starts serializing a JSON object.
    fn start_object(&mut self) {
        self.push_u8(b'{');
        self.push_newline();
        self.increment_ident();
    }

    /// Finishes serializing a JSON object.
    fn end_object(&mut self) {
        self.push_newline();
        self.decrement_ident();
        self.push_indent();
        self.push_u8(b'}');
    }

    // TODO: store is_field flag in serializer?
    fn visit_value_inner(&mut self, v: Value<'_>, is_field: bool) {
        macro_rules! visit_num {
            ($n:expr) => {
                if is_field {
                    self.push_u8(b'"');
                    // TODO: remove unwrap
                    write!(self.out, "{}", $n).unwrap();
                    self.push_u8(b'"');
                } else {
                    // TODO: remove unwrap
                    write!(self.out, "{}", $n).unwrap();
                }
            };
        }
        match v {
            Value::Listable(l) => {
                self.start_array();
                l.visit(&mut VisitStructure {
                    first: true,
                    inner: self,
                    kind: ValueKind::List,
                });
                self.end_array();
            }
            Value::Mappable(m) => {
                self.start_object();
                m.visit(&mut VisitStructure {
                    first: true,
                    inner: self,
                    kind: ValueKind::Map,
                });
                self.end_object();
            }
            Value::Structable(s) => {
                if s.definition().fields().is_named() {
                    self.start_object();
                    s.visit(&mut VisitStructure {
                        first: true,
                        inner: self,
                        kind: ValueKind::Named,
                    });
                    self.end_object();
                } else {
                    self.start_array();
                    s.visit(&mut VisitStructure {
                        first: true,
                        inner: self,
                        kind: ValueKind::Unnamed,
                    });
                    self.end_array();
                }
            }
            Value::Enumerable(e) => {
                if e.variant().is_named_fields() {
                    self.start_object();
                    e.visit(&mut VisitStructure {
                        first: true,
                        inner: self,
                        kind: ValueKind::Named,
                    });
                    self.end_object();
                } else {
                    self.start_array();
                    e.visit(&mut VisitStructure {
                        first: true,
                        inner: self,
                        kind: ValueKind::Unnamed,
                    });
                    self.end_array();
                }
            }
            Value::String(s) => {
                self.push_u8(b'"');
                // TODO: escape
                self.push_bytes(s.as_bytes());
                self.push_u8(b'"');
            }
            Value::Char(c) => {
                self.push_u8(b'"');
                // TODO: escape
                write!(self.out, "{}", c).unwrap();
                self.push_u8(b'"');
            }
            Value::Path(p) => {
                self.push_u8(b'"');
                // TODO: escape?
                write!(self.out, "{}", p.display()).unwrap();
                self.push_u8(b'"');
            }
            Value::Bool(b) => {
                // TODO: how to handle if it it field
                visit_num!(b);
            }
            Value::I8(n) => {
                visit_num!(n);
            }
            Value::I16(n) => {
                visit_num!(n);
            }
            Value::I32(n) => {
                visit_num!(n);
            }
            Value::I64(n) => {
                visit_num!(n);
            }
            Value::I128(n) => {
                visit_num!(n);
            }
            Value::Isize(n) => {
                visit_num!(n);
            }
            Value::U8(n) => {
                visit_num!(n);
            }
            Value::U16(n) => {
                visit_num!(n);
            }
            Value::U32(n) => {
                visit_num!(n);
            }
            Value::U64(n) => {
                visit_num!(n);
            }
            Value::U128(n) => {
                visit_num!(n);
            }
            Value::Usize(n) => {
                visit_num!(n);
            }
            Value::Unit => {
                assert!(!is_field);
                self.push_bytes(b"null");
            }
            _ => {}
        }
    }
}

impl<W: Write> Visit for Serializer<W> {
    fn visit_value(&mut self, value: Value<'_>) {
        self.visit_value_inner(value, false)
    }

    fn visit_named_fields(&mut self, _: &NamedValues<'_>) {
        unreachable!()
    }

    fn visit_unnamed_fields(&mut self, _: &[Value<'_>]) {
        unreachable!()
    }

    fn visit_entry(&mut self, _: Value<'_>, _: Value<'_>) {
        unreachable!()
    }
}

struct VisitStructure<'a, W> {
    first: bool,
    inner: &'a mut Serializer<W>,
    kind: ValueKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValueKind {
    // `Value::Mappable`.
    // Serialized as JSON object.
    Map,
    // `Value::Listable`.
    // Serialized as JSON array.
    List,
    // `Structable` or `Enumerable` with named fields.
    // Serialized as JSON object.
    Named,
    // `Structable` or `Enumerable` with unnamed fields.
    // Serialized as JSON array.
    Unnamed,
}

impl<W: Write> Visit for VisitStructure<'_, W> {
    fn visit_value(&mut self, value: Value<'_>) {
        assert_eq!(self.kind, ValueKind::List);
        if self.first {
            self.first = false;
        } else {
            self.inner.push_u8(b',');
            self.inner.push_newline();
        }
        self.inner.push_indent();
        self.inner.visit_value_inner(value, false);
    }

    fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
        assert_eq!(self.kind, ValueKind::Map);
        assert!(!matches!(
            key,
            Value::Listable(..)
                | Value::Mappable(..)
                | Value::Structable(..)
                | Value::Enumerable(..)
        ));
        if self.first {
            self.first = false;
        } else {
            self.inner.push_u8(b',');
            self.inner.push_newline();
        }
        self.inner.push_indent();
        self.inner.visit_value_inner(key, true);
        self.inner.push_u8(b':');
        self.inner.push_space();
        self.inner.visit_value_inner(value, false);
    }

    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        assert_eq!(self.kind, ValueKind::Named);
        for (f, &v) in named_values {
            if self.first {
                self.first = false;
            } else {
                self.inner.push_u8(b',');
                self.inner.push_newline();
            }
            self.inner.push_indent();
            self.inner.push_u8(b'"');
            self.inner.push_bytes(f.name().as_bytes());
            self.inner.push_u8(b'"');
            self.inner.push_u8(b':');
            self.inner.push_space();
            self.inner.visit_value_inner(v, false);
        }
    }

    fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
        assert_eq!(self.kind, ValueKind::Unnamed);
        for &v in values {
            if self.first {
                self.first = false;
            } else {
                self.inner.push_u8(b',');
                self.inner.push_newline();
            }
            self.inner.push_indent();
            self.inner.visit_value_inner(v, false);
        }
    }
}
