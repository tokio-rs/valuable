use crate::*;
use crate::field::*;

/// Access values for a struct's static fields
pub struct NamedValues<'a> {
    fields: &'a [NamedField<'a>],
    values: &'a [Value<'a>],
}

impl<'a> NamedValues<'a> {
    pub fn new(fields: &'a [NamedField<'a>], values: &'a [Value<'a>]) -> NamedValues<'a> {
        NamedValues { fields, values }
    }

    pub fn get(&self, field: &NamedField<'_>) -> Option<&Value<'_>> {
        use std::mem;

        let idx = (field as *const _ as usize - &self.fields[0] as *const _ as usize) / mem::size_of::<NamedField>();
        self.values.get(idx)
    }

    pub fn entries<'b>(&'b self) -> impl Iterator<Item = (&'b NamedField, &'b Value<'a>)> + 'b {
        self.fields
            .iter()
            .enumerate()
            .map(move |(i, field)| (field, &self.values[i]))
    }
}
