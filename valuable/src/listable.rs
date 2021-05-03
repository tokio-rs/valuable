use crate::*;

use core::fmt;

pub trait Listable: Valuable {
    fn size_hint(&self) -> (usize, Option<usize>);
}

impl<L: ?Sized + Listable> Listable for &L {
    fn size_hint(&self) -> (usize, Option<usize>) {
        L::size_hint(*self)
    }
}

impl<L: ?Sized + Listable> Listable for Box<L> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        L::size_hint(&**self)
    }
}

impl<T: Valuable> Listable for &'_ [T] {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<T: Valuable> Listable for Vec<T> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl fmt::Debug for dyn Listable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugListable<'a, 'b> {
            fmt: fmt::DebugList<'a, 'b>,
        }

        impl Visit for DebugListable<'_, '_> {
            fn visit_slice(&mut self, slice: Slice<'_>) {
                for value in &slice {
                    self.fmt.entry(&value);
                }
            }
        }

        let mut debug = DebugListable {
            fmt: fmt.debug_list(),
        };
        self.visit(&mut debug);
        debug.fmt.finish()
    }
}
