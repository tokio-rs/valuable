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
