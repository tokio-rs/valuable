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

macro_rules! slice {
    (
        $(
            $(#[$attrs:meta])*
            ($($generics:tt)*) $ty:ty,
        )*
    ) => {
        $(
            $(#[$attrs])*
            impl<$($generics)*> Valuable for $ty {
                fn as_value(&self) -> Value<'_> {
                    Value::Listable(self as &dyn Listable)
                }

                fn visit(&self, visit: &mut dyn Visit) {
                    T::visit_slice(self, visit);
                }
            }

            $(#[$attrs])*
            impl<$($generics)*> Listable for $ty {
                fn size_hint(&self) -> (usize, Option<usize>) {
                    (self.len(), Some(self.len()))
                }
            }
        )*
    };
}

slice! {
    (T: Valuable) &'_ [T],
    #[cfg(feature = "alloc")]
    (T: Valuable) alloc::boxed::Box<[T]>,
    #[cfg(feature = "alloc")]
    (T: Valuable) alloc::rc::Rc<[T]>,
    #[cfg(not(valuable_no_atomic_cas))]
    #[cfg(feature = "alloc")]
    (T: Valuable) alloc::sync::Arc<[T]>,
    (T: Valuable, const N: usize) [T; N],
    #[cfg(feature = "alloc")]
    (T: Valuable) alloc::vec::Vec<T>,
}

macro_rules! collection {
    (
        $(
            $(#[$attrs:meta])*
            ($($generics:tt)*) $ty:ty,
        )*
    ) => {
        $(
            $(#[$attrs])*
            impl<$($generics)*> Valuable for $ty {
                fn as_value(&self) -> Value<'_> {
                    Value::Listable(self as &dyn Listable)
                }

                fn visit(&self, visit: &mut dyn Visit) {
                    for value in self.iter() {
                        visit.visit_value(value.as_value());
                    }
                }
            }

            $(#[$attrs])*
            impl<$($generics)*> Listable for $ty {
                fn size_hint(&self) -> (usize, Option<usize>) {
                    (self.len(), Some(self.len()))
                }
            }
        )*
    };
}

collection! {
    #[cfg(feature = "alloc")]
    (T: Valuable) alloc::collections::VecDeque<T>,
    #[cfg(feature = "alloc")]
    (T: Valuable) alloc::collections::LinkedList<T>,
    #[cfg(feature = "alloc")]
    (T: Valuable + Ord) alloc::collections::BinaryHeap<T>,
    #[cfg(feature = "alloc")]
    (T: Valuable + Ord) alloc::collections::BTreeSet<T> ,
    #[cfg(feature = "std")]
    (T: Valuable + Eq + std::hash::Hash, H: std::hash::BuildHasher) std::collections::HashSet<T, H>,
}

impl fmt::Debug for dyn Listable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugListable<'a, 'b> {
            fmt: fmt::DebugList<'a, 'b>,
        }

        impl Visit for DebugListable<'_, '_> {
            fn visit_value(&mut self, value: Value<'_>) {
                self.fmt.entry(&value);
            }
        }

        let mut debug = DebugListable {
            fmt: fmt.debug_list(),
        };

        self.visit(&mut debug);
        debug.fmt.finish()
    }
}
