use crate::{Slice, Value, Visit};

use core::fmt;
use core::num::Wrapping;

pub trait Valuable {
    fn as_value(&self) -> Value<'_>;

    fn visit(&self, visit: &mut dyn Visit);

    fn visit_slice(slice: &[Self], visit: &mut dyn Visit)
    where
        Self: Sized,
    {
        for item in slice {
            visit.visit_value(item.as_value());
        }
    }
}

impl<V: ?Sized + Valuable> Valuable for &V {
    fn as_value(&self) -> Value<'_> {
        (*self).as_value()
    }

    fn visit(&self, visit: &mut dyn Visit) {
        V::visit(*self, visit);
    }
}

#[cfg(feature = "alloc")]
impl<V: ?Sized + Valuable> Valuable for alloc::boxed::Box<V> {
    fn as_value(&self) -> Value<'_> {
        (&**self).as_value()
    }

    fn visit(&self, visit: &mut dyn Visit) {
        V::visit(&**self, visit);
    }
}

#[cfg(feature = "alloc")]
impl<V: ?Sized + Valuable> Valuable for alloc::rc::Rc<V> {
    fn as_value(&self) -> Value<'_> {
        (&**self).as_value()
    }

    fn visit(&self, visit: &mut dyn Visit) {
        V::visit(&**self, visit);
    }
}

#[cfg(not(valuable_no_atomic_cas))]
#[cfg(feature = "alloc")]
impl<V: ?Sized + Valuable> Valuable for alloc::sync::Arc<V> {
    fn as_value(&self) -> Value<'_> {
        (&**self).as_value()
    }

    fn visit(&self, visit: &mut dyn Visit) {
        V::visit(&**self, visit);
    }
}

macro_rules! valuable {
    (
        $(
            $variant:ident($ty:ty),
        )*
    ) => {
        $(
            impl Valuable for $ty {
                fn as_value(&self) -> Value<'_> {
                    Value::$variant(*self)
                }

                fn visit(&self, visit: &mut dyn Visit) {
                    visit.visit_value(self.as_value());
                }

                fn visit_slice(slice: &[Self], visit: &mut dyn Visit)
                where
                    Self: Sized,
                {
                    visit.visit_primitive_slice(Slice::$variant(slice));
                }
            }
        )*
    };
}

valuable! {
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
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
}

macro_rules! nonzero {
    (
        $(
            $variant:ident($ty:ident),
        )*
    ) => {
        $(
            impl Valuable for core::num::$ty {
                fn as_value(&self) -> Value<'_> {
                    Value::$variant(self.get())
                }

                fn visit(&self, visit: &mut dyn Visit) {
                    visit.visit_value(self.as_value());
                }
            }
        )*
    };
}

nonzero! {
    I8(NonZeroI8),
    I16(NonZeroI16),
    I32(NonZeroI32),
    I64(NonZeroI64),
    I128(NonZeroI128),
    Isize(NonZeroIsize),
    U8(NonZeroU8),
    U16(NonZeroU16),
    U32(NonZeroU32),
    U64(NonZeroU64),
    U128(NonZeroU128),
    Usize(NonZeroUsize),
}

impl<T: Valuable> Valuable for Wrapping<T> {
    fn as_value(&self) -> Value<'_> {
        self.0.as_value()
    }

    fn visit(&self, visit: &mut dyn Visit) {
        self.0.visit(visit);
    }
}

impl Valuable for () {
    fn as_value(&self) -> Value<'_> {
        Value::Unit
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(Value::Unit);
    }

    fn visit_slice(slice: &[Self], visit: &mut dyn Visit)
    where
        Self: Sized,
    {
        visit.visit_primitive_slice(Slice::Unit(slice));
    }
}

impl<T: Valuable> Valuable for Option<T> {
    fn as_value(&self) -> Value<'_> {
        match self {
            Some(v) => v.as_value(),
            None => Value::Unit,
        }
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(self.as_value());
    }
}

impl Valuable for &'_ str {
    fn as_value(&self) -> Value<'_> {
        Value::String(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(Value::String(self));
    }

    fn visit_slice(slice: &[Self], visit: &mut dyn Visit)
    where
        Self: Sized,
    {
        visit.visit_primitive_slice(Slice::Str(slice));
    }
}

#[cfg(feature = "alloc")]
impl Valuable for alloc::string::String {
    fn as_value(&self) -> Value<'_> {
        Value::String(&self[..])
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(Value::String(self));
    }

    fn visit_slice(slice: &[Self], visit: &mut dyn Visit)
    where
        Self: Sized,
    {
        visit.visit_primitive_slice(Slice::String(slice));
    }
}

#[cfg(feature = "std")]
impl Valuable for std::path::Path {
    fn as_value(&self) -> Value<'_> {
        struct Error;
        static ERROR: Error = Error;
        const MSG: &str = "path is not valid UTF-8 string";
        impl fmt::Debug for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(&MSG, f)
            }
        }
        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&MSG, f)
            }
        }
        impl std::error::Error for Error {}

        match self.to_str() {
            Some(s) => Value::String(s),
            None => Value::Error(&ERROR),
        }
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(self.as_value());
    }
}

#[cfg(feature = "std")]
impl Valuable for std::path::PathBuf {
    fn as_value(&self) -> Value<'_> {
        self.as_path().as_value()
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(self.as_value());
    }
}

#[cfg(feature = "std")]
impl Valuable for dyn std::error::Error + '_ {
    fn as_value(&self) -> Value<'_> {
        Value::Error(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(self.as_value());
    }
}

impl fmt::Debug for dyn Valuable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.as_value();
        value.fmt(fmt)
    }
}
