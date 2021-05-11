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

#[cfg(feature = "alloc")]
impl<L: ?Sized + Listable> Listable for alloc::boxed::Box<L> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        L::size_hint(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<L: ?Sized + Listable> Listable for alloc::rc::Rc<L> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        L::size_hint(&**self)
    }
}

#[cfg(not(valuable_no_atomic_cas))]
#[cfg(feature = "alloc")]
impl<L: ?Sized + Listable> Listable for alloc::sync::Arc<L> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        L::size_hint(&**self)
    }
}

impl<T: Valuable> Listable for &'_ [T] {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable> Listable for alloc::boxed::Box<[T]> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<T: Valuable, const N: usize> Listable for [T; N] {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable> Listable for alloc::vec::Vec<T> {
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
