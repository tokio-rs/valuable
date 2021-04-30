use crate::field::*;
use crate::*;

use core::fmt;

pub trait Structable: Valuable {
    fn definition(&self) -> StructDef<'_>;
}
pub struct StructDef<'a> {
    /// Type name
    pub name: &'a str,

    /// Fields
    pub fields: Fields<'a>,

    /// Is this a dynamic struct?
    pub is_dynamic: bool,
}

impl fmt::Debug for dyn Structable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugStruct<'a, 'b> {
            name: &'b str,
            fmt: &'b mut fmt::Formatter<'a>,
            res: fmt::Result,
        }

        impl Visit for DebugStruct<'_, '_> {
            fn visit_named_fields(&mut self, record: &NamedValues<'_>) {
                let mut d = self.fmt.debug_struct(self.name);

                for (field, value) in record.entries() {
                    d.field(field.name(), value);
                }

                self.res = d.finish();
            }

            fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
                let mut d = self.fmt.debug_tuple(self.name);

                for value in values {
                    d.field(value);
                }

                self.res = d.finish();
            }
        }

        let def = self.definition();
        let mut visit = DebugStruct {
            name: def.name(),
            fmt,
            res: Ok(()),
        };

        self.visit(&mut visit);
        visit.res
    }
}

impl StructDef<'_> {
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
