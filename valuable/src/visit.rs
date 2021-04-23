use crate::*;

pub trait Visit {
    /// Visits a struct's static components
    fn visit_struct(&mut self, record: &Record<'_>) {
        drop(record);
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