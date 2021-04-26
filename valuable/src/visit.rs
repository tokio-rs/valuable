use crate::*;

pub trait Visit {
    /// Visits a struct's named fields
    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        drop(named_values);
    }

    /// Visits a struct's unnamed fields (tuple struct).
    fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
        drop(values);
    }

    /// Visit a list item
    fn visit_item(&mut self, value: Value<'_>) {
        drop(value);
    }

    /// Visit multiple list items at a time
    fn visit_items(&mut self, values: &[Value<'_>]) {
        visit_items(self, values);
    }
}

fn visit_items<T: Visit + ?Sized>(v: &mut T, values: &[Value<'_>]) {
    for value in values {
        v.visit_item(value.as_value());
    }
}