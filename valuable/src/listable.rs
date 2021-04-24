use crate::*;

use core::fmt;

pub trait Listable {
    fn size_hint(&self) -> (usize, Option<usize>);

    fn visit(&self, visitor: &mut dyn Visit);
}

impl<L: ?Sized + Listable> Listable for &L {
    fn size_hint(&self) -> (usize, Option<usize>) {
        L::size_hint(*self)
    }

    fn visit(&self, visitor: &mut dyn Visit) {
        L::visit(*self, visitor)
    }
}

impl<L: ?Sized + Listable> Listable for Box<L> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        L::size_hint(&**self)
    }

    fn visit(&self, visitor: &mut dyn Visit) {
        L::visit(&**self, visitor)
    }
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

impl fmt::Debug for dyn Listable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Debug<'a, 'b>(fmt::DebugList<'a, 'b>);

        impl Visit for Debug<'_, '_> {
            fn visit_item(&mut self, value: Value<'_>) {
                self.0.entry(&value);
            }
        }

        let mut debug = Debug(fmt.debug_list());
        self.visit(&mut debug);

        debug.0.finish()
    }
}
