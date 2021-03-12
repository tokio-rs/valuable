use crate::{Fields, Field, Value};

use std::fmt;

pub trait Structable {
    fn fields(&self) -> Fields;

    // fn field_by_name(&self, name: &str) -> Option<Field>;

    fn get(&self, field: &Field) -> Option<Value<'_>>;
}

pub(crate) fn debug(value: &dyn Structable, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut f = fmt.debug_struct("");

    for field in value.fields().iter() {
        f.field(field.name(), &value.get(&field).unwrap());
    }

    f.finish()
}
