use crate::{Fields, Field, Value};

use std::fmt;

pub trait Valuable {
    fn fields(&self) -> Fields;

    // fn field_by_name(&self, name: &str) -> Option<Field>;

    fn get(&self, field: &Field) -> Option<Value<'_>>;
}

pub(crate) fn debug(valuable: &dyn Valuable, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut f = fmt.debug_struct("");

    for field in valuable.fields().iter() {
        f.field(field.name(), &valuable.get(&field).unwrap());
    }

    f.finish()
}
