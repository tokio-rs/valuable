use crate::*;

use std::fmt;

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum Value<'a> {
    Bool(bool),
    Char(char),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    String(&'a str),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Unit, // TODO: None?
    Error(&'a dyn std::error::Error),
    Listable(&'a dyn Listable),
    Mappable(&'a dyn Mappable),
    Structable(&'a dyn Structable),
}

impl Valuable for Value<'_> {
    fn as_value(&self) -> Value<'_> {
        self.clone()
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(self.clone());
    }

    fn visit_slice(slice: &[Self], visit: &mut dyn Visit)
    where
        Self: Sized,
    {
        visit.visit_slice(Slice::Value(slice));
    }
}

impl Default for Value<'_> {
    fn default() -> Self {
        Value::Unit
    }
}

impl fmt::Debug for Value<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;

        match *self {
            String(v) => v.fmt(fmt),
            U8(v) => v.fmt(fmt),
            U16(v) => v.fmt(fmt),
            U32(v) => v.fmt(fmt),
            U64(v) => v.fmt(fmt),
            U128(v) => v.fmt(fmt),
            Usize(v) => v.fmt(fmt),
            Unit => ().fmt(fmt),
            Error(v) => fmt::Debug::fmt(v, fmt),
            Listable(v) => v.fmt(fmt),
            Structable(v) => v.fmt(fmt),
            // Structable(v) => structable::debug(v, fmt),
            _ => unimplemented!(),
        }
    }
}
