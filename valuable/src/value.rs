use crate::{Listable, Structable};
use crate::structable;

use std::fmt;

#[non_exhaustive]
pub enum Value<'a> {
    String(&'a str),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Unit,
    // More here
    Listable(&'a dyn Listable),
    Structable(&'a dyn Structable),
}

pub trait AsValue {
    fn as_value(&self) -> Value<'_>;
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
            Listable(v) => unimplemented!(),
            Structable(v) => structable::debug(v, fmt),
        }
    }
}