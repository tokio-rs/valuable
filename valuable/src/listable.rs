use crate::{Valuable, Value, Visit};

pub trait Listable {
    fn size_hint(&self) -> (usize, Option<usize>);

    fn visit(&self, visitor: &mut dyn Visit);
}

impl<T: Valuable> Listable for [T] {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn visit(&self, visitor: &mut dyn Visit) {
        unimplemented!()
    }
}

impl<T: Valuable> Listable for Vec<T> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn visit(&self, visitor: &mut dyn Visit) {
        unimplemented!()
    }
}
