use crate::{Valuable, Visit};

pub trait Listable {
    fn size_hint(&self) -> (usize, Option<usize>);

    fn visit(&self, visitor: &mut dyn Visit);
}

impl<T: Valuable> Listable for [T] {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn visit(&self, visitor: &mut dyn Visit) {
        const N: usize = 8;

        let mut batch: [_; N] = Default::default();
        let mut curr = 0;

        for v in self {
            if curr == N {
                visitor.visit_items(&batch[..]);
                curr = 0;
            }

            batch[curr] = v.as_value();
            curr += 1;
        }

        if curr > 0 {
            visitor.visit_items(&batch[..curr]);
        }
    }
}

impl<T: Valuable> Listable for Vec<T> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn visit(&self, visitor: &mut dyn Visit) {
        <[T]>::visit(&self, visitor)
    }
}
