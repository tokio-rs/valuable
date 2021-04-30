use crate::*;

use core::fmt;

#[non_exhaustive]
pub enum Slice<'a> {
    Bool(&'a [bool]),
    Char(&'a [char]),
    F32(&'a [f32]),
    F64(&'a [f64]),
    I8(&'a [i8]),
    I16(&'a [i16]),
    I32(&'a [i32]),
    I64(&'a [i64]),
    I128(&'a [i128]),
    Isize(&'a [isize]),
    // TODO: Should we keep separate string and str?
    Str(&'a [&'a str]),
    String(&'a [String]),
    U8(&'a [u8]),
    U16(&'a [u16]),
    U32(&'a [u32]),
    U64(&'a [u64]),
    U128(&'a [u128]),
    Usize(&'a [usize]),
    Value(&'a [Value<'a>]),
    Unit(&'a [()]),
    /*
    Error(&'a dyn std::error::Error),
    Listable(&'a dyn Listable),
    Structable(&'a dyn Structable),
    */
}

impl fmt::Debug for Slice<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Slice::*;

        let mut d = fmt.debug_list();

        match *self {
            Bool(v) => d.entries(v),
            Char(v) => d.entries(v),
            F32(v) => d.entries(v),
            F64(v) => d.entries(v),
            I8(v) => d.entries(v),
            I16(v) => d.entries(v),
            I32(v) => d.entries(v),
            I64(v) => d.entries(v),
            I128(v) => d.entries(v),
            Isize(v) => d.entries(v),
            Str(v) => d.entries(v),
            String(v) => d.entries(v),
            U8(v) => d.entries(v),
            U16(v) => d.entries(v),
            U32(v) => d.entries(v),
            U64(v) => d.entries(v),
            U128(v) => d.entries(v),
            Usize(v) => d.entries(v),
            Value(v) => d.entries(v),
            Unit(v) => d.entries(v),
        };

        d.finish()
    }
}
