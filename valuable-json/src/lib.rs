#![cfg(feature = "std")]
#![warn(rust_2018_idioms)]

use std::io::Write;

use valuable::*;

pub fn to_string(v: &dyn Valuable, pretty: bool) -> String {
    let mut out = Vec::with_capacity(128);
    let mut ser = if pretty {
        Serializer::new_pretty(&mut out)
    } else {
        Serializer::new(&mut out)
    };
    ser.visit_value(v.as_value());
    String::from_utf8(out).unwrap()
}

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
    pub fn new(out: W) -> Self {
        Self { out, style: None }
    }

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

    fn visit_map(&mut self, m: &dyn Mappable) {
        struct MapVisitor<'a, W> {
            first: bool,
            inner: &'a mut Serializer<W>,
        }
        impl<W: Write> Visit for MapVisitor<'_, W> {
            fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
                assert!(!matches!(
                    key,
                    Value::Listable(..) | Value::Mappable(..) | Value::Structable(..)
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

            fn visit_named_fields(&mut self, _: &NamedValues<'_>) {
                unreachable!()
            }

            fn visit_unnamed_fields(&mut self, _: &[Value<'_>]) {
                unreachable!()
            }

            fn visit_value(&mut self, _: Value<'_>) {
                unreachable!()
            }
        }

        self.push_u8(b'{');
        self.push_newline();
        self.increment_ident();
        m.visit(&mut MapVisitor {
            first: true,
            inner: self,
        });
        self.push_newline();
        self.decrement_ident();
        self.push_indent();
        self.push_u8(b'}');
    }

    fn visit_list(&mut self, l: &dyn Listable) {
        struct ListVisitor<'a, W> {
            first: bool,
            inner: &'a mut Serializer<W>,
        }
        impl<W: Write> Visit for ListVisitor<'_, W> {
            fn visit_value(&mut self, value: Value<'_>) {
                if self.first {
                    self.first = false;
                } else {
                    self.inner.push_u8(b',');
                    self.inner.push_newline();
                }
                self.inner.push_indent();
                self.inner.visit_value_inner(value, false);
            }

            fn visit_entry(&mut self, _: Value<'_>, _: Value<'_>) {
                unreachable!()
            }

            fn visit_named_fields(&mut self, _: &NamedValues<'_>) {
                unreachable!()
            }

            fn visit_unnamed_fields(&mut self, _: &[Value<'_>]) {
                unreachable!()
            }
        }

        self.push_u8(b'[');
        self.push_newline();
        self.increment_ident();
        l.visit(&mut ListVisitor {
            first: true,
            inner: self,
        });
        self.push_newline();
        self.decrement_ident();
        self.push_indent();
        self.push_u8(b']');
    }

    fn visit_field(&mut self, field: &NamedField<'_>, value: Value<'_>) {
        self.push_indent();
        self.push_u8(b'"');
        self.push_bytes(field.name().as_bytes());
        self.push_u8(b'"');
        self.push_u8(b':');
        self.push_space();

        self.visit_value_inner(value, false);
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
                self.visit_list(l);
            }
            Value::Mappable(m) => {
                self.visit_map(m);
            }
            Value::Structable(s) => {
                s.visit(self);
            }
            Value::Enumerable(e) => {
                e.visit(self);
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

    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        self.push_u8(b'{');
        self.push_newline();
        self.increment_ident();
        let mut first = true;
        for (f, v) in named_values {
            if first {
                first = false;
            } else {
                self.push_u8(b',');
                self.push_newline();
            }
            self.visit_field(f, *v);
        }
        self.push_newline();
        self.decrement_ident();
        self.push_indent();
        self.push_u8(b'}');
    }

    fn visit_unnamed_fields(&mut self, _: &[Value<'_>]) {
        unreachable!()
    }

    fn visit_entry(&mut self, _: Value<'_>, _: Value<'_>) {
        unreachable!()
    }
}
