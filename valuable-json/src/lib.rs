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
//!     valuable_json::to_string(&point).unwrap(),
//!     r#"{"x":1,"y":2}"#,
//! );
//! ```

use std::io;

use valuable::*;

macro_rules! try_block {
    ($expr:expr) => {
        (|| -> io::Result<_> {
            $expr;
            Ok(())
        })()
    };
}

// TODO: should we define our own error type?
#[cold]
fn invalid_data(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg)
}

/// Serialize the given value JSON into the IO stream.
pub fn to_writer<W, V>(writer: W, value: &V) -> io::Result<()>
where
    W: io::Write,
    V: ?Sized + Valuable,
{
    Serializer::new(writer).serialize(value)
}

/// Serialize the given value as pretty-printed JSON into the IO stream.
pub fn to_writer_pretty<W, V>(writer: W, value: &V) -> io::Result<()>
where
    W: io::Write,
    V: ?Sized + Valuable,
{
    Serializer::pretty(writer).serialize(value)
}

/// Serialize the given value as a byte vector of JSON.
pub fn to_vec<V>(value: &V) -> io::Result<Vec<u8>>
where
    V: ?Sized + Valuable,
{
    let mut out = Vec::with_capacity(128);
    to_writer(&mut out, value)?;
    Ok(out)
}

/// Serialize the given value as a pretty-printed byte vector of JSON.
pub fn to_vec_pretty<V>(value: &V) -> io::Result<Vec<u8>>
where
    V: ?Sized + Valuable,
{
    let mut out = Vec::with_capacity(128);
    to_writer_pretty(&mut out, value)?;
    Ok(out)
}

/// Serialize the given value as a string of JSON.
pub fn to_string<V>(value: &V) -> io::Result<String>
where
    V: ?Sized + Valuable,
{
    let vec = to_vec(value)?;
    if cfg!(debug_assertions) {
        Ok(String::from_utf8(vec).unwrap())
    } else {
        // SAFETY: We do not emit invalid UTF-8.
        Ok(unsafe { String::from_utf8_unchecked(vec) })
    }
}

/// Serialize the given value as a pretty-printed string of JSON.
pub fn to_string_pretty<V>(value: &V) -> io::Result<String>
where
    V: ?Sized + Valuable,
{
    let vec = to_vec_pretty(value)?;
    if cfg!(debug_assertions) {
        Ok(String::from_utf8(vec).unwrap())
    } else {
        // SAFETY: We do not emit invalid UTF-8.
        Ok(unsafe { String::from_utf8_unchecked(vec) })
    }
}

/// A JSON serializer.
#[derive(Debug)]
pub struct Serializer<W> {
    out: W,
    option: SerializerOption,
    error: Option<io::Error>,
}

// TODO: Investigate what the better way to set these options is.
#[derive(Debug)]
struct SerializerOption {
    reject_nan: bool,
    escape_solidus: bool,
    style: Option<Style>,
}

#[derive(Debug, Clone, Copy)]
struct Style {
    current_indent: usize,
    indent_size: usize,
}

impl SerializerOption {
    fn new(style: Option<Style>) -> Self {
        Self {
            style,
            // Default behavior match serde_json.
            reject_nan: false,
            escape_solidus: false,
        }
    }
}

impl<W: io::Write> Serializer<W> {
    /// Creates a new JSON serializer.
    pub fn new(out: W) -> Self {
        Self {
            out,
            option: SerializerOption::new(None),
            error: None,
        }
    }

    /// Creates a new JSON pretty print serializer.
    pub fn pretty(out: W) -> Self {
        Self {
            out,
            option: SerializerOption::new(Some(Style {
                indent_size: 2,
                current_indent: 0,
            })),
            error: None,
        }
    }

    /// Return an error when encountered to NaN or infinity.
    /// By default, valuable-json, serializes NaN and infinity as null.
    pub fn reject_nan(&mut self) -> &mut Self {
        self.option.reject_nan = true;
        self
    }

    /// Escape solidus (aka slashes, `/`).
    /// By default, valuable-json serializes solidus without escape.
    pub fn escape_solidus(&mut self) -> &mut Self {
        self.option.escape_solidus = true;
        self
    }

    /// Serializes the given value as a JSON.
    pub fn serialize<V>(&mut self, value: &V) -> io::Result<()>
    where
        V: ?Sized + Valuable,
    {
        valuable::visit(value, self);
        if let Some(e) = self.error.take() {
            return Err(e);
        }
        Ok(())
    }

    fn increment_ident(&mut self) {
        if let Some(style) = &mut self.option.style {
            style.current_indent += 1;
        }
    }

    fn decrement_ident(&mut self) {
        if let Some(style) = &mut self.option.style {
            style.current_indent -= 1;
        }
    }

    fn push_indent(&mut self) -> io::Result<()> {
        if let Some(style) = self.option.style {
            for _ in 0..style.current_indent * style.indent_size {
                self.push_u8(b' ')?;
            }
        }
        Ok(())
    }

    fn push_newline(&mut self) -> io::Result<()> {
        if self.option.style.is_some() {
            self.push_u8(b'\n')?;
        }
        Ok(())
    }

    fn push_space(&mut self) -> io::Result<()> {
        if self.option.style.is_some() {
            self.push_u8(b' ')?;
        }
        Ok(())
    }

    fn push_u8(&mut self, byte: u8) -> io::Result<()> {
        self.push_bytes(&[byte])
    }

    fn push_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.out.write_all(bytes)
    }

    fn push_finite_float(&mut self, n: impl ryu::Float) -> io::Result<()> {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(n);
        self.push_bytes(s.as_bytes())
    }

    fn push_null(&mut self) -> io::Result<()> {
        self.push_bytes(b"null")
    }

    fn push_escaped_string(&mut self, s: &str) -> io::Result<()> {
        self.start_string()?;

        let bytes = s.as_bytes();

        let mut start = 0;

        for (i, &byte) in bytes.iter().enumerate() {
            let escape = escape(byte, self.option.escape_solidus);

            if matches!(escape, Escape::None) {
                continue;
            }

            if start < i {
                self.push_bytes(&bytes[start..i])?;
            }

            match escape {
                Escape::Char(bytes) => self.push_bytes(&bytes)?,
                Escape::Control(bytes) => self.push_bytes(&bytes)?,
                Escape::None => unreachable!(),
            }

            start = i + 1;
        }

        if start != bytes.len() {
            self.push_bytes(&bytes[start..])?;
        }

        self.end_string()
    }

    fn start_string(&mut self) -> io::Result<()> {
        self.push_u8(b'"')
    }

    fn end_string(&mut self) -> io::Result<()> {
        self.push_u8(b'"')
    }

    /// Starts serializing a JSON array.
    fn start_array(&mut self) -> io::Result<()> {
        self.push_u8(b'[')?;
        self.push_newline()?;
        self.increment_ident();
        Ok(())
    }

    /// Finishes serializing a JSON array.
    fn end_array(&mut self, is_empty: bool) -> io::Result<()> {
        if let Some(e) = self.error.take() {
            return Err(e);
        }
        if is_empty {
            self.push_u8(b'[')?;
        } else {
            self.push_newline()?;
            self.decrement_ident();
            self.push_indent()?;
        }
        self.push_u8(b']')
    }

    /// Starts serializing a JSON object.
    fn start_object(&mut self) -> io::Result<()> {
        self.push_u8(b'{')?;
        self.push_newline()?;
        self.increment_ident();
        Ok(())
    }

    /// Finishes serializing a JSON object.
    fn end_object(&mut self, is_empty: bool) -> io::Result<()> {
        if let Some(e) = self.error.take() {
            return Err(e);
        }
        if is_empty {
            self.push_u8(b'{')?;
        } else {
            self.push_newline()?;
            self.decrement_ident();
            self.push_indent()?;
        }
        self.push_u8(b'}')
    }

    fn visit_value_inner(&mut self, value: Value<'_>, is_field: bool) -> io::Result<()> {
        macro_rules! visit_num {
            ($n:expr) => {
                if is_field {
                    self.start_string()?;
                    write!(self.out, "{}", $n)?;
                    self.end_string()?;
                } else {
                    write!(self.out, "{}", $n)?;
                }
            };
        }
        match value {
            Value::Listable(l) => {
                if is_field {
                    return Err(invalid_data("list cannot be a key"));
                }
                let mut v = VisitStructure {
                    first: true,
                    inner: self,
                    kind: ValueKind::List,
                };
                l.visit(&mut v);
                v.inner.end_array(v.first)?;
            }
            Value::Mappable(m) => {
                if is_field {
                    return Err(invalid_data("map cannot be a key"));
                }
                let mut v = VisitStructure {
                    first: true,
                    inner: self,
                    kind: ValueKind::Map,
                };
                m.visit(&mut v);
                v.inner.end_object(v.first)?;
            }
            Value::Structable(s) => {
                if is_field {
                    return Err(invalid_data("struct cannot be a key"));
                }
                if s.definition().fields().is_named() {
                    let mut v = VisitStructure {
                        first: true,
                        inner: self,
                        kind: ValueKind::Named,
                    };
                    s.visit(&mut v);
                    v.inner.end_object(v.first)?;
                } else {
                    let mut v = VisitStructure {
                        first: true,
                        inner: self,
                        kind: ValueKind::Unnamed,
                    };
                    s.visit(&mut v);
                    v.inner.end_array(v.first)?;
                }
            }
            Value::Enumerable(e) => {
                if is_field {
                    return Err(invalid_data("enum cannot be a key"));
                }

                self.visit_key(&mut true, Value::String(e.variant().name()))?;

                if e.variant().is_named_fields() {
                    let mut v = VisitStructure {
                        first: true,
                        inner: self,
                        kind: ValueKind::Named,
                    };
                    e.visit(&mut v);
                    v.inner.end_object(v.first)?;
                } else {
                    let mut v = VisitStructure {
                        first: true,
                        inner: self,
                        kind: ValueKind::Unnamed,
                    };
                    e.visit(&mut v);
                    v.inner.end_array(v.first)?;
                }

                self.end_object(false)?;
            }
            Value::Tuplable(t) => {
                if is_field {
                    let msg = if t.definition().is_unit() {
                        "unit cannot be a key"
                    } else {
                        "tuple cannot be a key"
                    };
                    return Err(invalid_data(msg));
                }
                if t.definition().is_unit() {
                    self.push_null()?;
                } else {
                    let mut v = VisitStructure {
                        first: true,
                        inner: self,
                        kind: ValueKind::Unnamed,
                    };
                    t.visit(&mut v);
                    v.inner.end_array(v.first)?;
                }
            }
            Value::String(s) => {
                self.push_escaped_string(s)?;
            }
            Value::Char(c) => {
                self.push_escaped_string(&c.to_string())?;
            }
            Value::Path(p) => {
                self.push_escaped_string(&p.display().to_string())?;
            }
            Value::Bool(b) => {
                if is_field {
                    return Err(invalid_data("bool cannot be a key"));
                }
                write!(self.out, "{}", b)?;
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
            Value::F32(n) => {
                if is_field {
                    return Err(invalid_data("float cannot be a key"));
                }
                if n.is_finite() {
                    self.push_finite_float(n)?;
                } else if self.option.reject_nan {
                    let msg = if n.is_nan() {
                        "NaN cannot be a JSON value"
                    } else {
                        "infinity cannot be a JSON value"
                    };
                    return Err(invalid_data(msg));
                } else {
                    self.push_null()?;
                }
            }
            Value::F64(n) => {
                if is_field {
                    return Err(invalid_data("float cannot be a key"));
                }
                if n.is_finite() {
                    self.push_finite_float(n)?;
                } else if self.option.reject_nan {
                    let msg = if n.is_nan() {
                        "NaN cannot be a JSON value"
                    } else {
                        "infinity cannot be a JSON value"
                    };
                    return Err(invalid_data(msg));
                } else {
                    self.push_null()?;
                }
            }
            Value::Unit => {
                self.push_null()?;
            }
            v => {
                return Err(invalid_data(&format!("unsupported value kind: {:?}", v)));
            }
        }
        Ok(())
    }

    fn visit_key(&mut self, first: &mut bool, key: Value<'_>) -> io::Result<()> {
        if *first {
            self.start_object()?;
            *first = false;
        } else {
            self.push_u8(b',')?;
            self.push_newline()?;
        }
        self.push_indent()?;
        self.visit_value_inner(key, true)?;
        self.push_u8(b':')?;
        self.push_space()
    }
}

impl<W: io::Write> Visit for Serializer<W> {
    fn visit_value(&mut self, value: Value<'_>) {
        if self.error.is_some() {
            return;
        }
        if let Err(e) = self.visit_value_inner(value, false) {
            self.error = Some(e);
        }
    }

    fn visit_named_fields(&mut self, _: &NamedValues<'_>) {
        if self.error.is_none() {
            self.error = Some(invalid_data("value needs to call visit_value first"));
        }
    }

    fn visit_unnamed_fields(&mut self, _: &[Value<'_>]) {
        if self.error.is_none() {
            self.error = Some(invalid_data("value needs to call visit_value first"));
        }
    }

    fn visit_entry(&mut self, _: Value<'_>, _: Value<'_>) {
        if self.error.is_none() {
            self.error = Some(invalid_data("value needs to call visit_value first"));
        }
    }

    fn visit_primitive_slice(&mut self, _: Slice<'_>) {
        if self.error.is_none() {
            self.error = Some(invalid_data("value needs to call visit_value first"));
        }
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
    // `Structable` or `Enumerable` with unnamed fields, or `Tuplable`.
    // Serialized as JSON array.
    Unnamed,
}

impl ValueKind {
    #[cold]
    fn as_str(&self) -> &'static str {
        match self {
            Self::Map => "map",
            Self::List => "list",
            Self::Named => "named struct/variant",
            Self::Unnamed => "unnamed struct/variant",
        }
    }
}

impl<W: io::Write> Visit for VisitStructure<'_, W> {
    fn visit_value(&mut self, value: Value<'_>) {
        if self.inner.error.is_some() {
            return;
        }
        if self.kind != ValueKind::List {
            self.inner.error = Some(invalid_data(&format!(
                "visit_value in {}",
                self.kind.as_str()
            )));
            return;
        }
        if let Err(e) = try_block!({
            if self.first {
                self.inner.start_array()?;
                self.first = false;
            } else {
                self.inner.push_u8(b',')?;
                self.inner.push_newline()?;
            }
            self.inner.push_indent()?;
            self.inner.visit_value_inner(value, false)?;
        }) {
            self.inner.error = Some(e);
        }
    }

    fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
        if self.inner.error.is_some() {
            return;
        }
        if self.kind != ValueKind::Map {
            self.inner.error = Some(invalid_data(&format!(
                "visit_entry in {}",
                self.kind.as_str()
            )));
            return;
        }

        if let Err(e) = try_block!({
            self.inner.visit_key(&mut self.first, key)?;
            self.inner.visit_value_inner(value, false)?;
        }) {
            self.inner.error = Some(e);
        }
    }

    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        if self.inner.error.is_some() {
            return;
        }
        if self.kind != ValueKind::Named {
            self.inner.error = Some(invalid_data(&format!(
                "visit_named_fields in {}",
                self.kind.as_str()
            )));
            return;
        }
        if let Err(e) = try_block!({
            for (f, &v) in named_values {
                self.inner
                    .visit_key(&mut self.first, Value::String(f.name()))?;
                self.inner.visit_value_inner(v, false)?;
            }
        }) {
            self.inner.error = Some(e);
        }
    }

    fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
        if self.inner.error.is_some() {
            return;
        }
        if self.kind != ValueKind::Unnamed {
            self.inner.error = Some(invalid_data(&format!(
                "visit_unnamed_fields in {}",
                self.kind.as_str()
            )));
            return;
        }
        if let Err(e) = try_block!({
            for &v in values {
                if self.first {
                    self.inner.start_array()?;
                    self.first = false;
                } else {
                    self.inner.push_u8(b',')?;
                    self.inner.push_newline()?;
                }
                self.inner.push_indent()?;
                self.inner.visit_value_inner(v, false)?;
            }
        }) {
            self.inner.error = Some(e);
        }
    }
}

enum Escape {
    Char([u8; 2]),
    Control([u8; 6]),
    None,
}

#[inline]
fn escape(byte: u8, escape_solidus: bool) -> Escape {
    Escape::Char(*match byte {
        // quote (`"`)
        b'"' => b"\\\"",
        // reverse solidus (`\`)
        b'\\' => b"\\\\",
        // solidus (`/`)
        b'/' => {
            if !escape_solidus {
                return Escape::None;
            }
            b"\\/"
        }
        // line feed character (`\n`)
        b'\n' => b"\\n",
        // carriage return character (`\r`)
        b'\r' => b"\\r",
        // tab character (`\t`)
        b'\t' => b"\\t",
        // backspace character (`\b`)
        0x08 => b"\\b",
        // form feed character (`\f`)
        0x0C => b"\\f",
        // control character (`\u00XX`).
        // Based on https://github.com/serde-rs/json/blob/v1.0.64/src/ser.rs#L1790-L1801
        0x00..=0x1F => {
            static HEX: [u8; 16] = *b"0123456789abcdef";
            return Escape::Control([
                b'\\',
                b'u',
                b'0',
                b'0',
                HEX[(byte >> 4) as usize],
                HEX[(byte & 0xF) as usize],
            ]);
        }
        _ => return Escape::None,
    })
}
