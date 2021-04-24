use crate::*;
use crate::field::*;

use core::fmt;

pub trait Structable {
    fn definition(&self) -> Definition<'_>;

    fn visit(&self, visitor: &mut dyn Visit);
}

impl fmt::Debug for dyn Structable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Debug<'a, 'b>(fmt::DebugStruct<'a, 'b>);

        impl Visit for Debug<'_, '_> {
            fn visit_named_fields(&mut self, record: &NamedValues<'_>) {
                for (field, value) in record.entries() {
                    self.0.field(field.name(), value);
                }
            }
        }
    
        let def = self.definition();
        let mut debug = Debug(fmt.debug_struct(def.name()));
    
        self.visit(&mut debug);
    
        debug.0.finish()
    }
}
