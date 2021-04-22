
use crate::{Valuable, Value};

use std::{collections::HashMap};

pub trait Mappable {
    fn len(&self) -> usize;

    fn get(&self, key: &Value<'_>) -> Option<Value<'_>>;

    fn iter(&self, f: &mut dyn FnMut(&mut dyn Iterator<Item = (Value<'_>, Value<'_>)>));
}

impl<K: Valuable, V: Valuable> Mappable for HashMap<K, V> {
    fn len(&self) -> usize {
        HashMap::len(self)
    }

    fn get(&self, key: &Value<'_>) -> Option<Value<'_>> {
        HashMap::iter(self)
            .find(|(k, _)| k.as_value() == *key)
            .map(|(_, v)| v.as_value())
    }

    fn iter(&self, f: &mut dyn FnMut(&mut dyn Iterator<Item = (Value<'_>, Value<'_>)>)) {
        f(&mut HashMap::iter(self).map(|(k, v)| (k.as_value(), v.as_value())));
    }
}
