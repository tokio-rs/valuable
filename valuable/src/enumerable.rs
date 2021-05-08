use crate::field::*;
use crate::*;

use core::fmt;

pub trait Enumerable: Valuable {
    fn definition(&self) -> EnumDef<'_>;
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

pub struct Variant<'a> {
    pub name: &'a str,
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

impl<'a> Variant<'a> {
    pub const fn new(name: &'a str) -> Variant<'a> {
        Variant { name }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

impl fmt::Debug for dyn Enumerable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugEnumerable<'a, 'b> {
            name: &'b str,
            fmt: &'b mut fmt::Formatter<'a>,
            res: fmt::Result,
        }

        impl Visit for DebugEnumerable<'_, '_> {
            fn visit_variant_named_fields(
                &mut self,
                variant: &Variant<'_>,
                named_values: &NamedValues<'_>,
            ) {
                let name = format!("{}::{}", self.name, variant.name());
                let mut d = self.fmt.debug_struct(&name);

                for (field, value) in named_values.entries() {
                    d.field(field.name(), value);
                }

                self.res = d.finish();
            }

            fn visit_variant_unnamed_fields(
                &mut self,
                variant: &Variant<'_>,
                values: &[Value<'_>],
            ) {
                let name = format!("{}::{}", self.name, variant.name());
                let mut d = self.fmt.debug_tuple(&name);

                for value in values {
                    d.field(value);
                }

                self.res = d.finish();
            }
        }

        let def = self.definition();
        let mut visit = DebugEnumerable {
            name: def.name(),
            fmt,
            res: Ok(()),
        };

        self.visit(&mut visit);
        visit.res
    }
}
