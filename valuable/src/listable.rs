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

#[cfg(feature = "alloc")]
impl<T: Valuable> Valuable for alloc::rc::Rc<[T]> {
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        T::visit_slice(self, visit);
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable> Listable for alloc::rc::Rc<[T]> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable> Valuable for alloc::sync::Arc<[T]> {
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        T::visit_slice(self, visit);
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable> Listable for alloc::sync::Arc<[T]> {
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

#[cfg(feature = "alloc")]
impl<T: Valuable> Valuable for alloc::collections::VecDeque<T> {
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        for value in self.iter() {
            visit.visit_value(value.as_value());
        }
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable> Listable for alloc::collections::VecDeque<T> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable> Valuable for alloc::collections::LinkedList<T> {
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        for value in self.iter() {
            visit.visit_value(value.as_value());
        }
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable> Listable for alloc::collections::LinkedList<T> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable + Ord> Valuable for alloc::collections::BinaryHeap<T> {
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        for value in self.iter() {
            visit.visit_value(value.as_value());
        }
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable + Ord> Listable for alloc::collections::BinaryHeap<T> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable + Ord> Valuable for alloc::collections::BTreeSet<T> {
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        for value in self.iter() {
            visit.visit_value(value.as_value());
        }
    }
}

#[cfg(feature = "alloc")]
impl<T: Valuable + Ord> Listable for alloc::collections::BTreeSet<T> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

#[cfg(feature = "std")]
impl<T, H> Valuable for std::collections::HashSet<T, H>
where
    T: Valuable + Eq + std::hash::Hash,
    H: std::hash::BuildHasher,
{
    fn as_value(&self) -> Value<'_> {
        Value::Listable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        for value in self.iter() {
            visit.visit_value(value.as_value());
        }
    }
}

#[cfg(feature = "std")]
impl<T, H> Listable for std::collections::HashSet<T, H>
where
    T: Valuable + Eq + std::hash::Hash,
    H: std::hash::BuildHasher,
{
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
