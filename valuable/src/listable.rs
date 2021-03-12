use crate::{Valuable, Value};

pub trait Listable {
    fn len(&self) -> usize;

    fn iter(&self, f: &mut dyn FnMut(&mut dyn Iterator<Item = Value<'_>>));
}

impl<T: Valuable> Listable for [T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn iter(&self, f: &mut dyn FnMut(&mut dyn Iterator<Item = Value<'_>>)) {
        f(&mut <[T]>::iter(self).map(Valuable::as_value));
    }
}

impl<T: Valuable> Listable for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn iter(&self, f: &mut dyn FnMut(&mut dyn Iterator<Item = Value<'_>>)) {
        f(&mut <[T]>::iter(self).map(Valuable::as_value));
    }
}