use crate::field::*;
use crate::*;

#[cfg(feature = "alloc")]
use alloc::format;
use core::fmt;

pub trait Enumerable: Valuable {
    fn definition(&self) -> EnumDef<'_>;

    fn variant(&self) -> Variant<'_>;
}

pub struct EnumDef<'a> {
    /// Enum type name
    name: &'a str,

    /// Known variants
    variants: &'a [VariantDef<'a>],

    /// `true` when not all variants are statically known
    is_dynamic: bool,
}

pub struct VariantDef<'a> {
    /// Variant name
    name: &'a str,

    fields: Fields<'a>,

    /// Are all fields statically known?
    is_dynamic: bool,
}

pub enum Variant<'a> {
    Static(&'static VariantDef<'static>),
    Dynamic(DynamicVariant<'a>),
}

pub struct DynamicVariant<'a> {
    name: &'a str,
    is_named_fields: bool,
}

impl<'a> EnumDef<'a> {
    pub const fn new(
        name: &'a str,
        variants: &'a [VariantDef<'a>],
        is_dynamic: bool,
    ) -> EnumDef<'a> {
        EnumDef {
            name,
            variants,
            is_dynamic,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn variants(&self) -> &[VariantDef<'_>] {
        self.variants
    }

    pub fn is_dynamic(&self) -> bool {
        self.is_dynamic
    }
}

impl<'a> VariantDef<'a> {
    pub const fn new(name: &'a str, fields: Fields<'a>, is_dynamic: bool) -> VariantDef<'a> {
        VariantDef {
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

impl Variant<'_> {
    pub fn name(&self) -> &str {
        match self {
            Variant::Static(v) => v.name(),
            Variant::Dynamic(v) => v.name(),
        }
    }

    pub fn is_named_fields(&self) -> bool {
        match self {
            Variant::Static(v) => v.fields().is_named(),
            Variant::Dynamic(v) => v.is_named_fields(),
        }
    }

    pub fn is_unnamed_fields(&self) -> bool {
        !self.is_named_fields()
    }
}

impl<'a> DynamicVariant<'a> {
    pub const fn new(name: &'a str, is_named_fields: bool) -> DynamicVariant<'a> {
        DynamicVariant {
            name,
            is_named_fields,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn is_named_fields(&self) -> bool {
        self.is_named_fields
    }

    pub fn is_unnamed_fields(&self) -> bool {
        !self.is_named_fields
    }
}

impl fmt::Debug for dyn Enumerable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let def = self.definition();
        let variant = self.variant();
        let name = format!("{}::{}", def.name(), variant.name());

        if variant.is_named_fields() {
            struct DebugEnum<'a, 'b> {
                fmt: fmt::DebugStruct<'a, 'b>,
            }

            let mut debug = DebugEnum {
                fmt: fmt.debug_struct(&name),
            };

            impl Visit for DebugEnum<'_, '_> {
                fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
                    for (field, value) in named_values.entries() {
                        self.fmt.field(field.name(), value);
                    }
                }
            }

            self.visit(&mut debug);

            debug.fmt.finish()
        } else {
            struct DebugEnum<'a, 'b> {
                fmt: fmt::DebugTuple<'a, 'b>,
            }

            let mut debug = DebugEnum {
                fmt: fmt.debug_tuple(&name),
            };

            impl Visit for DebugEnum<'_, '_> {
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

impl<E: ?Sized + Enumerable> Enumerable for &E {
    fn definition(&self) -> EnumDef<'_> {
        E::definition(*self)
    }

    fn variant(&self) -> Variant<'_> {
        E::variant(*self)
    }
}

#[cfg(feature = "alloc")]
impl<E: ?Sized + Enumerable> Enumerable for alloc::boxed::Box<E> {
    fn definition(&self) -> EnumDef<'_> {
        E::definition(&**self)
    }

    fn variant(&self) -> Variant<'_> {
        E::variant(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<E: ?Sized + Enumerable> Enumerable for alloc::rc::Rc<E> {
    fn definition(&self) -> EnumDef<'_> {
        E::definition(&**self)
    }

    fn variant(&self) -> Variant<'_> {
        E::variant(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<E: ?Sized + Enumerable> Enumerable for alloc::sync::Arc<E> {
    fn definition(&self) -> EnumDef<'_> {
        E::definition(&**self)
    }

    fn variant(&self) -> Variant<'_> {
        E::variant(&**self)
    }
}
