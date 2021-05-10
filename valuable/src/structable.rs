use crate::field::*;
use crate::*;

use core::fmt;

pub trait Structable: Valuable {
    fn definition(&self) -> StructDef<'_>;
}
pub struct StructDef<'a> {
    /// Type name
    name: &'a str,

    /// Fields
    fields: Fields<'a>,

    /// Is this a dynamic struct?
    is_dynamic: bool,
}

impl fmt::Debug for dyn Structable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let def = self.definition();

        if def.fields().is_named() {
            struct DebugStruct<'a, 'b> {
                fmt: fmt::DebugStruct<'a, 'b>,
            }

            let mut debug = DebugStruct {
                fmt: fmt.debug_struct(def.name()),
            };

            impl Visit for DebugStruct<'_, '_> {
                fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
                    for (field, value) in named_values.entries() {
                        self.fmt.field(field.name(), value);
                    }
                }
            }

            self.visit(&mut debug);

            debug.fmt.finish()
        } else {
            struct DebugStruct<'a, 'b> {
                fmt: fmt::DebugTuple<'a, 'b>,
            }

            let mut debug = DebugStruct {
                fmt: fmt.debug_tuple(def.name()),
            };

            impl Visit for DebugStruct<'_, '_> {
                fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
                    for value in values {
                        self.fmt.field(value);
                    }
                }
            }

            self.visit(&mut debug);

            debug.fmt.finish()
        }
    }
}

impl<'a> StructDef<'a> {
    pub fn new(name: &'a str, fields: Fields<'a>, is_dynamic: bool) -> StructDef<'a> {
        StructDef {
            name,
            fields,
            is_dynamic,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn fields(&self) -> &Fields<'_> {
        &self.fields
    }

    pub fn is_dynamic(&self) -> bool {
        self.is_dynamic
    }
}
