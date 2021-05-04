use crate::*;

use core::fmt;

macro_rules! value {
    (
        $(
            $variant:ident($ty:ty),
        )*
    ) => {
        #[non_exhaustive]
        #[derive(Clone, Copy)]
        pub enum Value<'a> {
            $(
                $variant($ty),
            )*
            Unit, // TODO: None?
        }

        $(
            impl<'a> From<$ty> for Value<'a> {
                fn from(src: $ty) -> Value<'a> {
                    Value::$variant(src)
                }
            }
        )*

        impl fmt::Debug for Value<'_> {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                use Value::*;

                match self {
                    $(
                        $variant(v) => fmt::Debug::fmt(v, fmt),
                    )*
                    Unit => ().fmt(fmt),
                }
            }
        }
    }
}

value! {
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
    Error(&'a dyn std::error::Error),
    Listable(&'a dyn Listable),
    Mappable(&'a dyn Mappable),
    Structable(&'a dyn Structable),
    Enumerable(&'a dyn Enumerable),
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

macro_rules! num_convert {
    (
        $(
            $ty:ty => ( $as:ident ),
        )*
    ) => {
        $(
            impl<'a> Value<'a> {
                pub fn $as(&self) -> Option<$ty> {
                    use Value::*;
                    use core::convert::TryInto;

                    match *self {
                        I8(v) => v.try_into().ok(),
                        I16(v) => v.try_into().ok(),
                        I32(v) => v.try_into().ok(),
                        I64(v) => v.try_into().ok(),
                        I128(v) => v.try_into().ok(),
                        Isize(v) => v.try_into().ok(),
                        U8(v) => v.try_into().ok(),
                        U16(v) => v.try_into().ok(),
                        U32(v) => v.try_into().ok(),
                        U64(v) => v.try_into().ok(),
                        U128(v) => v.try_into().ok(),
                        Usize(v) => v.try_into().ok(),
                        _ => None,
                    }
                }
            }
        )*
    }
}

num_convert! {
    i8 => (as_i8),
    i16 => (as_i16),
    i32 => (as_i32),
    i64 => (as_i64),
    i128 => (as_i128),
    isize => (as_isize),
    u8 => (as_u8),
    u16 => (as_u16),
    u32 => (as_u32),
    u64 => (as_u64),
    u128 => (as_u128),
    usize => (as_usize),
}
