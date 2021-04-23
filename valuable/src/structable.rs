use crate::{Definition, Field, Record, Value, Visit};

use core::fmt;

pub trait Structable {
    fn definition(&self) -> Definition<'_>;

    fn visit(&self, visitor: &mut dyn Visit);
}

pub(crate) fn debug(value: &dyn Structable, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    struct Debug<'a, 'b>(fmt::DebugStruct<'a, 'b>);

    impl Visit for Debug<'_, '_> {
        fn visit_struct(&mut self, record: &Record<'_>) {
            for (field, value) in record.entries() {
                self.0.field(field.name(), value);
            }
        }
    }

    let def = value.definition();
    let mut debug = Debug(fmt.debug_struct(def.name()));

    value.visit(&mut debug);

    debug.0.finish()
}
