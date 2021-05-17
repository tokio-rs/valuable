use crate::field::*;
use crate::*;

use core::fmt;

pub trait Structable: Valuable {
    fn definition(&self) -> StructDef<'_>;
}

#[non_exhaustive]
pub enum StructDef<'a> {
    #[non_exhaustive]
    Static {
        name: &'static str,
        fields: Fields<'static>,
    },

    #[non_exhaustive]
    Dynamic { name: &'a str, fields: Fields<'a> },
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
    pub const fn new_static(name: &'static str, fields: Fields<'static>) -> StructDef<'a> {
        StructDef::Static { name, fields }
    }

    pub const fn new_dynamic(name: &'a str, fields: Fields<'a>) -> StructDef<'a> {
        StructDef::Dynamic { name, fields }
    }

    pub fn name(&self) -> &str {
        match self {
            StructDef::Static { name, .. } => name,
            StructDef::Dynamic { name, .. } => name,
        }
    }

    pub fn fields(&self) -> &Fields<'_> {
        match self {
            StructDef::Static { fields, .. } => fields,
            StructDef::Dynamic { fields, .. } => fields,
        }
    }

    pub fn is_static(&self) -> bool {
        matches!(self, StructDef::Static { .. })
    }

    pub fn is_dynamic(&self) -> bool {
        matches!(self, StructDef::Dynamic { .. })
    }
}

impl<S: ?Sized + Structable> Structable for &S {
    fn definition(&self) -> StructDef<'_> {
        S::definition(*self)
    }
}

impl<S: ?Sized + Structable> Structable for &mut S {
    fn definition(&self) -> StructDef<'_> {
        S::definition(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<S: ?Sized + Structable> Structable for alloc::boxed::Box<S> {
    fn definition(&self) -> StructDef<'_> {
        S::definition(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<S: ?Sized + Structable> Structable for alloc::rc::Rc<S> {
    fn definition(&self) -> StructDef<'_> {
        S::definition(&**self)
    }
}

#[cfg(not(valuable_no_atomic_cas))]
#[cfg(feature = "alloc")]
impl<S: ?Sized + Structable> Structable for alloc::sync::Arc<S> {
    fn definition(&self) -> StructDef<'_> {
        S::definition(&**self)
    }
}
