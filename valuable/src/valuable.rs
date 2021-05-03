use crate::{Listable, Slice, Value, Visit};

use core::fmt;

pub trait Valuable {
    fn as_value(&self) -> Value<'_>;

    fn visit(&self, visit: &mut dyn Visit);

    fn visit_slice(slice: &[Self], visit: &mut dyn Visit)
    where
        Self: Sized,
    {
        const N: usize = 8;

        let mut batch: [_; N] = Default::default();
        let mut curr = 0;

        for v in slice {
            if curr == N {
                visit.visit_slice(Slice::Value(&batch[..]));
                curr = 0;
            }

            batch[curr] = v.as_value();
            curr += 1;
        }

        if curr > 0 {
            visit.visit_slice(Slice::Value(&batch[..curr]));
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

impl<V: ?Sized + Valuable> Valuable for Box<V> {
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
                    visit.visit_slice(Slice::$variant(slice));
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
        visit.visit_slice(Slice::Unit(slice));
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
        visit.visit_slice(Slice::Str(slice));
    }
}

impl Valuable for String {
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
        visit.visit_slice(Slice::String(slice));
    }
}

impl<T: Valuable> Valuable for &'_ [T] {
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self as &dyn Listable)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        T::visit_slice(self, visit);
    }
}

impl<T: Valuable> Valuable for Vec<T> {
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        T::visit_slice(self, visit);
    }
}

impl fmt::Debug for dyn Valuable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.as_value();
        value.fmt(fmt)
    }
}
