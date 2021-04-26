use crate::*;
use crate::field::*;

use core::fmt;

pub trait Structable {
    fn definition(&self) -> StructDef<'_>;

    fn visit(&self, visitor: &mut dyn Visit);
}
pub struct StructDef<'a> {
    /// Type name
    pub name: &'a str,

    /// Static fields
    pub static_fields: &'static [NamedField<'static>],

    /// If not all fields are statically known, then true
    pub is_dynamic: bool,
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

impl StructDef<'_> {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn static_fields(&self) -> &'static [NamedField<'static>] {
        self.static_fields
    }
}
