use crate::*;

use std::collections::HashMap;

pub trait Mappable: Valuable {
    fn size_hint(&self) -> (usize, Option<usize>);
}

impl<K: Valuable, V: Valuable> Valuable for HashMap<K, V> {
    fn as_value(&self) -> Value<'_> {
        Value::Mappable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        for (key, value) in self.iter() {
            visit.visit_entry(key.as_value(), value.as_value());
        }
    }
}

impl<K: Valuable, V: Valuable> Mappable for HashMap<K, V> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter().size_hint()
    }
}
