use crate::{AsValue, Value};

pub trait Listable /* : AsValue */ {
    fn len(&self) -> usize;

    fn get(&self, index: usize) -> Option<Value<'_>>;
}

impl<T: AsValue> Listable for [T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn get(&self, index: usize) -> Option<Value<'_>> {
        <[T]>::get(self, index).map(AsValue::as_value)
    }
}

impl<T: AsValue> Listable for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn get(&self, index: usize) -> Option<Value<'_>> {
        <[T]>::get(self, index).map(AsValue::as_value)
    }
}