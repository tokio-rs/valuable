use crate::*;

pub trait Visit {
    /// Visits a struct's static components
    fn visit_struct(&mut self, record: &Record<'_>) {
        unimplemented!();
    }

    /// Visit a list item
    fn visit_item(&mut self, value: Value<'_>) {
        unimplemented!();
    }
}
