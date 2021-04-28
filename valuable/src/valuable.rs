use crate::{Value, Slice, Visit};

pub trait Valuable {
    fn as_value(&self) -> Value<'_>;

    fn visit_slice(slice: &[Self], visit: &mut dyn Visit)
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl<V: ?Sized + Valuable> Valuable for &V {
    fn as_value(&self) -> Value<'_> {
        V::as_value(*self)
    }
}

impl<V: ?Sized + Valuable> Valuable for Box<V> {
    fn as_value(&self) -> Value<'_> {
        V::as_value(&**self)
    }
}

macro_rules! impl_valuable {
    ($variant:ident($ty:ty)) => {
        impl Valuable for $ty {
            fn as_value(&self) -> Value<'_> {
                Value::$variant(*self)
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
}

impl Valuable for str {
    fn as_value(&self) -> Value<'_> {
        Value::String(self)
    }
}

impl Valuable for String {
    fn as_value(&self) -> Value<'_> {
        Value::String(self)
    }
}

// This is not `Valuable for [T]` because we cannot cast &[T] to &dyn Trait.
impl<T: Valuable> Valuable for &[T] {
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self)
    }
}

impl<T: Valuable> Valuable for Vec<T> {
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self)
    }
}
