use crate::*;

pub trait Visit {
    /// Visit a single value
    fn visit_value(&mut self, value: Value<'_>) {
        drop(value);
    }

    /// Visits a struct's named fields
    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        drop(named_values);
    }

    /// Visits a struct's unnamed fields (tuple struct).
    fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
        drop(values);
    }

    /// Visit a slice
    fn visit_primitive_slice(&mut self, slice: Slice<'_>) {
        for value in slice {
            self.visit_item(value);
        }
    }

    fn visit_item(&mut self, value: Value<'_>) {
        drop(value);
    }

    // TODO: should we batch visit entries?
    fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
        drop((key, value));
    }
}
