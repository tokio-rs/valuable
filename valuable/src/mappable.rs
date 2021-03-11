use crate::{AsValue, Value};

use std::collections::HashMap;

pub trait Mappable /*: AsValue */ {
    fn len(&self) -> usize;

    /// Get a value by key
    fn get(&self, key: &Value<'_>) -> Option<Value<'_>>;

    /// Get an iterator to keys in the map
    fn iter(&self, f: &mut dyn FnMut(&mut dyn Iterator<Item = Item<'_>>));
}

type Item<'a> = (Value<'a>, Value<'a>);

impl<T, U> Mappable for HashMap<T, U>
where
    T: AsValue,
    U: AsValue,
{
    fn len(&self) -> usize {
        HashMap::len(self)
    }

    fn get(&self, key: &Value<'_>) -> Option<Value<'_>> {
        // How to implement??
        unimplemented!();
    }

    fn iter(&self, f: &mut dyn FnMut(&mut dyn Iterator<Item = Item<'_>>)) {
        let mut iter = HashMap::iter(self)
            .map(|(k, v)| (k.as_value(), v.as_value()));

        f(&mut iter)
    }
}