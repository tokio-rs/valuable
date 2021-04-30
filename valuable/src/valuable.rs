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

macro_rules! impl_valuable {
    ($variant:ident($ty:ty)) => {
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
    };
}

impl_valuable!(Bool(bool));
impl_valuable!(Char(char));
impl_valuable!(I8(i8));
impl_valuable!(I16(i16));
impl_valuable!(I32(i32));
impl_valuable!(I64(i64));
impl_valuable!(I128(i128));
impl_valuable!(Isize(isize));
impl_valuable!(U8(u8));
impl_valuable!(U16(u16));
impl_valuable!(U32(u32));
impl_valuable!(U64(u64));
impl_valuable!(U128(u128));
impl_valuable!(Usize(usize));

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
