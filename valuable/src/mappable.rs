use crate::*;

use core::fmt;

pub trait Mappable: Valuable {
    fn size_hint(&self) -> (usize, Option<usize>);
}

impl<M: ?Sized + Mappable> Mappable for &M {
    fn size_hint(&self) -> (usize, Option<usize>) {
        M::size_hint(*self)
    }
}

impl<M: ?Sized + Mappable> Mappable for &mut M {
    fn size_hint(&self) -> (usize, Option<usize>) {
        M::size_hint(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<M: ?Sized + Mappable> Mappable for alloc::boxed::Box<M> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        M::size_hint(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<M: ?Sized + Mappable> Mappable for alloc::rc::Rc<M> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        M::size_hint(&**self)
    }
}

#[cfg(not(valuable_no_atomic_cas))]
#[cfg(feature = "alloc")]
impl<M: ?Sized + Mappable> Mappable for alloc::sync::Arc<M> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        M::size_hint(&**self)
    }
}

#[cfg(feature = "std")]
impl<K: Valuable, V: Valuable> Valuable for std::collections::HashMap<K, V> {
    fn as_value(&self) -> Value<'_> {
        Value::Mappable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        for (key, value) in self.iter() {
            visit.visit_entry(key.as_value(), value.as_value());
        }
    }
}

#[cfg(feature = "std")]
impl<K: Valuable, V: Valuable> Mappable for std::collections::HashMap<K, V> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter().size_hint()
    }
}

#[cfg(feature = "alloc")]
impl<K: Valuable, V: Valuable> Valuable for alloc::collections::BTreeMap<K, V> {
    fn as_value(&self) -> Value<'_> {
        Value::Mappable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        for (key, value) in self.iter() {
            visit.visit_entry(key.as_value(), value.as_value());
        }
    }
}

#[cfg(feature = "alloc")]
impl<K: Valuable, V: Valuable> Mappable for alloc::collections::BTreeMap<K, V> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter().size_hint()
    }
}

impl fmt::Debug for dyn Mappable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugMappable<'a, 'b> {
            fmt: fmt::DebugMap<'a, 'b>,
        }

        impl Visit for DebugMappable<'_, '_> {
            fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
                self.fmt.entry(&key, &value);
            }
        }

        let mut debug = DebugMappable {
            fmt: fmt.debug_map(),
        };
        self.visit(&mut debug);
        debug.fmt.finish()
    }
}
