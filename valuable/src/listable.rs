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
        T::visit_slice(self, visitor);
        /*
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
        */
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
        struct DebugListable<'a, 'b> {
            fmt: &'a mut fmt::Formatter<'b>,
            res: fmt::Result,
        }

        impl Visit for DebugListable<'_, '_> {
            fn visit_slice(&mut self, slice: Slice<'_>) {
                use core::fmt::Debug;
                self.res = slice.fmt(self.fmt);
            }
        }

        let mut debug = DebugListable {
            fmt,
            res: Ok(()),
        };
        self.visit(&mut debug);
        debug.res
    }
}
