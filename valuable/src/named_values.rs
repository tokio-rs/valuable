use core::iter::{self, FusedIterator};

use crate::field::*;
use crate::*;

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
        use core::mem;

        let idx = (field as *const _ as usize - &self.fields[0] as *const _ as usize)
            / mem::size_of::<NamedField>();
        self.values.get(idx)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Value<'_>> {
        for (index, field) in self.fields.iter().enumerate() {
            if field.name() == name {
                return Some(&self.values[index]);
            }
        }

        None
    }

    pub fn entries<'b>(&'b self) -> Entries<'a, 'b> {
        Entries {
            iter: self.fields.iter().enumerate(),
            values: self.values,
        }
    }
}

pub struct Entries<'a, 'b> {
    iter: iter::Enumerate<core::slice::Iter<'b, NamedField<'a>>>,
    values: &'a [Value<'a>],
}

impl<'a, 'b> Iterator for Entries<'a, 'b> {
    type Item = (&'b NamedField<'a>, &'b Value<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(move |(i, field)| (field, &self.values[i]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl DoubleEndedIterator for Entries<'_, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(move |(i, field)| (field, &self.values[i]))
    }
}

impl ExactSizeIterator for Entries<'_, '_> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl FusedIterator for Entries<'_, '_> {}
